mod render;
mod theme;
mod types;
mod vcs;

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use types::HookInput;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if !args.is_empty() {
        handle_cli(&args);
        return;
    }

    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading stdin");
        std::process::exit(1);
    }

    let hook: HookInput = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error parsing JSON");
            std::process::exit(1);
        }
    };

    let version = hook.version.as_deref().unwrap_or("?");
    let context_percent = calculate_context_percent(&hook);

    let cwd = hook.cwd.as_deref().unwrap_or(".");
    let (branch, file_changes, line_changes) = vcs::get_vcs_info(cwd);

    render::print_status_line(
        version,
        cwd,
        context_percent,
        &branch,
        &file_changes,
        &line_changes,
    );
}

fn handle_cli(args: &[String]) {
    match args[0].as_str() {
        "--list-themes" | "-l" => theme::list_themes(),
        "--set-theme" | "-t" => {
            if args.len() < 2 {
                eprintln!("Usage: cc-quadstat --set-theme <theme>");
                std::process::exit(1);
            }
            theme::set_theme(&args[1]);
        }
        "--init" => init_config(),
        "--help" | "-h" => print_help(),
        other => {
            eprintln!("Unknown option: {}", other);
            print_help();
            std::process::exit(1);
        }
    }
}

fn calculate_context_percent(hook: &HookInput) -> i32 {
    let remaining = hook
        .context_window
        .as_ref()
        .and_then(|cw| cw.remaining_percentage)
        .unwrap_or(100);

    remaining.clamp(0, 100) as i32
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

fn init_config() {
    let home = home_dir();
    let settings_path = home.join(".claude").join("settings.json");
    let binary_path = home.join(".claude").join("scripts").join("cc-quadstat");

    if !binary_path.exists() {
        eprintln!("cc-quadstat binary not found at {}", binary_path.display());
        eprintln!("Please copy the binary first:");
        eprintln!("  mkdir -p ~/.claude/scripts && cp cc-quadstat {}", binary_path.display());
        std::process::exit(1);
    }

    let mut settings: serde_json::Value = if let Ok(data) = fs::read_to_string(&settings_path) {
        match serde_json::from_str(&data) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse settings.json: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        serde_json::json!({})
    };

    settings["statusLine"] = serde_json::json!({
        "type": "command",
        "command": binary_path.to_string_lossy(),
    });

    let output = match serde_json::to_string_pretty(&settings) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to serialize settings: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = fs::write(&settings_path, output) {
        eprintln!("Failed to write settings.json: {}", e);
        std::process::exit(1);
    }

    println!("Claude Code settings updated");
    println!("  statusLine command: {}", binary_path.display());
    println!();
    println!("Restart Claude Code to apply changes");
}

fn print_help() {
    println!("cc-quadstat - Status line for Claude Code");
    println!();
    println!("Usage:");
    println!("  cc-quadstat                    Run as statusLine (reads JSON from stdin)");
    println!("  cc-quadstat --list-themes      List available themes");
    println!("  cc-quadstat --set-theme <name> Set theme");
    println!("  cc-quadstat --init             Initialize Claude Code settings");
    println!("  cc-quadstat --help             Show this help");
}
