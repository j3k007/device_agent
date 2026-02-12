# Device Agent - System Monitoring Platform

A comprehensive cross-platform device monitoring solution with secure self-registration, device fingerprinting, and real-time system tracking.

---

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Features](#features)
- [Project Structure](#project-structure)
- [Quick Start](#quick-start)
- [Installation](#installation)
  - [Backend Setup](#backend-setup)
  - [Agent Setup](#agent-setup)
- [Authentication & Registration](#authentication--registration)
- [Usage](#usage)
- [Configuration](#configuration)
- [Admin Interface](#admin-interface)
- [Development](#development)
- [Deployment](#deployment)
- [Troubleshooting](#troubleshooting)
- [API Documentation](#api-documentation)
- [Security](#security)
- [Contributing](#contributing)

---

## 🔍 Overview

The Device Agent is a **secure, enterprise-grade monitoring solution** that collects comprehensive system information from devices and sends it to a centralized Django backend for storage, analysis, and management.

**Key Components:**
- **Rust Agent**: Cross-platform system monitoring agent with hardware fingerprinting
- **Django Backend**: REST API server with database storage and beautiful admin interface
- **Self-Registration**: Automated device onboarding with admin approval workflow
- **Security**: Device binding, theft detection, and encrypted token storage

---

## 🏗 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                Device (macOS/Linux/Windows)                     │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Rust Agent                                               │  │
│  │  ├── Hardware Fingerprint Generation (SHA256)             │  │
│  │  ├── Self-Registration Request                            │  │
│  │  ├── Collects: CPU, Memory, IPs, Services, Software       │  │
│  │  ├── Encrypted Token Storage (AES-256-GCM)                │  │
│  │  ├── Automatic Retry with Exponential Backoff             │  │
│  │  └── Sends via HTTPS with Bearer Token                    │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                            │ HTTPS + Bearer Token
                            │ + Device Fingerprint
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Django Backend Server                         │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  REST API (Django + DRF)                                  │  │
│  │  ├── Self-Registration Endpoint (No Auth)                 │  │
│  │  ├── Admin Approval System                                │  │
│  │  ├── Token Authentication + Device Validation             │  │
│  │  ├── Device Fingerprint Verification                      │  │
│  │  ├── Theft Detection & Auto-Disable                       │  │
│  │  ├── Database Storage (Device, Services, Software)        │  │
│  │  └── Beautiful Admin Interface with Visualizations        │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## ✨ Features

### 🦀 Rust Agent

#### **System Monitoring**
- ✅ **Cross-platform**: macOS, Linux, Windows
- ✅ **Hardware info**: CPU, memory (total/available/usage %), hostname
- ✅ **Network info**: IPv4 addresses with associated IPv6 addresses
- ✅ **Service tracking**: Running services/daemons (543+ services on macOS)
- ✅ **Software inventory**: Installed applications with tracking
- ✅ **OS information**: Type, version, architecture

#### **Security Features**
- ✅ **Hardware fingerprinting**: Unique device identification via UUID, MAC, serial numbers
- ✅ **Token encryption**: AES-256-GCM encrypted token storage
- ✅ **Device binding**: One token = one specific device
- ✅ **Theft detection**: Automatic detection when token used on different device
- ✅ **Secure storage**: OS-level file permissions (600 - owner only)

#### **Reliability**
- ✅ **Automatic retry**: Exponential backoff on failures
- ✅ **Graceful shutdown**: Ctrl+C handling with cleanup
- ✅ **Logging**: Rotating daily file logs + console output
- ✅ **Service installation**: systemd, launchd, Windows Service support

---

### 🐍 Django Backend

#### **API & Authentication**
- ✅ **REST API**: Django REST Framework with JSON responses
- ✅ **Token authentication**: Secure agent authentication with validation
- ✅ **Device fingerprint validation**: Verify device identity on every request
- ✅ **Self-registration API**: Automated device onboarding
- ✅ **Theft prevention**: Automatic token disabling after multiple mismatches

#### **Database Storage (Phase 3)**
- ✅ **Device tracking**: Comprehensive device information storage
- ✅ **Service monitoring**: Track 500+ running services per device
- ✅ **Software inventory**: Monitor installed applications
- ✅ **History tracking**: First seen, last seen, active/inactive status
- ✅ **IP address storage**: IPv4 → [IPv6...] mapping format

#### **Admin Interface**
- ✅ **Device management**: Beautiful dashboard with visualizations
- ✅ **Memory usage**: Color-coded progress bars (Green/Orange/Red)
- ✅ **Online status**: Real-time device status indicators
- ✅ **Service/Software lists**: Filterable, searchable tables
- ✅ **Registration approval**: One-click approve/reject workflow
- ✅ **Token management**: Create, revoke, track usage
- ✅ **Security alerts**: Fingerprint mismatch notifications

#### **Additional Features**
- ✅ **Request logging**: Detailed API logs
- ✅ **CORS support**: Cross-origin requests
- ✅ **Database support**: SQLite (dev) / PostgreSQL (prod)
- ✅ **Admin bulk actions**: Approve/reject multiple registrations

---

## 📁 Project Structure

```
device-agent/
├── backend/                      # Django REST API
│   ├── config/                   # Django settings
│   ├── agents/                   # Agent token management
│   │   ├── models.py            # AgentToken, PendingRegistration
│   │   ├── authentication.py    # Token + fingerprint authentication
│   │   ├── admin.py             # Registration approval interface
│   │   ├── views.py             # Registration API endpoints
│   │   └── urls.py              # /api/agents/register/, /status/
│   ├── devices/                  # Device data storage (Phase 3)
│   │   ├── models.py            # Device, DeviceService, DeviceSoftware
│   │   ├── serializers.py       # Data validation
│   │   ├── admin.py             # Beautiful device admin interface
│   │   └── migrations/          # Database schema
│   ├── api/                      # API endpoints
│   │   ├── views.py             # Heartbeat with DB storage
│   │   └── urls.py              # URL routing
│   ├── logs/                     # Application logs
│   ├── manage.py
│   ├── requirements.txt
│   └── .env                      # Environment variables
│
├── src/                          # Rust agent source code
│   ├── main.rs                   # CLI + main loop + registration
│   ├── models.rs                 # SystemInfo with fingerprint
│   ├── config.rs                 # Configuration
│   ├── crypto.rs                 # Token encryption
│   ├── fingerprint.rs            # Device fingerprint generation
│   ├── sender.rs                 # HTTP client
│   ├── retry.rs                  # Retry logic
│   └── collector/                # Platform-specific collectors
│       ├── mod.rs                # Collector orchestration
│       ├── common.rs             # Basic info + IPs + memory
│       ├── macos.rs              # macOS services & software
│       ├── linux.rs              # Linux services & software
│       └── windows.rs            # Windows services & software
│
├── install/                      # Installation scripts
│   ├── macos/
│   │   ├── install.sh
│   │   ├── uninstall.sh
│   │   └── com.deviceagent.plist
│   ├── linux/
│   │   ├── install.sh
│   │   ├── uninstall.sh
│   │   └── device-agent.service
│   └── windows/
│       ├── install.ps1
│       └── uninstall.ps1
│
├── Cargo.toml                    # Rust dependencies
├── config.toml                   # Agent configuration (not in git)
├── config.example.toml           # Configuration template
└── README.md                     # This file
```

---

## 🚀 Quick Start

### Prerequisites

**Backend:**
- Python 3.10+
- pip

**Agent:**
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))

---

### 1. Start Backend

```bash
# Navigate to backend
cd device-agent/backend

# Create virtual environment
python3 -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Setup database
python manage.py migrate

# Create admin user
python manage.py createsuperuser
# Username: admin
# Password: (your choice)

# Start server
python manage.py runserver
```

**Backend is now running at:** http://localhost:8000

---

### 2. Setup Agent

```bash
# Navigate to agent directory
cd device-agent

# Copy example config
cp config.example.toml config.toml

# Edit config with your agent details
nano config.toml

# Update these fields:
# agent_id = "laptop-001"
# agent_name = "John's MacBook Pro"

# Build agent
cargo build --release
```

---

### 3. Self-Register Device (New!)

```bash
# Request registration
./target/release/device-agent --init
```

**Output:**
```
=== Device Agent Initialization ===

Requesting registration for:
  Agent ID:   laptop-001
  Agent Name: John's MacBook Pro

Collecting system information...
Generating device fingerprint...
✓ Fingerprint: a1b2c3d4e5f6...

Sending registration request to:
  http://localhost:8000/api/agents/register/

✓ Registration request submitted successfully!

Registration request submitted. Waiting for admin approval.

Next steps:
1. Admin will review your request in Django admin
2. Check status: device-agent --check-status
3. Once approved, token will be saved automatically
```

---

### 4. Approve Registration (Admin)

1. Open Django Admin: http://localhost:8000/admin/
2. Login with admin credentials
3. Navigate to: **Agents → Pending Registrations**
4. You'll see your device's registration request with:
   - Agent ID: `laptop-001`
   - Hostname: `Johns-MacBook-Pro`
   - OS: `macos 14.0`
   - Fingerprint: `a1b2c3d4e5f6...`
5. Click the **"Approve"** button
6. Token is automatically generated and bound to the device

---

### 5. Check Status & Get Token

```bash
# Check if approved
./target/release/device-agent --check-status
```

**Output:**
```
=== Checking Registration Status ===

Checking status for: laptop-001

✓ Status: APPROVED

Your device has been approved!

Saving token automatically...
✓ Token saved successfully!

You can now start the agent:
  device-agent
```

---

### 6. Start Monitoring

```bash
./target/release/device-agent
```

**Output:**
```
✓ Configuration loaded from config.toml
✓ Logging to file: ./logs/agent_20250212.log
[INFO] === Device Agent Starting ===
[INFO] Agent ID: laptop-001
[INFO] Agent Name: John's MacBook Pro
[INFO] Collection Interval: 30 seconds
[INFO] Token Location: ./.token
[INFO] Press Ctrl+C to stop

[INFO] === Collection Iteration #1 ===
[INFO] Starting system information collection...
[INFO] Collecting services...
[INFO] ✓ Found 543 services
[INFO] Collecting installed software...
[INFO] ✓ Found 22 installed applications
[INFO] Collection completed in 2.34s

[INFO] Sending data to backend: http://localhost:8000/api/heartbeat/
[INFO] ✓ Data sent successfully (status: 200 OK)
[INFO] ✓ Device ID: 1
[DEBUG]   Hostname: Johns-MacBook-Pro
[DEBUG]   Services: 543
[DEBUG]   Software: 22
[DEBUG]   Memory Usage: 45.2%

[INFO] Waiting 30 seconds until next collection...
```

---

### 7. View in Admin Interface

Visit: http://localhost:8000/admin/devices/device/

You'll see:
- ✅ Device with **hostname**, **OS**, **CPU** info
- ✅ **Memory usage** with color-coded progress bar
- ✅ **Online status** (● Online / ● Stale / ● Offline)
- ✅ **543 services** tracked
- ✅ **22 applications** tracked
- ✅ **Last heartbeat** timestamp

Click on device to see:
- Complete device details
- IP addresses (IPv4 → [IPv6...])
- Memory breakdown (total/used/available)
- Links to services and software lists

---

## 🔐 Authentication & Registration

### Self-Registration Workflow (Recommended)

The new self-registration system allows devices to request access automatically:

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Device initiates registration                            │
│    $ device-agent --init                                    │
│    Sends: agent_id, hostname, OS, fingerprint               │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Backend creates PendingRegistration                      │
│    Status: "pending"                                        │
│    Stored: device_fingerprint (for binding)                 │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Admin reviews in Django admin                            │
│    Sees: Agent ID, Hostname, OS, Fingerprint                │
│    Action: Click "Approve" or "Reject"                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. On approval:                                             │
│    - Token auto-generated                                   │
│    - Token pre-bound to device fingerprint                  │
│    - Status: "approved"                                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. Device checks status                                     │
│    $ device-agent --check-status                            │
│    Receives token if approved                               │
│    Auto-saves encrypted token                               │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 6. Device starts monitoring                                 │
│    $ device-agent                                           │
│    Token already bound to hardware                          │
│    Cannot be used on different device                       │
└─────────────────────────────────────────────────────────────┘
```

---

### Device Fingerprinting & Security

#### How Device Binding Works

1. **Fingerprint Generation**: Agent generates unique hardware fingerprint
   - **macOS**: Hardware UUID + Serial Number + MAC address
   - **Linux**: Machine ID + Product UUID + Board Serial + MAC
   - **Windows**: UUID + BIOS Serial + MAC
   - **Hash**: SHA-256 hash of combined identifiers

2. **First Heartbeat**: Token automatically bound to fingerprint
   ```
   Token "agt_abc123" → Bound to fingerprint "a1b2c3d4e5f6..."
   ```

3. **Every Subsequent Request**: Backend validates fingerprint
   ```
   Request fingerprint == Stored fingerprint ✅ → Allow
   Request fingerprint != Stored fingerprint ❌ → Reject + Alert
   ```

---

#### Theft Detection

If someone steals a token and tries to use it on a different device:

```
┌─────────────────────────────────────────────────────────────┐
│ Attacker steals token from Device A                         │
│ Token: agt_abc123 (bound to fingerprint: aaa111)            │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ Attacker uses token on Device B                             │
│ Device B fingerprint: bbb222                                │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend validates:                                          │
│   Expected: aaa111                                          │
│   Received: bbb222                                          │
│   Result: MISMATCH! ❌                                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ Backend actions:                                            │
│ 1. Returns 401 Unauthorized                                 │
│ 2. Logs security alert:                                     │
│    🚨 SECURITY ALERT: Fingerprint mismatch                  │
│ 3. Increments mismatch counter                              │
│ 4. After 5 mismatches → Auto-disable token                  │
└─────────────────────────────────────────────────────────────┘
```

**Django Admin shows:**
- ⚠ Mismatch count with warnings
- 🚨 Last mismatch timestamp
- Token auto-disabled after 5 attempts

---

### Security Features

✅ **Hardware-based identity**: Cannot be spoofed or transferred  
✅ **Token binding**: One token locked to one physical device  
✅ **Theft detection**: Immediate alerts when token used elsewhere  
✅ **Auto-disable**: Token disabled after repeated misuse  
✅ **Encrypted storage**: AES-256-GCM encryption at rest  
✅ **OS permissions**: Token files restricted (600 - owner only)  
✅ **Audit trail**: All auth attempts logged with fingerprints  
✅ **Admin visibility**: See which device each token is bound to  

---

### Manual Registration (Legacy)

For manual token distribution:

```bash
# 1. Admin creates token in Django admin
# 2. Admin copies token
# 3. On device:
device-agent --register agt_xxxxxxxxxxxxxxxxxxxxxx
```

**Note**: Self-registration is recommended for better security and scalability.

---

## 💻 Usage

### Agent Commands

```bash
# Self-registration workflow (Recommended)
device-agent --init                    # Request registration
device-agent --check-status            # Check if approved + get token

# Manual registration (Legacy)
device-agent --register <token>        # Register with pre-created token

# Token management
device-agent --check-token             # Verify token exists and is valid
device-agent --unregister              # Delete token and key files

# Monitoring
device-agent                           # Start continuous monitoring

# Help
device-agent --help                    # Show all commands
```

---

### Backend Management

#### Admin Interface Sections

**Main Dashboard**: http://localhost:8000/admin/

1. **Pending Registrations** (`/admin/agents/pendingregistration/`)
   - View all device registration requests
   - See: Agent ID, Hostname, OS, Fingerprint
   - Actions: Approve, Reject
   - Bulk actions: Approve/reject multiple
   - Filter by: Status, OS type, Request date

2. **Agent Tokens** (`/admin/agents/agenttoken/`)
   - View all registered tokens
   - See: Agent ID, Active status, Binding status, Mismatch alerts
   - Device binding info: Hostname, Fingerprint
   - Security: Mismatch count with color-coded warnings
   - Actions: Deactivate tokens

3. **Devices** (`/admin/devices/device/`)
   - View all monitored devices
   - See: Hostname, OS, Memory usage bar, Online status
   - Click device for: Full details, IP addresses, Memory breakdown
   - Links to: Services list, Software list
   - Filter by: Online status, OS type, Last heartbeat

4. **Device Services** (`/admin/devices/deviceservice/`)
   - View all services across all devices
   - See: Service name, Device, Active status
   - Filter by: Active/Inactive, OS type, Device
   - Search: Service name, Device hostname

5. **Device Software** (`/admin/devices/devicesoftware/`)
   - View all software across all devices
   - See: Software name, Device, Installed status
   - Filter by: Installed/Uninstalled, OS type, Device
   - Search: Software name, Device hostname

---

## 📊 Admin Interface

### Device List View

Beautiful visualization with:

| Feature | Description |
|---------|-------------|
| **Hostname** | Device name with link to details |
| **Agent** | Linked agent ID |
| **OS** | Operating system and version |
| **Status** | ● Online (green) / ● Stale (orange) / ● Offline (red) |
| **Memory** | Color-coded progress bar (45.2%) |
| **Services** | Active service count (543) |
| **Software** | Installed app count (22) |
| **Last Seen** | Relative time (2m ago, 5h ago) |

### Device Detail View

Organized sections:

**Device Information:**
- Agent token (linked)
- Hostname
- OS type and version
- CPU information

**Memory Information:**
- Total Memory: 8.00 GB
- Available Memory: 4.23 GB
- Used Memory: 3.77 GB
- Memory Usage: 45.2%

**Network Information:**
- IP Addresses (formatted JSON):
  ```json
  {
    "192.168.1.4": [
      "2401:4900:1c65:3792:c5f:98ad:1144:577a",
      "2401:4900:1c65:3792:31c3:2e1:2818:adab"
    ]
  }
  ```

**Status:**
- Online status
- Last heartbeat
- First seen

**Statistics:**
- **543 active services** (clickable link)
- **22 installed applications** (clickable link)

---

## ⚙️ Configuration

### Agent Configuration (config.toml)

```toml
[collection]
interval_seconds = 30              # Collection frequency (30 seconds)

[output]
output_directory = "./data"        # Where to save JSON files
save_to_file = true                # Save data locally
timestamp_format = "%Y%m%d_%H%M%S" # Filename timestamp format

[logging]
level = "info"                     # Log level: trace, debug, info, warn, error
console = true                     # Log to console
file = true                        # Log to file
log_directory = "./logs"           # Log file location

[agent]
agent_id = "laptop-001"            # Unique agent identifier
agent_name = "John's MacBook Pro"  # Human-readable name

[retry]
max_retries = 3                    # Max retry attempts on failure
initial_delay_ms = 1000            # Initial retry delay (1 second)
max_delay_ms = 10000               # Max retry delay (10 seconds)

[server]
enabled = true                     # Enable backend communication
url = "http://localhost:8000/api/heartbeat/"  # Backend URL
timeout_seconds = 30               # HTTP request timeout
```

---

### Backend Configuration (.env)

```bash
# Django Settings
DEBUG=True
SECRET_KEY=your-secret-key-here
ALLOWED_HOSTS=localhost,127.0.0.1

# Database
DATABASE_URL=sqlite:///db.sqlite3
# For PostgreSQL:
# DATABASE_URL=postgresql://user:password@localhost:5432/deviceagent

# CORS
CORS_ALLOW_ALL_ORIGINS=True         # Development only!
# For production:
# CORS_ALLOWED_ORIGINS=https://yourdomain.com

# Logging
LOG_LEVEL=INFO
```

---

## 🐛 Troubleshooting

### Agent Issues

#### "Fingerprint mismatch detected"

**Cause:** Token being used on different device.

**Symptoms:**
```
[ERROR] ✗ Authentication failed (401 Unauthorized)
[ERROR]    Device fingerprint mismatch
```

**Solution:**
This is a **security feature** - it means someone is trying to use your token on a different device.

1. If this is expected (you moved the agent):
   - Unregister on old device: `device-agent --unregister`
   - Request new registration: `device-agent --init`
   - Get approval from admin
   
2. If this is unexpected:
   - **Security incident!** Token may be compromised
   - Check Django admin for mismatch alerts
   - Revoke token immediately
   - Issue new token

---

#### "Registration already pending"

**Cause:** You already requested registration.

**Solution:**
```bash
# Check status
device-agent --check-status

# If pending too long, contact admin
```

---

#### "Device already registered"

**Cause:** This hardware already has a token.

**Solutions:**

1. **If you forgot:** Check for existing token
   ```bash
   device-agent --check-token
   ```

2. **If lost token:** Contact admin to:
   - Find your existing token (search by hostname)
   - Or delete old registration and request new one

---

### Backend Issues

#### How to view security alerts

```bash
# In Django admin
# Go to: Agents → Agent Tokens
# Look for: ⚠ or 🚨 in "Security Alerts" column
# Click token to see: Mismatch count, Last mismatch time

# In logs
tail -f backend/logs/api.log | grep "SECURITY ALERT"
```

---

#### How to handle compromised tokens

1. **Identify compromised token:**
   - Check "Security Alerts" column in admin
   - Look for high mismatch counts

2. **Disable token:**
   - Edit token in admin
   - Uncheck "Is active"
   - Save

3. **Contact device owner:**
   - Ask them to re-register
   - Investigate how token was compromised

4. **Review logs:**
   - Check attempted fingerprints
   - Identify unauthorized device

---

## 📚 API Documentation

### Authentication

All protected endpoints require:
```
Authorization: Bearer agt_xxxxxxxxxxxxxxxxxxxxxx
```

Plus device fingerprint in request body:
```json
{
  "device_fingerprint": "a1b2c3d4e5f6abcdef..."
}
```

---

### Endpoints

#### Health Check

**GET** `/api/health/`

No authentication required.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2025-02-12T10:30:00.000000",
  "version": "2.0.0"
}
```

---

#### Register Device

**POST** `/api/agents/register/`

No authentication required (public endpoint).

**Request:**
```json
{
  "agent_id": "laptop-001",
  "agent_name": "John's MacBook Pro",
  "hostname": "Johns-MacBook-Pro",
  "os_type": "macos",
  "os_version": "14.0",
  "device_fingerprint": "a1b2c3d4e5f6abcdef1234567890..."
}
```

**Response (Success - 202 Accepted):**
```json
{
  "status": "pending",
  "message": "Registration request submitted successfully. Waiting for admin approval.",
  "registration_id": 1,
  "agent_id": "laptop-001",
  "agent_name": "John's MacBook Pro",
  "instructions": "Check status with: device-agent --check-status"
}
```

**Response (Already Registered - 409 Conflict):**
```json
{
  "error": "Device already registered",
  "message": "This device is already registered. If you lost your token, contact your administrator.",
  "agent_id": "laptop-001"
}
```

---

#### Check Registration Status

**GET** `/api/agents/register/<agent_id>/status/`

No authentication required.

**Response (Approved):**
```json
{
  "status": "approved",
  "message": "Registration approved! Token is ready.",
  "agent_id": "laptop-001",
  "agent_name": "John's MacBook Pro",
  "token": "agt_xxxxxxxxxxxxxxxxxxxxx",
  "approved_at": "2025-02-12T10:30:00.000000"
}
```

**Response (Pending):**
```json
{
  "status": "pending",
  "message": "Registration is pending admin approval",
  "agent_id": "laptop-001",
  "requested_at": "2025-02-12T10:25:00.000000"
}
```

---

#### Heartbeat (With Database Storage)

**POST** `/api/heartbeat/`

Authentication required + device fingerprint.

**Request:**
```json
{
  "agent_id": "laptop-001",
  "agent_name": "John's MacBook Pro",
  "device_fingerprint": "a1b2c3d4e5f6abcdef...",
  "hostname": "Johns-MacBook-Pro",
  "os_type": "macos",
  "os_version": "14.0",
  "cpu_info": "Apple M1",
  "memory_total": 8589934592,
  "memory_available": 4294967296,
  "ip_addresses": {
    "192.168.1.4": [
      "2401:4900:1c65:3792:c5f:98ad:1144:577a"
    ]
  },
  "services": [
    "com.apple.Finder",
    "com.apple.Safari",
    ...
  ],
  "installed_software": [
    "Visual Studio Code",
    "Google Chrome",
    ...
  ],
  "collected_at": "2025-02-12T10:30:00Z"
}
```

**Response (Success - 200 OK):**
```json
{
  "status": "success",
  "message": "Heartbeat received and stored",
  "timestamp": "2025-02-12T10:30:00.000000",
  "device_id": 1,
  "device": {
    "hostname": "Johns-MacBook-Pro",
    "os": "macos 14.0",
    "memory_usage_percent": 45.2,
    "services_count": 543,
    "software_count": 22
  }
}
```

**Response (Fingerprint Mismatch - 401 Unauthorized):**
```json
{
  "error": "Authentication failed",
  "message": "Device fingerprint mismatch. This token is bound to a different device."
}
```

---

## 🔒 Security

### Security Features Summary

| Feature | Description | Benefit |
|---------|-------------|---------|
| **Device Fingerprinting** | SHA-256 hash of hardware IDs | Unique, unforgeable device identity |
| **Token Binding** | Token locked to specific fingerprint | Cannot reuse token on different device |
| **Theft Detection** | Monitors fingerprint on every request | Immediate detection of token theft |
| **Auto-Disable** | Token disabled after 5 mismatches | Automatic protection against attacks |
| **Encrypted Storage** | AES-256-GCM encryption | Protects token at rest |
| **File Permissions** | 600 (owner read/write only) | OS-level protection |
| **Audit Trail** | All attempts logged | Full visibility into auth events |
| **Admin Approval** | Manual review before access | Human verification gate |

---

### Best Practices

1. **Use self-registration**: Better security than manual token distribution
2. **Monitor mismatch alerts**: Review security alerts regularly
3. **Revoke compromised tokens**: Immediately disable if suspicious
4. **Rotate tokens**: Periodically refresh tokens for security
5. **Limit token access**: Only admins should see tokens
6. **Use HTTPS**: Always use HTTPS in production
7. **Enable firewall**: Restrict backend access to known IPs

---

## 🎯 Roadmap

### Phase 1: ✅ Basic Communication (Complete)
- ✅ Rust agent collects system data
- ✅ Django REST API backend
- ✅ Basic heartbeat endpoint
- ✅ JSON file output

### Phase 2: ✅ Authentication & Security (Complete)
- ✅ Token-based authentication
- ✅ Encrypted token storage (AES-256-GCM)
- ✅ Self-registration workflow
- ✅ Admin approval system
- ✅ Device fingerprinting (SHA-256)
- ✅ Token binding to hardware
- ✅ Theft detection with alerts
- ✅ Automatic token disabling
- ✅ Registration status API

### Phase 3: ✅ Database Storage (Complete)
- ✅ Device model with full info
- ✅ Service tracking (543+ services)
- ✅ Software inventory (22+ apps)
- ✅ IP address storage (IPv4 → [IPv6...])
- ✅ Memory usage tracking
- ✅ Online/offline status
- ✅ Service active/inactive tracking
- ✅ Software installed/uninstalled tracking
- ✅ Beautiful admin interface
- ✅ Memory usage visualization
- ✅ Color-coded status indicators
- ✅ Filterable/searchable lists

### Phase 4: 📋 Planned
- ⏳ Web dashboard (non-admin)
- ⏳ Real-time monitoring with WebSockets
- ⏳ Historical data & charts
- ⏳ Alerts and notifications
- ⏳ Email alerts on security events
- ⏳ Multi-tenancy support
- ⏳ Role-based access control
- ⏳ Advanced analytics
- ⏳ Device grouping/tagging
- ⏳ Custom metrics

---

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open Pull Request

**Development Guidelines:**
- Follow Rust style guidelines (`cargo fmt`)
- Write tests for new features
- Update documentation
- Test on multiple platforms (macOS, Linux, Windows)

---

## 📝 License

This project is licensed under the MIT License.

---

## 🔗 Links

- **Repository**: https://github.com/yourusername/device-agent
- **Issues**: https://github.com/yourusername/device-agent/issues
- **Documentation**: https://github.com/yourusername/device-agent/wiki

---

## 📞 Support

For support:
1. Check the [Troubleshooting](#troubleshooting) section
2. Review [existing issues](https://github.com/yourusername/device-agent/issues)
3. Open a [new issue](https://github.com/yourusername/device-agent/issues/new)

---