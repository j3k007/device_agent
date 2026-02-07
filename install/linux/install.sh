#!/bin/bash
# install/linux/install.sh - Install device-agent as Linux service

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "==================================="
echo "Device Agent - Linux Installation"
echo "==================================="
echo ""

if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Error: Please run as root (use sudo)${NC}"
    exit 1
fi

# Variables
BINARY_PATH="/usr/local/bin/device-agent"
INSTALL_DIR="/opt/device-agent"
SERVICE_FILE="/etc/systemd/system/device-agent.service"

# Step 1: Build
echo -e "${YELLOW}Step 1: Building release binary...${NC}"
cargo build --release
echo -e "${GREEN}✓ Build complete${NC}"

# Step 2: Create user
echo -e "${YELLOW}Step 2: Creating system user...${NC}"
if ! id -u deviceagent > /dev/null 2>&1; then
    useradd --system --no-create-home --shell /bin/false deviceagent
    echo -e "${GREEN}✓ User created${NC}"
else
    echo -e "${YELLOW}! User already exists${NC}"
fi

# Step 3: Create directories
echo -e "${YELLOW}Step 3: Creating directories...${NC}"
mkdir -p "$INSTALL_DIR"/{data,logs}
cp target/release/device-agent "$BINARY_PATH"
chmod +x "$BINARY_PATH"

if [ ! -f "$INSTALL_DIR/config.toml" ]; then
    cp config.toml "$INSTALL_DIR/config.toml"
    sed -i "s|output_directory = \"./data\"|output_directory = \"$INSTALL_DIR/data\"|g" "$INSTALL_DIR/config.toml"
    sed -i "s|log_directory = \"./logs\"|log_directory = \"$INSTALL_DIR/logs\"|g" "$INSTALL_DIR/config.toml"
fi

chown -R deviceagent:deviceagent "$INSTALL_DIR"
echo -e "${GREEN}✓ Directories created${NC}"

# Step 4: Install systemd service
echo -e "${YELLOW}Step 4: Installing systemd service...${NC}"
cp install/linux/device-agent.service "$SERVICE_FILE"
systemctl daemon-reload
systemctl enable device-agent
systemctl start device-agent
echo -e "${GREEN}✓ Service installed and started${NC}"

echo ""
echo "==================================="
echo -e "${GREEN}Installation Complete!${NC}"
echo "==================================="
echo ""
echo "Service Commands:"
echo "  Status:  sudo systemctl status device-agent"
echo "  Start:   sudo systemctl start device-agent"
echo "  Stop:    sudo systemctl stop device-agent"
echo "  Restart: sudo systemctl restart device-agent"
echo "  Logs:    sudo journalctl -u device-agent -f"
echo ""
echo "Configuration: $INSTALL_DIR/config.toml"
echo "Data Files:    $INSTALL_DIR/data/"
echo ""