# Steam Patch

Steam Patch is a tool designed to enhance your Steam experience by applying patches to the Steam client.

## Installation

To install Steam Patch, run next command in your terminal

   ```bash
   curl -L https://github.com/Maclay74/steam-patch/releases/latest/download/install.sh | sh
   ```

To uninstall:
   ```bash
   curl -L https://github.com/Maclay74/steam-patch/releases/latest/download/uninstall.sh | sh
   ```

This script will add a new service to `systemctl` and apply the necessary patches to your Steam client. The patches will also be automatically applied every time you restart your system.

Please note that the tool does not currently support patching after a Steam restart.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use and modify the code according to the terms of the license.