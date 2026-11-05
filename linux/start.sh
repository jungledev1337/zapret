#!/bin/bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$DIR")"
LISTS_DIR="$ROOT_DIR/lists"
QNUM=200

ARCH="$(uname -m)"
case "$ARCH" in
    x86_64|amd64) BIN="$DIR/bin/x86_64/nfqws" ;;
    aarch64|arm64) BIN="$DIR/bin/arm64/nfqws" ;;
    *) echo "[-] Unsupported arch: $ARCH"; exit 1 ;;
esac

if [ ! -f "$BIN" ]; then
    echo "[-] Binary not found: $BIN"
    exit 1
fi
chmod +x "$BIN"

if [ "$EUID" -ne 0 ]; then
    echo "[-] Must be run as root (use sudo)"
    exit 1
fi

modprobe nfnetlink_queue 2>/dev/null || true
modprobe xt_NFQUEUE 2>/dev/null || true

if [ "$1" == "--test" ] || [ "$1" == "test" ]; then
    echo "[+] Arch: $ARCH"
    echo "[+] Binary: $BIN"
    echo "[+] Lists: $LISTS_DIR"
    "$BIN" --help >/dev/null 2>&1 && echo "[+] nfqws binary check: OK" || echo "[-] nfqws binary check failed"
    exit 0
fi

cleanup() {
    echo ""
    echo "[*] Cleaning up rules..."
    "$DIR/stop.sh" >/dev/null 2>&1 || true
    exit 0
}
trap cleanup SIGINT SIGTERM

"$DIR/stop.sh" >/dev/null 2>&1 || true

echo "[*] Adding NFQUEUE rules (queue $QNUM)..."
TCP_PORTS="80,443,2053,2083,2087,2096,8443"
UDP_PORTS="443,19294:19344,50000:50100"

if command -v iptables >/dev/null 2>&1; then
    iptables -w 5 -t mangle -I POSTROUTING -p tcp -m multiport --dports "$TCP_PORTS" -j NFQUEUE --queue-num "$QNUM" --queue-bypass
    iptables -w 5 -t mangle -I POSTROUTING -p udp -m multiport --dports "$UDP_PORTS" -j NFQUEUE --queue-num "$QNUM" --queue-bypass
fi

if [ -f /proc/net/if_inet6 ] && command -v ip6tables >/dev/null 2>&1; then
    ip6tables -w 5 -t mangle -I POSTROUTING -p tcp -m multiport --dports "$TCP_PORTS" -j NFQUEUE --queue-num "$QNUM" --queue-bypass 2>/dev/null || true
    ip6tables -w 5 -t mangle -I POSTROUTING -p udp -m multiport --dports "$UDP_PORTS" -j NFQUEUE --queue-num "$QNUM" --queue-bypass 2>/dev/null || true
fi

echo "[*] nfqws running. Press Ctrl+C to stop."

"$BIN" --qnum="$QNUM" \
--filter-tcp=443 --hostlist="$LISTS_DIR/list-adblock.txt" --dpi-desync=rst --new \
--filter-udp=443 --hostlist="$LISTS_DIR/list-general.txt" --hostlist="$LISTS_DIR/list-general-user.txt" --hostlist-exclude="$LISTS_DIR/list-exclude.txt" --dpi-desync=fake --dpi-desync-repeats=6 --new \
--filter-udp=19294-19344,50000-50100 --filter-l7=discord,stun --dpi-desync=fake --dpi-desync-repeats=6 --new \
--filter-tcp=2053,2083,2087,2096,8443 --hostlist-domains=discord.media --dpi-desync=multisplit --dpi-desync-split-pos=1 --new \
--filter-tcp=443 --hostlist="$LISTS_DIR/list-google.txt" --dpi-desync=multisplit --dpi-desync-split-pos=1 --new \
--filter-tcp=80,443 --hostlist="$LISTS_DIR/list-general.txt" --hostlist="$LISTS_DIR/list-general-user.txt" --hostlist-exclude="$LISTS_DIR/list-exclude.txt" --dpi-desync=multisplit --dpi-desync-split-pos=1 --new \
--filter-tcp=80,443,8443 --ipset="$LISTS_DIR/ipset-all.txt" --hostlist-exclude="$LISTS_DIR/list-exclude.txt" --dpi-desync=multisplit --dpi-desync-split-pos=1
