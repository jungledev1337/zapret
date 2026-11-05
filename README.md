# zapret — High-Performance DPI Bypass Suite (Windows & Linux)

[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-blue?style=flat-square)](https://github.com/jungledev1337/zapret)
[![Rust](https://img.shields.io/badge/GUI-Rust%20%28eframe%2Fegui%29-orange?style=flat-square)](https://github.com/jungledev1337/zapret)
[![Engine](https://img.shields.io/badge/engine-WinDivert%20%7C%20NFQUEUE-green?style=flat-square)](https://github.com/jungledev1337/zapret)
[![License: MIT](https://img.shields.io/badge/license-MIT-purple?style=flat-square)](LICENSE)
[![Creator](https://img.shields.io/badge/developer-%40jungle__dev-black?style=flat-square)](https://t.me/jungle_dev)

> **Autonomous Deep Packet Inspection (DPI) circumvention suite with zero VPN overhead.**  
> Native frosted liquid-glass GUI for Windows 10/11 + high-throughput NFQUEUE engine for Linux (Ubuntu, Debian, Arch, Fedora, ARM64/Raspberry Pi).

---

## ⚡ Key Highlights

- **Discord Unblock**: Fixes infinite "Connecting..." and "RTC Connecting" on voice channels, WebRTC media streams (`19294-19344`, `50000-50100/UDP`), gateway, and file servers (`discord.media`).
- **Telegram Acceleration**: Instant media download and seamless MTProto DC1–DC5 proxy bypass (`91.108.0.0/16`, `149.154.0.0/16`) without third-party VPN latency.
- **YouTube 4K & AdBlock**: Unthrottled 4K 60FPS video playback (fixes buffer stutters on `googlevideo.com`) paired with an **in-engine TCP RST ad-blocker** that drops Google ad-networks instantly without browser extensions.
- **Dual Platform (Windows & Linux)**:
  - **Windows**: Modern, lightweight (~4MB) native Rust GUI with Apple VisionOS-inspired acrylic blur, compact toggle switches, draggable title bar, and auto-elevating UAC support.
  - **Linux**: High-performance `nfqws` daemon supporting systemd, auto architecture detection (`x86_64` & `aarch64`/`arm64`), automated `iptables`/`nftables` packet queues, and clean SIGINT trap handlers.
- **Zero Configuration Required**: Download, run, and everything works out of the box.

---

## 🖥️ Windows Quick Start

### 1. Download & Launch GUI
1. Clone or download the repository:
   ```cmd
   git clone https://github.com/jungledev1337/zapret.git
   cd zapret
   ```
2. Run **`zapret.exe`**.  
   *The application will automatically request Administrator privileges via UAC to interact with the high-speed WinDivert network filter driver.*
3. Toggle Discord, Telegram, or YouTube on/off in real-time. Preferences are saved automatically to `settings.json`.

### Console Alternative (Batch)
If you prefer running without GUI:
- Start: Right-click **`1_START_ZAPRET.bat`** → Run as Administrator.
- Stop: Run **`2_STOP_ZAPRET.bat`**.

---

## 🐧 Linux Quick Start (Ubuntu / Debian / Arch / Fedora / Alpine)

Zapret includes precompiled ELF binaries for both `x86_64` (AMD64) and `aarch64` (ARM64).

### Standalone Runner
```bash
git clone https://github.com/jungledev1337/zapret.git
cd zapret/linux
chmod +x start.sh stop.sh

# Start the bypass daemon
sudo ./start.sh

# Stop and flush iptables NFQUEUE rules cleanly
sudo ./stop.sh
```

### Run as a Background Systemd Service
To ensure Zapret starts automatically on system boot:
```bash
cd zapret/linux
sudo cp zapret.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now zapret
```
Check service status:
```bash
sudo systemctl status zapret
```

---

## 🔬 How It Works (Packet Desynchronization)

Unlike VPNs that encrypt and route all traffic through remote servers (increasing ping and causing captchas), **zapret** runs locally on your network interface.

```text
               +-------------------------------------------+
               |              Outbound Traffic             |
               +---------------------+---------------------+
                                     |
               +---------------------v---------------------+
               |   Target Matches (Discord/TG/YouTube)?    |
               +----------+-----------------------+--------+
                          |                       |
                         YES                      NO
                          |                       |
               +----------v----------+    +-------v-------+
               |  DPI Packet Desync  |    | Direct Pass   |
               |  - Fake QUIC / UDP  |    | (No Overhead) |
               |  - 2-Byte Split     |    +---------------+
               |  - Seq Overlap      |
               |  - Instant Ad-RST   |
               +----------+----------+
                          |
               +----------v--------------------------------+
               | TSP / ISP DPI Sensor Fooled & Bypassed   |
               +-------------------------------------------+
```

1. **TCP Multisplit & Sequence Overlap**: Splits TLS ClientHello packets at the first byte (`--dpi-desync-split-pos=1`) with fake SNI prefixes (`--dpi-desync-split-seqovl`). The DPI filter fails to assemble the SNI, while target servers seamlessly reconstruct the stream.
2. **Discord Voice Fake UDP**: Discord voice channels rely on proprietary UDP packets. Zapret injects active fake packet payloads (`ACTIVE_DISCORD_UDP.bin`) with repeated desync passes, breaking DPI throttling without disrupting WebRTC audio.
3. **In-Engine Ad-Blocking**: Outbound handshakes to known telemetry and advertising endpoints (`list-adblock.txt`) receive a synthetic TCP RST, causing YouTube's player to skip ads in zero milliseconds.

---

## 📂 Repository Structure

```text
├── app/                  # Pure Rust GUI source code
│   ├── src/
│   │   ├── main.rs       # Window lifecycle, acrylic backdrop & tray
│   │   ├── lib.rs        # Modular library crate (zapret_lib)
│   │   ├── engine.rs     # Process orchestration & WinDivert/NFQWS rule builder
│   │   ├── ui.rs         # Apple-style toggles & macOS traffic light controls
│   │   └── config.rs     # JSON settings persistence
│   ├── build.rs          # MSVC UAC elevation manifest linker
│   └── tests/            # Automated unit tests (ports, args, serialization)
├── bin/                  # Windows x64 WinDivert driver & winws binaries
├── linux/                # Linux NFQUEUE runner suite
│   ├── bin/              # Precompiled binaries (x86_64 & arm64)
│   ├── start.sh          # Auto-architecture detection & iptables queue binding
│   ├── stop.sh           # Clean queue flushing & process termination
│   └── zapret.service    # Systemd daemon configuration
├── lists/                # Target domain lists & Telegram IP subnets
│   ├── list-general.txt  # Discord, cloud services, and global blocks
│   ├── list-google.txt   # Google & YouTube video playback CDN servers
│   ├── list-adblock.txt  # YouTube ad server endpoints for instant RST
│   └── ipset-all.txt     # Telegram DC1-DC5 CIDR subnets
├── zapret.exe            # Ready-to-run Windows 64-bit executable
├── 1_START_ZAPRET.bat    # Windows CMD launcher (auto-elevating fallback)
├── 2_STOP_ZAPRET.bat     # Windows stop script
└── LICENSE               # MIT License
```

---

## 🛠️ Building from Source

### Prerequisites
- [Rust](https://rustup.rs/) (edition 2021+)
- Windows 10/11 or modern Linux distribution

```bash
cd app
cargo build --release
```
The compiled binary will be located at `app/target/release/zapret` (`.exe` on Windows).

Run test suite:
```bash
cargo test
```

---

## 👤 Author

Developed and maintained by **[@jungle_dev](https://t.me/jungle_dev)**.  
GitHub: **[jungledev1337](https://github.com/jungledev1337)**

---

## ⚖️ License

Distributed under the [MIT License](LICENSE).
