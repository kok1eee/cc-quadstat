use std::fs;
use std::path::PathBuf;

use crate::types::Theme;

pub const THEMES: &[(&str, Theme)] = &[
    ("tokyo-night", Theme {
        name: "Tokyo Night",
        model: [15, 57],
        version: [189, 60],
        branch: [0, 179],
        ctx_good: [0, 78],
        ctx_warn: [0, 214],
        ctx_bad: [15, 197],
    }),
    ("nord", Theme {
        name: "Nord",
        model: [0, 110],
        version: [254, 60],
        branch: [0, 179],
        ctx_good: [0, 108],
        ctx_warn: [0, 222],
        ctx_bad: [15, 167],
    }),
    ("dracula", Theme {
        name: "Dracula",
        model: [15, 141],
        version: [231, 61],
        branch: [0, 228],
        ctx_good: [0, 84],
        ctx_warn: [0, 215],
        ctx_bad: [15, 210],
    }),
    ("gruvbox", Theme {
        name: "Gruvbox",
        model: [230, 66],
        version: [223, 239],
        branch: [235, 214],
        ctx_good: [235, 142],
        ctx_warn: [235, 208],
        ctx_bad: [230, 124],
    }),
    ("catppuccin", Theme {
        name: "Catppuccin",
        model: [0, 183],
        version: [189, 60],
        branch: [0, 223],
        ctx_good: [0, 158],
        ctx_warn: [0, 223],
        ctx_bad: [15, 211],
    }),
    ("monokai", Theme {
        name: "Monokai",
        model: [15, 197],
        version: [231, 239],
        branch: [0, 186],
        ctx_good: [0, 148],
        ctx_warn: [0, 208],
        ctx_bad: [15, 196],
    }),
    ("solarized", Theme {
        name: "Solarized",
        model: [230, 37],
        version: [230, 240],
        branch: [235, 136],
        ctx_good: [230, 64],
        ctx_warn: [235, 166],
        ctx_bad: [230, 124],
    }),
    ("default", Theme {
        name: "Default",
        model: [0, 44],
        version: [0, 242],
        branch: [0, 178],
        ctx_good: [0, 34],
        ctx_warn: [0, 178],
        ctx_bad: [15, 160],
    }),
];

fn find_theme(name: &str) -> Option<Theme> {
    THEMES.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
}

fn get_config_dir() -> PathBuf {
    let home = dirs_home();
    home.join(".config").join("cc-quadstat")
}

fn get_theme_file() -> PathBuf {
    get_config_dir().join("theme")
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

pub fn get_theme_from_config() -> String {
    fs::read_to_string(get_theme_file())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "tokyo-night".to_string())
}

pub fn get_theme() -> Theme {
    let name = std::env::var("CC_THEME").unwrap_or_default();
    let name = if name.is_empty() {
        get_theme_from_config()
    } else {
        name
    };
    find_theme(&name).unwrap_or_else(|| find_theme("tokyo-night").unwrap())
}

pub fn list_themes() {
    let current = get_theme_from_config();
    println!("Available themes:");
    for (name, theme) in THEMES {
        let marker = if *name == current { "* " } else { "  " };
        println!("{}{} - {}", marker, name, theme.name);
    }
}

pub fn set_theme(name: &str) {
    if find_theme(name).is_none() {
        eprintln!("Unknown theme: {}", name);
        eprintln!("Use --list-themes to see available themes");
        std::process::exit(1);
    }

    let config_dir = get_config_dir();
    if let Err(e) = fs::create_dir_all(&config_dir) {
        eprintln!("Failed to create config directory: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = fs::write(get_theme_file(), name) {
        eprintln!("Failed to save theme: {}", e);
        std::process::exit(1);
    }

    println!("Theme set to: {}", name);
}
