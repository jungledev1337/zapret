use std::path::{Path, PathBuf};
use std::process::{Child, Command};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Default)]
pub struct EngineManager {
    process: Option<Child>,
}

impl EngineManager {
    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
        }

        #[cfg(windows)]
        {
            let _ = Command::new("taskkill")
                .args(["/f", "/im", "winws.exe"])
                .creation_flags(CREATE_NO_WINDOW)
                .status();
        }

        #[cfg(not(windows))]
        {
            let _ = Command::new("pkill").args(["-f", "nfqws"]).status();
        }
    }

    pub fn start(&mut self, base_dir: &Path, discord: bool, telegram: bool, youtube: bool) -> Result<(), String> {
        self.stop();

        if !discord && !telegram && !youtube {
            return Ok(());
        }

        let bin_dir = base_dir.join("bin");
        let lists_dir = base_dir.join("lists");
        let winws_exe = bin_dir.join("winws.exe");

        if !winws_exe.exists() {
            return Err("winws.exe not found".into());
        }

        let args = build_engine_args(&bin_dir, &lists_dir, discord, telegram, youtube);

        let mut cmd = Command::new(&winws_exe);
        cmd.args(&args).current_dir(base_dir);

        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW);

        match cmd.spawn() {
            Ok(child) => {
                self.process = Some(child);
                Ok(())
            }
            Err(e) => Err(format!("Spawn failed: {}", e)),
        }
    }
}

pub fn build_engine_args(
    bin_dir: &Path,
    lists_dir: &Path,
    discord: bool,
    telegram: bool,
    youtube: bool,
) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let (tcp_ports, udp_ports) = compute_ports(discord, telegram);

    args.push(format!("--wf-tcp={}", tcp_ports.join(",")));
    args.push(format!("--wf-udp={}", udp_ports.join(",")));

    let fake_quic = path_str(&bin_dir.join("quic_initial_www_google_com.bin"));
    let fake_discord = path_str(&bin_dir.join("ACTIVE_DISCORD_UDP.bin"));
    let tls_google = path_str(&bin_dir.join("tls_clienthello_www_google_com.bin"));
    let tls_general = path_str(&bin_dir.join("tls_clienthello_4pda_to.bin"));

    let list_gen = path_str(&lists_dir.join("list-general.txt"));
    let list_user = path_str(&lists_dir.join("list-general-user.txt"));
    let list_goog = path_str(&lists_dir.join("list-google.txt"));
    let list_excl = path_str(&lists_dir.join("list-exclude.txt"));
    let list_ad = path_str(&lists_dir.join("list-adblock.txt"));
    let ipset_all = path_str(&lists_dir.join("ipset-all.txt"));
    let ipset_excl = path_str(&lists_dir.join("ipset-exclude.txt"));

    if youtube {
        args.extend([
            // YouTube ad-blocking via instant RST to ad networks
            "--filter-tcp=443".into(),
            format!("--hostlist={}", list_ad),
            "--dpi-desync=rst".into(),
            "--new".into(),

            // YouTube 4K stream delivery bypass
            "--filter-udp=443".into(),
            format!("--hostlist={}", list_gen),
            format!("--hostlist={}", list_user),
            format!("--hostlist-exclude={}", list_excl),
            format!("--ipset-exclude={}", ipset_excl),
            "--dpi-desync=fake".into(),
            "--dpi-desync-repeats=6".into(),
            format!("--dpi-desync-fake-quic={}", fake_quic),
            "--new".into(),
            "--filter-tcp=443".into(),
            format!("--hostlist={}", list_goog),
            "--ip-id=zero".into(),
            "--dpi-desync=multisplit".into(),
            "--dpi-desync-split-seqovl=681".into(),
            "--dpi-desync-split-pos=1".into(),
            format!("--dpi-desync-split-seqovl-pattern={}", tls_google),
            "--new".into(),
        ]);
    }

    if discord {
        args.extend([
            "--filter-udp=19294-19344,50000-50100".into(),
            "--filter-l7=discord,stun".into(),
            "--dpi-desync=fake".into(),
            format!("--dpi-desync-fake-discord={}", fake_discord),
            format!("--dpi-desync-fake-stun={}", fake_discord),
            "--dpi-desync-repeats=6".into(),
            "--new".into(),
            "--filter-tcp=2053,2083,2087,2096,8443".into(),
            "--hostlist-domains=discord.media".into(),
            "--dpi-desync=multisplit".into(),
            "--dpi-desync-split-seqovl=681".into(),
            "--dpi-desync-split-pos=1".into(),
            format!("--dpi-desync-split-seqovl-pattern={}", tls_google),
            "--new".into(),
        ]);
    }

    if discord || telegram {
        args.extend([
            "--filter-tcp=80,443".into(),
            format!("--hostlist={}", list_gen),
            format!("--hostlist={}", list_user),
            format!("--hostlist-exclude={}", list_excl),
            format!("--ipset-exclude={}", ipset_excl),
            "--dpi-desync=multisplit".into(),
            "--dpi-desync-split-seqovl=568".into(),
            "--dpi-desync-split-pos=1".into(),
            format!("--dpi-desync-split-seqovl-pattern={}", tls_general),
            "--new".into(),
        ]);
    }

    if telegram {
        args.extend([
            "--filter-tcp=80,443,8443".into(),
            format!("--ipset={}", ipset_all),
            format!("--hostlist-exclude={}", list_excl),
            format!("--ipset-exclude={}", ipset_excl),
            "--dpi-desync=multisplit".into(),
            "--dpi-desync-split-seqovl=568".into(),
            "--dpi-desync-split-pos=1".into(),
            format!("--dpi-desync-split-seqovl-pattern={}", tls_general),
        ]);
    }

    args
}

pub fn compute_ports(discord: bool, telegram: bool) -> (Vec<&'static str>, Vec<&'static str>) {
    let mut tcp = vec!["80", "443"];
    let mut udp = vec!["443"];

    if discord {
        tcp.extend(["2053", "2083", "2087", "2096", "8443"]);
        udp.extend(["19294-19344", "50000-50100"]);
    }

    if telegram && !tcp.contains(&"8443") {
        tcp.push("8443");
    }

    (tcp, udp)
}

fn path_str(p: &PathBuf) -> String {
    p.to_string_lossy().to_string()
}
