# ⚙️ Steam Patch
Steam Patch is a tool designed to enhance your Steam experience by applying patches to the Steam client. 

## 📥 Installation
To install Steam Patch, run next command in your terminal

   ```bash
   curl -L https://github.com/Maclay74/steam-patch/releases/latest/download/install.sh | sh
   ```

To uninstall:
   ```bash
   curl -L https://github.com/Maclay74/steam-patch/releases/latest/download/uninstall.sh | sh
   ```

This script will add a new service to `systemctl` and apply the necessary patches to your Steam client. The patches will also be automatically applied every time you restart your system.

## 📋 Available Patches

Here is a list of currently available patches that can be applied:

1. **Fixes broken TDP slider in Quick Access Menu**: This patch fixes issues with the TDP slider in the Quick Access Menu.

2. **Replaced XBox menu icon with Steam one**: This patch replaces the XBox menu icon with the Steam one for a more cohesive look and feel.

## 🎯 Supported Devices

Here is a list of supported devices for the Steam Patch. Please replace the dummy values with actual ones when available:

- Asus Rog Ally (30 TDP, changes thermal policy)
- Ayaneo 2, Geek 1S (28 TDP) 
- GPD WM2 (28 TDP)
- Any other AMD device (25 TDP)

Before adjusting the TDP, please ensure your device can support the new value. 
There is a tangible risk of causing damage to your device otherwise. 
If you're aware that your device has different limitations, kindly reach out to me via 
[Discord](https://discordapp.com/users/maclay74), 
[Telegram](https://t.me/mikefinch), or 
[email](mailto:mishakozlov74@gmail.com), and I will handle it.

## 📝 License

This project is licensed under the [MIT License](LICENSE). Feel free to use and modify the code according to the terms of the license.

I've added a new section for supported devices and removed the note about no support of patching after a Steam restart as you requested.