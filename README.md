# Device Agent - System Monitoring Platform

A comprehensive cross-platform device monitoring solution consisting of a Rust-based agent and Django REST API backend.

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
- [Development](#development)
- [Deployment](#deployment)
- [Troubleshooting](#troubleshooting)
- [API Documentation](#api-documentation)
- [Contributing](#contributing)

---

## 🔍 Overview

The Device Agent is a secure, lightweight monitoring solution that collects system information from devices and sends it to a centralized Django backend for storage, analysis, and management.

**Key Components:**
- **Rust Agent**: Cross-platform system monitoring agent (runs on devices)
- **Django Backend**: REST API server with database storage and admin interface

---

## 🏗 Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                     Device (macOS/Linux/Windows)            │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Rust Agent                                           │  │
│  │  ├── Collects system info every N seconds            │  │
│  │  ├── Encrypted token storage                         │  │
│  │  ├── Automatic retry with exponential backoff        │  │
│  │  └── Sends via HTTPS to backend                      │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │ HTTPS + Bearer Token
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Django Backend Server                    │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  REST API (Django + DRF)                              │  │
│  │  ├── Token authentication                             │  │
│  │  ├── Data validation                                  │  │
│  │  ├── Database storage (SQLite/PostgreSQL)            │  │
│  │  └── Admin interface                                  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## ✨ Features

### Rust Agent
- ✅ **Cross-platform**: macOS, Linux, Windows
- ✅ **System monitoring**: CPU, memory, OS info, hostname, IP addresses
- ✅ **Service tracking**: Running services/daemons
- ✅ **Software inventory**: Installed applications
- ✅ **Secure authentication**: Encrypted token storage (AES-256-GCM)
- ✅ **Automatic retry**: Exponential backoff on failures
- ✅ **Logging**: Rotating file logs + console output
- ✅ **Service installation**: systemd, launchd, Windows Service
- ✅ **Graceful shutdown**: Ctrl+C handling

### Django Backend
- ✅ **REST API**: Django REST Framework
- ✅ **Token authentication**: Secure agent authentication
- ✅ **Database storage**: SQLite (dev) / PostgreSQL (prod)
- ✅ **Admin interface**: Device management dashboard
- ✅ **Request logging**: Detailed API logs
- ✅ **CORS support**: Cross-origin requests
- ✅ **Token management**: Create, revoke, track usage

---

## 📁 Project Structure
```
device-agent/
├── backend/                      # Django REST API
│   ├── config/                   # Django settings
│   ├── agents/                   # Agent token management
│   │   ├── models.py            # AgentToken model
│   │   ├── authentication.py    # Token authentication
│   │   └── admin.py             # Admin interface
│   ├── devices/                  # Device data (Phase 3)
│   ├── api/                      # API endpoints
│   │   ├── views.py             # Heartbeat endpoint
│   │   └── urls.py              # URL routing
│   ├── logs/                     # Application logs
│   ├── manage.py
│   ├── requirements.txt
│   └── .env                      # Environment variables
│
├── src/                          # Rust agent source code
│   ├── main.rs                   # CLI + main loop
│   ├── models.rs                 # Data structures
│   ├── config.rs                 # Configuration
│   ├── crypto.rs                 # Token encryption
│   ├── sender.rs                 # HTTP client
│   ├── retry.rs                  # Retry logic
│   └── collector/                # Platform-specific collectors
│       ├── mod.rs
│       ├── common.rs
│       ├── macos.rs
│       ├── linux.rs
│       └── windows.rs
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
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

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

### 2. Create Agent Token

**Option A: Django Admin (Recommended)**

1. Visit: http://localhost:8000/admin/
2. Login with admin credentials
3. Go to: **Agents → Agent tokens**
4. Click: **Add Agent Token**
5. Fill in:
   - Agent ID: `my-device-001`
   - Agent name: `My Device`
6. Click: **Save**
7. **Copy the token** (starts with `agt_`)

**Option B: Django Shell**
```bash
cd backend
python manage.py shell
```
```python
from agents.models import AgentToken

token = AgentToken.objects.create(
    agent_id='my-device-001',
    agent_name='My Device'
)

print(f"\nToken: {token.token}")
# Copy this token!
exit()
```

---

### 3. Setup and Register Agent
```bash
# Navigate to agent directory
cd device-agent

# Copy example config
cp config.example.toml config.toml

# Edit config (optional - defaults work for local testing)
nano config.toml

# Build agent
cargo build --release

# Register agent with token from step 2
./target/release/device-agent --register agt_xxxxxxxxxxxxxxxxxxxxxx
```

**Output:**
```
=== Registering Device Agent ===

Encrypting and saving API token...
✓ Generated new encryption key at: ./.key
✓ Set file permissions to 600 (owner read/write only)
✓ Token encrypted and saved to: ./.token
✓ Token is 47 characters long

✓ Registration successful!

Your API token has been encrypted and stored securely.
You can now start the agent:
  device-agent
```

---

### 4. Start Agent
```bash
./target/release/device-agent
```

**Output:**
```
✓ Configuration loaded from config.toml
✓ Logging to file: ./logs/agent_20250208.log
[INFO] === Device Agent Starting ===
[INFO] Agent ID: my-device-001
[INFO] Collection Interval: 30 seconds
[INFO] Token Location: ./.token
[INFO] Press Ctrl+C to stop

[INFO] === Collection Iteration #1 ===
[INFO] Collection completed in 1.85s
[INFO] ✓ Data saved to: ./data/system_info_20250208_103000.json
[INFO] Sending data to backend: http://localhost:8000/api/heartbeat/
[INFO] ✓ Data sent successfully (status: 200 OK)
[INFO] ✓ Collection cycle completed successfully
[INFO] Waiting 30 seconds until next collection...
```

---

## 🔧 Installation

### Backend Setup (Detailed)

#### 1. Clone Repository
```bash
git clone https://github.com/yourusername/device-agent.git
cd device-agent/backend
```

#### 2. Create Virtual Environment
```bash
# Create
python3 -m venv venv

# Activate
source venv/bin/activate           # macOS/Linux
# or
venv\Scripts\activate              # Windows
```

#### 3. Install Dependencies
```bash
pip install -r requirements.txt
```

**requirements.txt includes:**
- Django 5.0
- Django REST Framework 3.14
- django-cors-headers 4.3
- python-decouple 3.8

#### 4. Configure Environment (Optional)
```bash
# Copy example environment file
cp .env.example .env

# Edit .env
nano .env
```

**.env variables:**
```bash
DEBUG=True
SECRET_KEY=your-secret-key-here
ALLOWED_HOSTS=localhost,127.0.0.1
CORS_ALLOW_ALL_ORIGINS=True
DATABASE_URL=sqlite:///db.sqlite3
```

#### 5. Initialize Database
```bash
# Run migrations
python manage.py makemigrations
python manage.py migrate

# Create superuser
python manage.py createsuperuser
```

#### 6. Start Development Server
```bash
python manage.py runserver

# Or specify host and port
python manage.py runserver 0.0.0.0:8000
```

---

### Agent Setup (Detailed)

#### 1. Install Rust

If you don't have Rust installed:
```bash
# Install rustup (Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### 2. Configure Agent
```bash
cd device-agent

# Copy example config
cp config.example.toml config.toml

# Edit configuration
nano config.toml
```

**Key settings in config.toml:**
```toml
[collection]
interval_seconds = 300              # How often to collect data (seconds)
include_services = true             # Collect running services
include_software = true             # Collect installed software

[server]
enabled = true                      # Enable backend communication
url = "http://localhost:8000/api/heartbeat/"  # Backend URL

[logging]
level = "info"                      # Log level: trace, debug, info, warn, error
```

#### 3. Build Agent
```bash
# Development build (faster, with debug symbols)
cargo build

# Production build (optimized)
cargo build --release
```

**Build output:**
```
   Compiling device-agent v0.1.0
    Finished release [optimized] target(s) in 45.2s
```

**Binary location:**
- Debug: `target/debug/device-agent`
- Release: `target/release/device-agent`

---

## 🔐 Authentication & Registration

### How It Works

1. **Backend generates token** (via Django admin or API)
2. **Admin copies token** (e.g., `agt_xxxxxxxxxxxxxxxxxxx`)
3. **Agent encrypts and stores token** locally using AES-256-GCM
4. **Agent sends token** in `Authorization` header with every request
5. **Backend validates token** and processes data

### Security Features

✅ **No plain-text tokens**: Token encrypted at rest  
✅ **OS-level permissions**: Token file restricted to owner (600)  
✅ **Machine-specific key**: Encryption key unique to each device  
✅ **No passwords**: Token-based authentication only  
✅ **Revocable**: Tokens can be deactivated in Django admin  
✅ **Trackable**: Last usage timestamp recorded  

---

### Registration Process

#### Step 1: Get Token from Backend

**Via Django Admin:**
1. Navigate to: http://localhost:8000/admin/agents/agenttoken/
2. Click: **Add Agent Token**
3. Enter:
   - **Agent ID**: Unique identifier (e.g., `laptop-001`)
   - **Agent Name**: Descriptive name (e.g., `John's MacBook Pro`)
4. Click: **Save**
5. **Copy the generated token**

**Via Django Shell:**
```bash
cd backend
python manage.py shell
```
```python
from agents.models import AgentToken

token = AgentToken.objects.create(
    agent_id='laptop-001',
    agent_name="John's MacBook Pro"
)

print(f"Token: {token.token}")
exit()
```

---

#### Step 2: Register Agent
```bash
cd device-agent

# Register with token
./target/release/device-agent --register agt_xxxxxxxxxxxxxxxxxxxxxx
```

**What happens:**
1. Agent validates token format
2. Generates encryption key (if not exists)
3. Encrypts token using AES-256-GCM
4. Saves to `.token` file with 600 permissions
5. Saves encryption key to `.key` file

**Files created:**
```
device-agent/
├── .token    # Encrypted API token (owner read/write only)
└── .key      # Encryption key (owner read/write only)
```

---

#### Step 3: Verify Registration
```bash
# Check token status
./target/release/device-agent --check-token
```

**Output:**
```
=== Token Status ===

✓ API token is registered
  Location: ./.token

✓ Token is valid and can be decrypted
  Length: 47 characters
  Starts with: agt_xxx...
```

---

### Token Management Commands
```bash
# Register agent
device-agent --register <token>

# Check if registered
device-agent --check-token

# Unregister (delete token)
device-agent --unregister

# Show help
device-agent --help
```

---

### Token Lifecycle
```
┌─────────────────────────────────────────────────────────────┐
│ 1. Admin creates token in Django                            │
│    Token: agt_abc123...                                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Admin copies token and registers agent                   │
│    $ device-agent --register agt_abc123...                  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Agent encrypts and stores token                          │
│    File: .token (encrypted)                                 │
│    Key: .key (encryption key)                               │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Agent starts and loads token                             │
│    Decrypts token from .token using .key                    │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. Agent sends heartbeat with token                         │
│    Authorization: Bearer agt_abc123...                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ 6. Backend validates token                                  │
│    ✓ Token exists in database                               │
│    ✓ Token is active                                        │
│    ✓ Updates last_used timestamp                            │
└─────────────────────────────────────────────────────────────┘
```

---

### Revoking Tokens

**To revoke a token:**

1. Go to Django Admin: http://localhost:8000/admin/agents/agenttoken/
2. Find the token
3. Uncheck **Is active**
4. Click **Save**

**Result:** Agent will get `401 Unauthorized` on next heartbeat.

**Agent logs will show:**
```
[ERROR] ✗ Authentication failed (401 Unauthorized)
[ERROR]    Your API token may be invalid or expired
[ERROR]    Try re-registering: device-agent --register <new_token>
```

---

## 💻 Usage

### Agent Commands
```bash
# Start agent (continuous monitoring)
device-agent

# Register with token
device-agent --register <token>

# Check token status
device-agent --check-token

# Unregister (delete token)
device-agent --unregister

# Show help
device-agent --help
```

---

### Backend Management

#### Django Admin Interface

Access at: http://localhost:8000/admin/

**Available sections:**
- **Agents → Agent tokens**: Manage API tokens
- **Authentication**: User management
- **Logs**: View application logs (in `backend/logs/`)

---

#### API Endpoints

**Health Check:**
```bash
curl http://localhost:8000/api/health/
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2025-02-08T10:30:00.000000",
  "version": "1.0.0"
}
```

**Heartbeat (requires auth):**
```bash
curl -X POST http://localhost:8000/api/heartbeat/ \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer agt_xxxxxxxxxxxxxx" \
  -d '{
    "agent_id": "test-001",
    "hostname": "test-machine",
    "os_type": "macos"
  }'
```

**Response:**
```json
{
  "status": "success",
  "message": "Heartbeat received and authenticated",
  "timestamp": "2025-02-08T10:30:00.000000",
  "agent": {
    "agent_id": "test-001",
    "agent_name": "Test Agent"
  }
}
```

---

### Viewing Logs

#### Agent Logs
```bash
# Real-time logs
tail -f logs/agent_*.log

# View last 100 lines
tail -n 100 logs/agent_20250208.log

# Search logs
grep ERROR logs/agent_*.log
```

#### Backend Logs
```bash
cd backend

# View all logs
tail -f logs/django.log

# View API logs only
tail -f logs/api.log

# View errors only
tail -f logs/errors.log
```

---

## ⚙️ Configuration

### Agent Configuration (config.toml)
```toml
[collection]
interval_seconds = 300              # Collection frequency (5 minutes)
include_services = true             # Collect running services
include_software = true             # Collect installed applications

[output]
output_directory = "./data"         # Where to save JSON files
save_to_file = true                 # Save data locally
timestamp_format = "%Y%m%d_%H%M%S"  # Filename timestamp format

[logging]
level = "info"                      # Log level
console = true                      # Log to console
file = true                         # Log to file
log_directory = "./logs"            # Log file location

[agent]
agent_id = "my-device-001"          # Unique agent identifier
agent_name = "My Device"            # Human-readable name

[retry]
max_retries = 5                     # Max retry attempts on failure
initial_delay_ms = 1000             # Initial retry delay (1 second)
max_delay_ms = 60000                # Max retry delay (60 seconds)

[server]
enabled = true                      # Enable backend communication
url = "http://localhost:8000/api/heartbeat/"  # Backend URL
timeout_seconds = 30                # HTTP request timeout
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

## 🛠 Development

### Running in Development Mode

**Backend:**
```bash
cd backend
source venv/bin/activate
python manage.py runserver
```

**Agent:**
```bash
cd device-agent
cargo run  # Uses debug build
```

---

### Project Development Setup
```bash
# Clone repository
git clone https://github.com/yourusername/device-agent.git
cd device-agent

# Setup backend
cd backend
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python manage.py migrate
python manage.py createsuperuser

# Setup agent
cd ..
cp config.example.toml config.toml
cargo build

# Run tests
cargo test
```

---

### Adding New Features

**Backend (Django):**
1. Create/modify models in `devices/models.py`
2. Create migrations: `python manage.py makemigrations`
3. Apply migrations: `python manage.py migrate`
4. Update admin: `devices/admin.py`
5. Update views: `api/views.py`

**Agent (Rust):**
1. Update data structures: `src/models.rs`
2. Update collectors: `src/collector/`
3. Build: `cargo build --release`
4. Test: `cargo test`

---

## 🚀 Deployment

### Production Deployment (Linux)

#### Backend Deployment

**1. Install dependencies:**
```bash
sudo apt update
sudo apt install python3 python3-pip postgresql nginx
```

**2. Setup PostgreSQL:**
```bash
sudo -u postgres createdb deviceagent
sudo -u postgres createuser deviceagent_user
sudo -u postgres psql
```
```sql
ALTER USER deviceagent_user WITH PASSWORD 'secure_password';
GRANT ALL PRIVILEGES ON DATABASE deviceagent TO deviceagent_user;
\q
```

**3. Configure backend:**
```bash
cd device-agent/backend
pip install -r requirements.txt
pip install gunicorn psycopg2-binary

# Update .env
DATABASE_URL=postgresql://deviceagent_user:secure_password@localhost/deviceagent
DEBUG=False
ALLOWED_HOSTS=yourdomain.com
```

**4. Run migrations:**
```bash
python manage.py migrate
python manage.py collectstatic
```

**5. Setup Gunicorn:**
```bash
gunicorn config.wsgi:application --bind 0.0.0.0:8000
```

**6. Setup Nginx:**
```nginx
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    location /static/ {
        alias /path/to/device-agent/backend/staticfiles/;
    }
}
```

---

#### Agent Deployment

**Install as systemd service:**
```bash
cd device-agent
sudo ./install/linux/install.sh
```

**Manual installation:**
```bash
# Build release
cargo build --release

# Copy binary
sudo cp target/release/device-agent /usr/local/bin/

# Create config directory
sudo mkdir -p /opt/device-agent
sudo cp config.toml /opt/device-agent/

# Register agent
sudo device-agent --register agt_xxxxxxxxxxxxxx

# Create systemd service
sudo cp install/linux/device-agent.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable device-agent
sudo systemctl start device-agent

# Check status
sudo systemctl status device-agent
```

---

### macOS Deployment
```bash
cd device-agent
sudo ./install/macos/install.sh
```

**Check status:**
```bash
launchctl list | grep deviceagent
tail -f /usr/local/var/device-agent/logs/agent_*.log
```

---

### Windows Deployment

**Run PowerShell as Administrator:**
```powershell
cd device-agent
.\install\windows\install.ps1
```

**Check status:**
```powershell
sc.exe query DeviceAgent
```

---

## 🐛 Troubleshooting

### Agent Issues

#### "No API token registered"

**Cause:** Token not registered yet.

**Solution:**
```bash
device-agent --register agt_xxxxxxxxxxxxxx
```

---

#### "Authentication failed (401)"

**Cause:** Invalid or revoked token.

**Solution:**
1. Check token is active in Django admin
2. Re-register with new token:
```bash
   device-agent --unregister
   device-agent --register <new_token>
```

---

#### "Failed to send request"

**Cause:** Backend not reachable.

**Solution:**
1. Check backend is running:
```bash
   curl http://localhost:8000/api/health/
```
2. Check URL in config.toml
3. Check firewall settings

---

### Backend Issues

#### "ModuleNotFoundError"

**Cause:** Missing Python dependencies.

**Solution:**
```bash
cd backend
source venv/bin/activate
pip install -r requirements.txt
```

---

#### "OperationalError: no such table"

**Cause:** Database not initialized.

**Solution:**
```bash
python manage.py migrate
```

---

#### "CORS error"

**Cause:** CORS not configured.

**Solution:**
Add to `backend/config/settings.py`:
```python
CORS_ALLOW_ALL_ORIGINS = True  # Development
# Or for production:
CORS_ALLOWED_ORIGINS = [
    "https://yourdomain.com",
]
```

---

### Common Issues

#### Permission denied on .token file

**Solution:**
```bash
chmod 600 .token .key
```

#### Port 8000 already in use

**Solution:**
```bash
# Kill process using port 8000
sudo lsof -ti:8000 | xargs kill -9

# Or use different port
python manage.py runserver 8001
```

#### Agent not collecting services/software

**Cause:** Insufficient permissions.

**Solution:**
```bash
# Run with elevated privileges
sudo device-agent
```

---

## 📚 API Documentation

### Authentication

All API requests (except health check) require Bearer token authentication:
```
Authorization: Bearer agt_xxxxxxxxxxxxxxxxxxxxxx
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
  "timestamp": "2025-02-08T10:30:00.000000",
  "version": "1.0.0"
}
```

---

#### Heartbeat

**POST** `/api/heartbeat/`

Authentication required.

**Request:**
```json
{
  "agent_id": "device-001",
  "agent_name": "My Device",
  "hostname": "my-laptop",
  "os_type": "macos",
  "os_version": "14.0",
  "cpu_info": "Apple M1",
  "memory_total": 8589934592,
  "memory_available": 2147483648,
  "ip_addresses": {
    "en0": "192.168.1.100"
  },
  "services": [
    {"name": "service1", "status": "running"}
  ],
  "installed_software": [
    {"name": "app1", "version": "1.0"}
  ],
  "collected_at": "2025-02-08T10:30:00Z"
}
```

**Response:**
```json
{
  "status": "success",
  "message": "Heartbeat received and authenticated",
  "timestamp": "2025-02-08T10:30:00.000000",
  "agent": {
    "agent_id": "device-001",
    "agent_name": "My Device"
  },
  "received_data": {
    "hostname": "my-laptop",
    "os_type": "macos",
    "services_count": 1,
    "software_count": 1
  }
}
```

---

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open Pull Request

---

## 📝 License

This project is licensed under the MIT License - see LICENSE file for details.

---

## 🔗 Links

- **Repository**: https://github.com/yourusername/device-agent
- **Issues**: https://github.com/yourusername/device-agent/issues
- **Documentation**: https://github.com/yourusername/device-agent/wiki

---

## 📞 Support

For support, please:
1. Check the [Troubleshooting](#troubleshooting) section
2. Review [existing issues](https://github.com/yourusername/device-agent/issues)
3. Open a [new issue](https://github.com/yourusername/device-agent/issues/new)

---

## 🎯 Roadmap

### Phase 1: ✅ Basic Communication (Complete)
- ✅ Rust agent collects system data
- ✅ Django REST API backend
- ✅ Basic heartbeat endpoint

### Phase 2: ✅ Authentication (Complete)
- ✅ Token-based authentication
- ✅ Encrypted token storage
- ✅ Token management CLI

### Phase 3: 🚧 Database Storage (In Progress)
- ⏳ Device model
- ⏳ Service tracking
- ⏳ Software inventory
- ⏳ Device history

### Phase 4: 📋 Planned
- Web dashboard
- Real-time monitoring
- Alerts and notifications
- Multi-tenancy support
- Advanced analytics

---