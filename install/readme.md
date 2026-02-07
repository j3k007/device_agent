cat > install/README.md << 'EOF'
# Device Agent Installation Guide

This directory contains installation scripts for different operating systems.

## macOS Installation

### Install
```bash
sudo ./macos/install.sh
```

### Uninstall
```bash
sudo ./macos/uninstall.sh
```

### Service Commands
```bash
# Start service
launchctl start com.deviceagent

# Stop service
launchctl stop com.deviceagent

# Check status
launchctl list | grep deviceagent

# View logs
tail -f /usr/local/var/device-agent/logs/agent_*.log
```

### Files Location
- Binary: `/usr/local/bin/device-agent`
- Config: `/usr/local/etc/device-agent/config.toml`
- Data: `/usr/local/var/device-agent/data/`
- Logs: `/usr/local/var/device-agent/logs/`

---

## Linux Installation

### Install
```bash
sudo ./linux/install.sh
```

### Uninstall
```bash
sudo ./linux/uninstall.sh
```

### Service Commands
```bash
# Start service
sudo systemctl start device-agent

# Stop service
sudo systemctl stop device-agent

# Restart service
sudo systemctl restart device-agent

# Check status
sudo systemctl status device-agent

# View logs
sudo journalctl -u device-agent -f
```

### Files Location
- Binary: `/usr/local/bin/device-agent`
- Config: `/opt/device-agent/config.toml`
- Data: `/opt/device-agent/data/`
- Logs: `/opt/device-agent/logs/`

---

## Windows Installation

### Install
Run PowerShell as Administrator:
```powershell
.\windows\install.ps1
```

### Uninstall
Run PowerShell as Administrator:
```powershell
.\windows\uninstall.ps1
```

### Service Commands
```powershell
# Start service
sc.exe start DeviceAgent

# Stop service
sc.exe stop DeviceAgent

# Check status
sc.exe query DeviceAgent

# View logs in Event Viewer
eventvwr.msc
```

### Files Location
- Binary: `C:\Program Files\DeviceAgent\device-agent.exe`
- Config: `C:\Program Files\DeviceAgent\config.toml`
- Data: `C:\Program Files\DeviceAgent\data\`
- Logs: `C:\Program Files\DeviceAgent\logs\`

---

## Requirements

- **All platforms**: Rust toolchain must be installed for building
- **Linux**: systemd-based system (Ubuntu, Debian, Fedora, etc.)
- **macOS**: macOS 10.12 or later
- **Windows**: Windows 10 or later

## Troubleshooting

### Service won't start
Check logs for errors:
- macOS: `tail -f /usr/local/var/device-agent/logs/*.log`
- Linux: `sudo journalctl -u device-agent -n 50`
- Windows: Check `C:\Program Files\DeviceAgent\logs\`

### Permission denied
Make sure you're running install scripts with appropriate privileges:
- macOS/Linux: Use `sudo`
- Windows: Run PowerShell as Administrator

### Service not found after install
Reload service daemon:
- macOS: `launchctl load ~/Library/LaunchAgents/com.deviceagent.plist`
- Linux: `sudo systemctl daemon-reload`
- Windows: Reboot or restart the Service Control Manager

## Support

For issues, please check:
1. Installation logs
2. Agent logs
3. System service status
4. File permissions

---

**Note**: Uninstalling will preserve data and configuration by default. 
You'll be prompted if you want to remove everything.
EOF