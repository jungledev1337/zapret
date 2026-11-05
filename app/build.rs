fn main() {
    #[cfg(target_os = "windows")]
    {
        // Tell MSVC linker to embed UAC requireAdministrator manifest
        println!("cargo:rustc-link-arg=/MANIFESTUAC:level='requireAdministrator'");
    }
}
