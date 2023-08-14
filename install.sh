#!/bin/sh

[ "$UID" -eq 0 ] || exec sudo "$0" "$@"

echo "Installing Steam Patch release..."

USER_DIR="$(getent passwd $SUDO_USER | cut -d: -f6)"
WORKING_FOLDER="${USER_DIR}/steam-patch"

# Create folder structure
mkdir "${WORKING_FOLDER}"
# Enable CEF debugging
touch "${USER_DIR}/.steam/steam/.cef-enable-remote-debugging"

# Download latest release and install it
RELEASE=$(curl -s 'https://api.github.com/repos/hicder/steam-patch/releases' | jq -r "first(.[] | select(.prerelease == "false"))")
VERSION=$(jq -r '.tag_name' <<< ${RELEASE} )
DOWNLOAD_URL=$(jq -r '.assets[].browser_download_url | select(endswith("steam-patch"))' <<< ${RELEASE})

printf "Installing version %s...\n" "${VERSION}"
curl -L $DOWNLOAD_URL --output ${WORKING_FOLDER}/steam-patch
chmod +x ${WORKING_FOLDER}/steam-patch

systemctl --user stop steam-patch 2> /dev/null
systemctl --user disable steam-patch 2> /dev/null

systemctl stop steam-patch 2> /dev/null
systemctl disable steam-patch 2> /dev/null

# Add new service file
cat > "${WORKING_FOLDER}/steam-patch.service" <<- EOM
[Unit]
Description=Steam Patches Loader
Wants=network.target
After=network.target

[Service]
Type=simple
User=root
ExecStart=${WORKING_FOLDER}/steam-patch --user=${SUDO_USER}
WorkingDirectory=${WORKING_FOLDER}

[Install]
WantedBy=multi-user.target
EOM

rm -f "/etc/systemd/system/steam-patch.service"
cp "${WORKING_FOLDER}/steam-patch.service" "/etc/systemd/system/steam-patch.service"

# Run service
systemctl daemon-reload
systemctl enable steam-patch.service
systemctl start steam-patch.service
