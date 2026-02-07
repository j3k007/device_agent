#!/bin/bash
# install/macos/uninstall.sh - Uninstall device-agent service

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "===================================="
echo "Device Agent - macOS Uninstallation"
echo "===================================="
echo ""

if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Error: Please run as root (use sudo)${NC}"
    exit 1
fi

PLIST_PATH="$HOME/Library/LaunchAgents/com.deviceagent.plist"

# Stop and unload service
echo -e "${YELLOW}Stopping service...${NC}"
launchctl stop com.deviceagent 2>/dev/null || true
launchctl unload "$PLIST_PATH" 2>/dev/null || true
echo -e "${GREEN}✓ Service stopped${NC}"

# Remove files
echo -e "${YELLOW}Removing files...${NC}"
rm -f "/usr/local/bin/device-agent"
rm -f "$PLIST_PATH"
echo -e "${GREEN}✓ Files removed${NC}"

echo ""
echo -e "${YELLOW}Note: Data and config files preserved at:${NC}"
echo "  /usr/local/etc/device-agent/"
echo "  /usr/local/var/device-agent/"
echo ""
echo "To remove all data:"
echo "  sudo rm -rf /usr/local/etc/device-agent"
echo "  sudo rm -rf /usr/local/var/device-agent"
echo ""