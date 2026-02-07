cat > install/linux/uninstall.sh << 'EOF'
#!/bin/bash
# install/linux/uninstall.sh - Uninstall device-agent service

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "======================================"
echo "Device Agent - Linux Uninstallation"
echo "======================================"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Error: Please run as root (use sudo)${NC}"
    exit 1
fi

# Variables
BINARY_PATH="/usr/local/bin/device-agent"
INSTALL_DIR="/opt/device-agent"
SERVICE_FILE="/etc/systemd/system/device-agent.service"

# Step 1: Stop service
echo -e "${YELLOW}Step 1: Stopping service...${NC}"
if systemctl is-active --quiet device-agent; then
    systemctl stop device-agent
    echo -e "${GREEN}✓ Service stopped${NC}"
else
    echo -e "${YELLOW}! Service not running${NC}"
fi

# Step 2: Disable service
echo -e "${YELLOW}Step 2: Disabling service...${NC}"
if systemctl is-enabled --quiet device-agent 2>/dev/null; then
    systemctl disable device-agent
    echo -e "${GREEN}✓ Service disabled${NC}"
else
    echo -e "${YELLOW}! Service not enabled${NC}"
fi

# Step 3: Remove service file
echo -e "${YELLOW}Step 3: Removing service file...${NC}"
if [ -f "$SERVICE_FILE" ]; then
    rm -f "$SERVICE_FILE"
    systemctl daemon-reload
    echo -e "${GREEN}✓ Service file removed${NC}"
else
    echo -e "${YELLOW}! Service file not found${NC}"
fi

# Step 4: Remove binary
echo -e "${YELLOW}Step 4: Removing binary...${NC}"
if [ -f "$BINARY_PATH" ]; then
    rm -f "$BINARY_PATH"
    echo -e "${GREEN}✓ Binary removed${NC}"
else
    echo -e "${YELLOW}! Binary not found${NC}"
fi

# Step 5: Ask about data removal
echo ""
echo -e "${YELLOW}Data and configuration preserved at:${NC}"
echo "  Configuration: $INSTALL_DIR/config.toml"
echo "  Data files:    $INSTALL_DIR/data/"
echo "  Log files:     $INSTALL_DIR/logs/"
echo ""

read -p "Do you want to remove all data and configuration? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Removing all data...${NC}"
    rm -rf "$INSTALL_DIR"
    echo -e "${GREEN}✓ All data removed${NC}"
    
    # Remove user if exists
    if id -u deviceagent > /dev/null 2>&1; then
        echo -e "${YELLOW}Removing system user...${NC}"
        userdel deviceagent 2>/dev/null || true
        echo -e "${GREEN}✓ User removed${NC}"
    fi
else
    echo -e "${YELLOW}Data preserved${NC}"
    echo ""
    echo "To manually remove data later:"
    echo "  sudo rm -rf $INSTALL_DIR"
    echo "  sudo userdel deviceagent"
fi

echo ""
echo "======================================"
echo -e "${GREEN}Uninstallation Complete!${NC}"
echo "======================================"
echo ""
EOF

chmod +x install/linux/uninstall.sh