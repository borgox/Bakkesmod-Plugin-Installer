# Plugin Installer for BakkesMod

This Rust project installs a plugin DLL and updates the `plugins.cfg` file for BakkesMod.

## Usage

1. Place your plugin DLL in the same directory as the installer executable.
2. Run the installer. It will:
   - Copy the DLL to the BakkesMod plugins folder.
   - Add the plugin load line to `plugins.cfg`.

## Customization

Edit the constants in `src/main.rs` to change plugin name, folder paths, or config line.

## Building

```sh
cargo build --release