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

1. **TDP Slider Fix for Quick Access Menu**: This patch addresses and resolves the issues with the TDP slider in the Quick Access Menu, ensuring a smoother user experience.

2. **Menu Icon Replacement** For a more integrated and consistent look, this patch replaces <picture> <source media="(prefers-color-scheme: light)" srcset="https://github-production-user-asset-6210df.s3.amazonaws.com/5504685/255038062-d99f3be6-ff5a-4570-9f21-a59204ccc804.png"> <img src="https://github-production-user-asset-6210df.s3.amazonaws.com/5504685/255038464-eb72c683-a1a5-4e5c-b81a-0131f8a76dd7.png" height="20" align="center"> </picture> icon to <picture> <source media="(prefers-color-scheme: light)" srcset="https://github.com/Maclay74/steam-patch/assets/5504685/9d15c179-bb92-4463-9a06-f8faecccf5fe"> <img src="https://github.com/Maclay74/steam-patch/assets/5504685/c76f7637-9f82-4786-b936-0ee3d99039e3" height="20" align="center"> </picture>
3. **Mapping Device-Specific Buttons for Asus Rog Ally**: This patch adjusts the mapping of the Asus Rog Ally's device-specific buttons for the Main Menu and Quick Access Menu to match the button mapping of the Steam Deck..

## 🎯 Supported Devices

Below is a list of devices supported by the Steam Patch:

- **Asus Rog Ally** (30 TDP, changes thermal policy) 
- Aya Neo 2, Geek 1S (28 TDP)
- GPD WM2 (28 TDP)
- Any other AMD device (25 TDP)

⚠️ **Please note**: From version 0.5 onwards, for **Asus Rog Ally**, it becomes necessary to disable **HandyGCCS**. 
This is because the patch now uses a different method to support the Menu and QAM buttons, 
and HandyGCCS can interfere with this new approach. Use the following command to disable HandyGCCS:
```
sudo systemctl disable handycon
```
To enable it back:
```
sudo systemctl enable handycon
```

Before adjusting the TDP, please ensure your device can support the new value. 
There is a tangible risk of causing damage to your device otherwise. 
If you're aware that your device has different limitations, kindly reach out to me via 
[Discord](https://discordapp.com/users/maclay74), 
[Telegram](https://t.me/mikefinch), or 
[email](mailto:mishakozlov74@gmail.com), and I will handle it.

## 📝 License

This project is licensed under the [MIT License](LICENSE). Feel free to use and modify the code according to the terms of the license.

I've added a new section for supported devices and removed the note about no support of patching after a Steam restart as you requested.
