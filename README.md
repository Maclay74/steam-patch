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

1. **Fixing broken TDP slider in Quick Access Menu**: This patch fixes issues with the TDP slider in the Quick Access Menu.

2. **Replacing <picture> <source media="(prefers-color-scheme: light)" srcset="https://github-production-user-asset-6210df.s3.amazonaws.com/5504685/255038062-d99f3be6-ff5a-4570-9f21-a59204ccc804.png"> <img 
src="https://github-production-user-asset-6210df.s3.amazonaws.com/5504685/255038464-eb72c683-a1a5-4e5c-b81a-0131f8a76dd7.png" height="20" align="center"> </picture> menu icon to <picture> <source media="(prefers-color-scheme: light)" srcset="https://github.com/Maclay74/steam-patch/assets/5504685/9d15c179-bb92-4463-9a06-f8faecccf5fe"> <img 
src="https://github.com/Maclay74/steam-patch/assets/5504685/c76f7637-9f82-4786-b936-0ee3d99039e3" height="20" align="center"> </picture>**: This patch replaces the XBox menu icon with the Steam one for a more cohesive look and feel.

## 🎯 Supported Devices

Here is a list of supported devices for the Steam Patch.

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
