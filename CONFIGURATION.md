# Configuration Guide

## Rust Agent Configuration

1. Copy the example config:
```bash
   cp config.example.toml config.toml
```

2. Edit `config.toml`:
   - Set `agent_id` to a unique identifier
   - Set `agent_name` to a descriptive name
   - Update `server.url` to your backend URL
   - Set `server.enabled = true` to enable backend communication

**Important:** `config.toml` is ignored by git (contains sensitive tokens)

---

## Django Backend Configuration

1. Copy the example environment file:
```bash
   cp backend/.env.example backend/.env
```

2. Edit `backend/.env`:
   - Set `SECRET_KEY` to a secure random string
   - Update `ALLOWED_HOSTS` for production
   - Configure database settings if using PostgreSQL

**Important:** `backend/.env` is ignored by git (contains secrets)

---

## Quick Start

### Development

**Rust Agent:**
```bash
cp config.example.toml config.toml
# Edit config.toml
cargo run
```

**Django Backend:**
```bash
cd backend
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
cp .env.example .env
# Edit .env
python manage.py migrate
python manage.py runserver
```

### Production

See `install/README.md` for production deployment instructions.
