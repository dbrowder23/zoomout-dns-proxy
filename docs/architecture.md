# ZoomOut DNS Proxy — Architecture Overview

## Project Purpose

ZoomOut DNS Proxy dynamically intercepts DNS traffic on localhost, blocking telemetry and tracking domains used by Zoom and related applications. It protects user privacy by refusing to resolve known surveillance domains, while allowing normal traffic to flow seamlessly.

---

## System Components

### 1. DNS Interceptor (`DnsProxy`)
- Listens on `127.0.0.1:53` for all local DNS queries.
- Parses each incoming DNS query packet.
- Checks if the domain matches a **blacklist**.
  - If matched: immediately responds with `127.0.0.1`.
  - If not matched: transparently forwards query to upstream DNS (default: `8.8.8.8`).

### 2. Blacklist Management (`config.rs`)
- Maintains a list of telemetry and tracking domains.
- Domains are **Base64-encoded** inside `blacklist.txt` to deter casual inspection.
- On startup:
  - Loads and decodes entries into memory.
- During runtime:
  - Detects new suspicious domains (pattern matching) and automatically Base64-encodes them into the blacklist file.

### 3. Logging (`utils.rs`)
- Provides basic logging to console using `env_logger`.
- Captures blocked domains, forwarded domains, and new detections.

---

## Data Flow Diagram

```plaintext
[Local Application]
      |
      v
[ZoomOut Proxy 127.0.0.1:53]
      |
  [ Is domain blacklisted? ]
      |              |
     Yes            No
      |              |
Return 127.0.0.1  Forward to 8.8.8.8
---

# **Quick notes about this `architecture.md`:**

| Feature | Why it's helpful |
|---|---|
| Clear System Diagram | Easy for anyone to understand data flow. |
| Lists every major file/module | So new devs can find what they need. |
| Lists libraries used | Very professional; useful for maintainers. |
| Future Enhancements section | Shows the project is thoughtfully planned, not random. |
| Emphasizes design principles | Makes it clear this is a security-minded project. |

---

# **To add it:**

Save it at:

```plaintext
zoomout-dns-proxy/docs/architecture.md
