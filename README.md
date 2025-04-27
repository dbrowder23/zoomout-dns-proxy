# ZoomOut DNS Proxy

**ZoomOut DNS Proxy** is a cross-platform, dynamic DNS interceptor designed to protect user privacy against intrusive telemetry, user fingerprinting, and ban tracking by Zoom and similar applications.

Built in Rust for maximum speed, security, and reliability, ZoomOut acts as a stealth DNS proxy that dynamically blocks known telemetry domains — and learns new ones during active use.

---

## Features

- **Dynamic Telemetry Protection**  
  Intercepts and blocks Zoom telemetry, metrics, and tracking servers in real-time.

- **Passive Learning Mode**  
  Auto-detects suspicious new domains and adds them to the blacklist.

- **Cross-Platform**  
  Native support for Linux (Debian/Ubuntu), Windows 11, and MacOS (including M1/M2).

- **User-Friendly Deployment**  
  Quick install script with no need to disable System Integrity Protection (SIP) on Macs.

- **Resilient Operation**  
  If a domain is unknown, ZoomOut passes it cleanly to your normal upstream DNS server (default: 8.8.8.8).

---

## Quick Install

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/dbrowder23/zoomout-dns-proxy/main/install.sh)"
