#!/bin/sh

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

echo "Installing Ally Steam Patch release..."

USER_DIR="$(getent passwd $SUDO_USER | cut -d: -f6)"
WORKING_FOLDER="${USER_DIR}/ally-steam-patch"

# Create folder structure
mkdir "${WORKING_FOLDER}"
# Enable CEF debugging
touch "${USER_DIR}/.steam/steam/.cef-enable-remote-debugging"

# Download latest release and install it
RELEASE=$(curl -s 'https://api.github.com/repos/Maclay74/ally-steam-patches/releases' | jq -r "first(.[] | select(.prerelease == "false"))")
VERSION=$(jq -r '.tag_name' <<< ${RELEASE} )
DOWNLOAD_URL=$(jq -r '.assets[].browser_download_url | select(endswith("ally-steam-patch"))' <<< ${RELEASE})

printf "Installing version %s...\n" "${VERSION}"
curl -L $DOWNLOAD_URL --output ${WORKING_FOLDER}/ally-steam-patch
chmod +x ${WORKING_FOLDER}/ally-steam-patch

systemctl --user stop ally-steam-patch 2> /dev/null
systemctl --user disable ally-steam-patch 2> /dev/null

systemctl stop ally-steam-patch 2> /dev/null
systemctl disable ally-steam-patch 2> /dev/null

# Add new service file
cat > "${WORKING_FOLDER}/ally-steam-patch.service" <<- EOM
[Unit]
Description=Steam Patches Loader
Wants=network.target
After=network.target

[Service]
Type=simple
User=gamer
ExecStart=${WORKING_FOLDER}/ally-steam-patch
WorkingDirectory=${WORKING_FOLDER}

[Install]
WantedBy=multi-user.target
EOM

rm -f "/etc/systemd/system/ally-steam-patch.service"
cp "${WORKING_FOLDER}/ally-steam-patch.service" "/etc/systemd/system/ally-steam-patch.service"

# Run service
systemctl daemon-reload
systemctl enable ally-steam-patch.service
systemctl start ally-steam-patch.service