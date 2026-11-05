#!/bin/bash
echo "[*] Stopping zapret..."

pkill -9 nfqws 2>/dev/null || true
pkill -9 tpws 2>/dev/null || true

TCP_PORTS="80,443,2053,2083,2087,2096,8443"
UDP_PORTS="443,19294:19344,50000:50100"
QNUM=200

if command -v iptables >/dev/null 2>&1; then
    while iptables -w 5 -t mangle -D POSTROUTING -p tcp -m multiport --dports "$TCP_PORTS" -j NFQUEUE --queue-num "$QNUM" --queue-bypass 2>/dev/null; do :; done
    while iptables -w 5 -t mangle -D POSTROUTING -p udp -m multiport --dports "$UDP_PORTS" -j NFQUEUE --queue-num "$QNUM" --queue-bypass 2>/dev/null; do :; done
fi

if command -v ip6tables >/dev/null 2>&1; then
    while ip6tables -w 5 -t mangle -D POSTROUTING -p tcp -m multiport --dports "$TCP_PORTS" -j NFQUEUE --queue-num "$QNUM" --queue-bypass 2>/dev/null; do :; done
    while ip6tables -w 5 -t mangle -D POSTROUTING -p udp -m multiport --dports "$UDP_PORTS" -j NFQUEUE --queue-num "$QNUM" --queue-bypass 2>/dev/null; do :; done
fi

echo "[+] Zapret stopped and iptables cleaned up."
