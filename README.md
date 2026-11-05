# zapret

DPI bypass suite for Windows and Linux.  
Creator: **@jungle_dev**

---

### Project Structure

```text
├── app/                  # Modular Rust GUI (eframe / egui)
│   ├── src/
│   │   ├── main.rs       # Entrypoint & window lifecycle
│   │   ├── config.rs     # JSON settings persistence
│   │   ├── engine.rs     # Process orchestration & argument generation
│   │   └── ui.rs         # Compact iOS-style widgets & grouped rows
│   └── tests/
│       └── engine_tests.rs # Unit tests (port mappings, args, serialization)
├── bin/                  # Windows WinDivert driver & winws binaries
├── linux/                # Linux NFQUEUE suite (nftables/iptables + nfqws)
│   ├── bin/              # nfqws binaries (x86_64, aarch64)
│   ├── start.sh          # Auto-detection, iptables rules & launch
│   ├── stop.sh           # Rule flushing & process termination
│   └── zapret.service    # Systemd daemon unit
├── lists/                # Target domain lists & Telegram MTProto IP subnets
├── zapret.exe            # Production Windows GUI executable
├── 1_START_ZAPRET.bat    # Windows console fallback
└── 2_STOP_ZAPRET.bat     # Windows process & driver cleanup
```

### Features

- **Discord**: Text gateway, Voice RTC UDP desync (`19294-19344`, `50000-50100`), media cdn.
- **Telegram**: Web & MTProto DC1-DC5 (`91.108.0.0/16`, `149.154.0.0/16`).
- **YouTube**: 4K/FullHD stream unthrottle (`googlevideo.com`, QUIC/TLS desync).
- **YouTube AdBlock**: Built-in network-level RST drop for Google/YouTube video ad-servers (`list-adblock.txt`).

---

### Windows (Quick Start)

Run **`zapret.exe`** as Administrator:
- Minimalist dark grouped list.
- iOS-style compact switches (36x20 px).
- Settings auto-save to `settings.json`.

---

### Linux (Quick Start)

```bash
cd linux
chmod +x start.sh stop.sh
sudo ./start.sh
```

To run as a systemd service:
```bash
sudo cp zapret.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now zapret
```
