#!/bin/sh

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

echo "Uninstalling Steam Patch..."

USER_DIR="$(getent passwd $SUDO_USER | cut -d: -f6)"
WORKING_FOLDER="${USER_DIR}/steam-patch"

# Disable and remove services
sudo systemctl disable --now steam-patch > /dev/null
sudo rm -f "${USER_DIR}/.config/systemd/user/steam-patch.service"
sudo rm -f "/etc/systemd/system/steam-patch.service"

# Remove temporary folder if it exists from the install process
rm -rf "/tmp/steam-patch"

# Cleanup services folder
sudo rm -rf "${WORKING_FOLDER}"
