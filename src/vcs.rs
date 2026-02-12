use std::path::Path;
use std::process::Command;

/// Returns (branch, file_changes, line_changes)
pub fn get_vcs_info(cwd: &str) -> (String, String, String) {
    let dir = if cwd.is_empty() { "." } else { cwd };

    if Path::new(dir).join(".jj").exists() {
        return get_jj_info(dir);
    }
    if Path::new(dir).join(".git").exists() {
        return get_git_info(dir);
    }

    (String::new(), String::new(), String::new())
}

fn get_jj_info(cwd: &str) -> (String, String, String) {
    let branch = Command::new("jj")
        .args(["log", "-r", "@", "--no-graph", "-T",
               "if(bookmarks, bookmarks.join(\" \"), change_id.shortest())"])
        .current_dir(cwd)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout).to_string();
                let first = s.lines().next().unwrap_or("").trim().to_string();
                if first.is_empty() { None } else { Some(first) }
            } else {
                None
            }
        })
        .unwrap_or_else(|| "@".to_string());

    let diff_output = Command::new("jj")
        .args(["diff", "--summary"])
        .current_dir(cwd)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).to_string())
            } else {
                None
            }
        })
        .unwrap_or_default();

    let file_changes = count_file_changes(&diff_output);

    let line_changes = Command::new("jj")
        .args(["diff", "--stat"])
        .current_dir(cwd)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(parse_stat_summary(&String::from_utf8_lossy(&o.stdout)))
            } else {
                None
            }
        })
        .unwrap_or_default();

    (branch, file_changes, line_changes)
}

fn get_git_info(cwd: &str) -> (String, String, String) {
    let branch = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(cwd)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if s.is_empty() { None } else { Some(s) }
            } else {
                None
            }
        })
        .unwrap_or_else(|| "detached".to_string());

    let file_changes = Command::new("git")
        .args(["diff", "--name-status"])
        .current_dir(cwd)
        .output()
        .ok()
        .map(|o| {
            if o.status.success() {
                count_file_changes(&String::from_utf8_lossy(&o.stdout))
            } else {
                String::new()
            }
        })
        .unwrap_or_default();

    let line_changes = Command::new("git")
        .args(["diff", "--shortstat"])
        .current_dir(cwd)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(parse_stat_summary(&String::from_utf8_lossy(&o.stdout)))
            } else {
                None
            }
        })
        .unwrap_or_default();

    (branch, file_changes, line_changes)
}

/// Parse "X files changed, Y insertions(+), Z deletions(-)" into "+Y-Z"
fn parse_stat_summary(output: &str) -> String {
    let last_line = output.lines().last().unwrap_or("").trim();
    if last_line.is_empty() {
        return String::new();
    }

    let mut insertions = 0u32;
    let mut deletions = 0u32;

    for part in last_line.split(',') {
        let part = part.trim();
        if part.contains("insertion") {
            if let Some(n) = part.split_whitespace().next().and_then(|s| s.parse().ok()) {
                insertions = n;
            }
        } else if part.contains("deletion") {
            if let Some(n) = part.split_whitespace().next().and_then(|s| s.parse().ok()) {
                deletions = n;
            }
        }
    }

    if insertions == 0 && deletions == 0 {
        return String::new();
    }

    let mut parts = Vec::new();
    if insertions > 0 { parts.push(format!("+{}", insertions)); }
    if deletions > 0 { parts.push(format!("-{}", deletions)); }
    parts.join("")
}

fn count_file_changes(output: &str) -> String {
    let mut added = 0u32;
    let mut modified = 0u32;
    let mut deleted = 0u32;

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        match line.chars().next() {
            Some('A') => added += 1,
            Some('M') => modified += 1,
            Some('D') => deleted += 1,
            Some('R') => modified += 1,
            _ => {}
        }
    }

    if added == 0 && modified == 0 && deleted == 0 {
        return String::new();
    }

    let mut parts = Vec::new();
    if modified > 0 { parts.push(format!("~{}", modified)); }
    if added > 0 { parts.push(format!("+{}", added)); }
    if deleted > 0 { parts.push(format!("-{}", deleted)); }
    parts.join("")
}
