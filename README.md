# BakkesMod Plugin Installer

A simple Rust utility to install a plugin DLL into **BakkesMod** and update the `plugins.cfg` file automatically.

---

## 🚀 Usage

1. Place your plugin DLL in the same directory as the installer executable.
2. Run the installer. It will:
   - Copy the DLL into the BakkesMod `plugins` folder.
   - Add the plugin load line to `plugins.cfg`.

---

## ⚙️ Customization

You can edit the constants in [`src/main.rs`](src/main.rs) to change:
- Plugin name  
- Folder paths  
- Config line  

---

## 🛠️ Building

```sh
cargo build --release
```

## 📦 Requirements
- [Rust](https://rustup.rs) (latest stable recommended)
- Bakeksmod installed

## License

This project is licensed under the [MIT License](LICENSE).