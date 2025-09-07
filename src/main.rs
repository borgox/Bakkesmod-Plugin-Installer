use std::env;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

// -- Constants for reusability ..
const PLUGIN_DLL_NAME: &str = "RLProfilePicturesREVAMP.dll";
const PLUGIN_LOAD_LINE: &str = "plugin load rlprofilepicturesrevamp";
const BAKKESMOD_FOLDER: &str = "bakkesmod/bakkesmod";
const PLUGINS_SUBFOLDER: &str = "plugins";
const CFG_SUBFOLDER: &str = "cfg";
const PLUGINS_CFG: &str = "plugins.cfg";

fn main() {
    // --- Current Dirs ---
    let exe_path = env::current_exe().expect("Unable to retrieve current executable path");
    let exe_dir = exe_path.parent().unwrap();

    println!("Changing current working dir to executable's directory");
    env::set_current_dir(exe_dir).expect("Unable to change path");

    // --- User Dirs ---
    let appdata = env::var("APPDATA").expect("Appdata not found");
    let bm_plugins_folder = PathBuf::from(format!("{}/{}/{}", appdata, BAKKESMOD_FOLDER, PLUGINS_SUBFOLDER));
    let dll_file = exe_dir.join(PLUGIN_DLL_NAME);
    if !dll_file.exists() {
        eprintln!("Error: DLL not found in executable directory: {:?}", dll_file);
        wait_for_enter();
        std::process::exit(1);
    }
    let cfg_folder = PathBuf::from(format!("{}/{}/{}", appdata, BAKKESMOD_FOLDER, CFG_SUBFOLDER));
    let cfg_file = cfg_folder.join(PLUGINS_CFG);

    // --- DLL Installation ---
    println!("\nInstalling DLL...");
    if !bm_plugins_folder.exists() {
        eprintln!("\nError: Destination folder does not exist: {:?}", bm_plugins_folder);
        wait_for_enter();
        std::process::exit(1);
    }

    let dest_file = bm_plugins_folder.join(PLUGIN_DLL_NAME);

    // If DLL exists -> remove it
    if dest_file.exists() {
        println!("Existing DLL found, removing...");
        if let Err(err) = fs::remove_file(&dest_file) {
            eprintln!("Failed to remove old DLL: {}", err);
            wait_for_enter();
            std::process::exit(1);
        }
    }

    // Copy fresh DLL
    match fs::copy(&dll_file, &dest_file) {
        Ok(_) => println!("Plugin DLL successfully copied to {:?}", bm_plugins_folder),
        Err(err) => {
            eprintln!("Error while copying DLL: {}", err);
            wait_for_enter();
            std::process::exit(1);
        }
    }

    // --- Config Installation ---
    println!("\nUpdating plugins.cfg...");
    let new_line = PLUGIN_LOAD_LINE;

    let mut lines: Vec<String> = Vec::new();
    if cfg_file.exists() {
        if let Ok(content) = fs::read_to_string(&cfg_file) {
            // Keep all lines except the one we want to ensure is only once
            lines = content
                .lines()
                .filter(|line| line.trim() != new_line)
                .map(|s| s.to_string())
                .collect();
        }
    }

    // Add plugin line at the end
    lines.push(new_line.to_string());

    if let Err(err) = fs::write(&cfg_file, lines.join("\n")) {
        eprintln!("Failed to update plugins.cfg: {}", err);
        wait_for_enter();
        std::process::exit(1);
    } else {
        println!("plugins.cfg updated successfully.");
    }

    println!("\n\nSetup successfully completed.\n");
    wait_for_enter();
}

fn wait_for_enter() {
    println!("Press ENTER to continue...");
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
}
