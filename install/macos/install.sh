#!/bin/bash
# install/macos/install.sh - Install device-agent as macOS service

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=================================="
echo "Device Agent - macOS Installation"
echo "=================================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Error: Please run as root (use sudo)${NC}"
    exit 1
fi

# Variables
BINARY_PATH="/usr/local/bin/device-agent"
CONFIG_DIR="/usr/local/etc/device-agent"
DATA_DIR="/usr/local/var/device-agent"
PLIST_PATH="$HOME/Library/LaunchAgents/com.deviceagent.plist"

# Step 1: Build release binary
echo -e "${YELLOW}Step 1: Building release binary...${NC}"
cargo build --release

# Step 2: Copy binary
echo -e "${YELLOW}Step 2: Installing binary to $BINARY_PATH...${NC}"
cp target/release/device-agent "$BINARY_PATH"
chmod +x "$BINARY_PATH"
echo -e "${GREEN}✓ Binary installed${NC}"

# Step 3: Create directories
echo -e "${YELLOW}Step 3: Creating directories...${NC}"
mkdir -p "$CONFIG_DIR"
mkdir -p "$DATA_DIR/data"
mkdir -p "$DATA_DIR/logs"
echo -e "${GREEN}✓ Directories created${NC}"

# Step 4: Copy configuration
echo -e "${YELLOW}Step 4: Installing configuration...${NC}"
if [ ! -f "$CONFIG_DIR/config.toml" ]; then
    cp config.toml "$CONFIG_DIR/config.toml"
    # Update paths in config
    sed -i '' "s|output_directory = \"./data\"|output_directory = \"$DATA_DIR/data\"|g" "$CONFIG_DIR/config.toml"
    sed -i '' "s|log_directory = \"./logs\"|log_directory = \"$DATA_DIR/logs\"|g" "$CONFIG_DIR/config.toml"
    echo -e "${GREEN}✓ Configuration installed${NC}"
else
    echo -e "${YELLOW}! Configuration already exists, skipping${NC}"
fi

# Step 5: Install launchd plist
echo -e "${YELLOW}Step 5: Installing launchd service...${NC}"
mkdir -p "$HOME/Library/LaunchAgents"
cp install/macos/com.deviceagent.plist "$PLIST_PATH"

# Update paths in plist
sed -i '' "s|/usr/local/bin/device-agent|$BINARY_PATH|g" "$PLIST_PATH"
sed -i '' "s|/usr/local/var/device-agent|$DATA_DIR|g" "$PLIST_PATH"
sed -i '' "s|/usr/local/etc/device-agent|$CONFIG_DIR|g" "$PLIST_PATH"

echo -e "${GREEN}✓ Service installed${NC}"

# Step 6: Load service
echo -e "${YELLOW}Step 6: Starting service...${NC}"
launchctl unload "$PLIST_PATH" 2>/dev/null || true
launchctl load "$PLIST_PATH"
echo -e "${GREEN}✓ Service started${NC}"

echo ""
echo "=================================="
echo -e "${GREEN}Installation Complete!${NC}"
echo "=================================="
echo ""
echo "Service Status Commands:"
echo "  Start:   launchctl start com.deviceagent"
echo "  Stop:    launchctl stop com.deviceagent"
echo "  Status:  launchctl list | grep deviceagent"
echo "  Logs:    tail -f $DATA_DIR/logs/agent_*.log"
echo ""
echo "Configuration: $CONFIG_DIR/config.toml"
echo "Data Files:    $DATA_DIR/data/"
echo "Log Files:     $DATA_DIR/logs/"
echo ""