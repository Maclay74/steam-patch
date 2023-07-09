#!/bin/sh

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

echo "Uninstalling Ally Steam Patches..."

USER_DIR="$(getent passwd $SUDO_USER | cut -d: -f6)"
WORKING_FOLDER="${USER_DIR}/ally-steam-patch"

# Disable and remove services
sudo systemctl disable --now ally-steam-patch > /dev/null
sudo rm -f "${USER_DIR}/.config/systemd/user/ally-steam-patch.service"
sudo rm -f "/etc/systemd/system/ally-steam-patch.service"

# Remove temporary folder if it exists from the install process
rm -rf "/tmp/ally-steam-patch"

# Cleanup services folder
sudo rm "${WORKING_FOLDER}"
