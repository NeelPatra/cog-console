# CoG Console 🎮

> **A Cross-Platform Game Console Wrapper for [CogDemos.ink](https://cogdemos.ink/games)**
> *Built with Tauri v2 + GitHub Actions*

![Build Status](https://img.shields.io/github/actions/workflow/status/NeelPatra/cog-console/build.yml?label=Build&logo=github)
![License](https://img.shields.io/github/license/NeelPatra/cog-console)

**CoG Console** is a lightweight, native application that turns the CogDemos gaming platform into a dedicated desktop and mobile experience. It features automatic content updates, a dedicated gaming window, and cross-platform support.

## 🚀 Features
* **Zero-Maintenance Content:** The app acts as a live portal. Any new game uploaded to the website appears instantly in the app.
* **Lightweight:** ~8MB on Windows, ~27MB on Android.
* **Cross-Platform:** One codebase builds for both Windows and Android.
* **Cloud Built:** The entire build process is automated via GitHub Actions—no local development environment required.

## 📥 Downloads
Go to the **[Releases Page](../../releases)** to download the latest version:
* 📱 **Android:** `CoG-Console-Android-Signed.apk` (Works on Android 10+)
* 💻 **Windows:** `CoG-Console-Windows.exe` (Installer)

---

## 🛠️ How to Build Your Own Console (Fork This Repo)

Want to turn *your* website into an app? You can use this repository as a template! You don't need to install Android Studio or Rust.

### Step 1: Fork the Repo
Click the **Fork** button at the top right of this page to copy this repository to your own GitHub account.

### Step 2: Configure Your URL
1.  Open the file `src-tauri/tauri.conf.json`.
2.  Find the `windows` section (around line 13).
3.  Change the `"url"` to your own website:
    ```json
    "windows": [
      {
        "title": "CoG Console",
        "label": "main",
        "width": 1024,
        "height": 768,
        "resizable": true,
        "fullscreen": false,
        "url": "https://cogdemos.ink/games"
      }
    ],
    ```
    *(Tip: You can also change `productName` at the top of the file to rename your app).*
4.  Commit the changes.

### Step 3: Run the Build Factory
1.  Go to the **Actions** tab in your forked repo.
2.  Select **Build CoG Console** on the left sidebar.
3.  Click the **Run workflow** ▾ dropdown and hit the green button.

**Wait ~10-15 minutes.**
GitHub will automatically:
1.  Set up the environment (Java, Rust, Node, Android SDK).
2.  Inject optimization settings to keep the app small.
3.  Generate valid signing keys.
4.  Build and Sign the APK and EXE.

When finished, download your custom apps from the "Artifacts" section of the run!

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


## 📜 License
This project is open-source and available under the **MIT License**. Feel free to fork, modify, and distribute your own versions.
