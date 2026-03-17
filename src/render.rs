use crate::types::Segment;
use crate::theme::get_theme;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const SEP: &str = "\u{E0B0}";

fn fg256(code: u8) -> String {
    format!("\x1b[38;5;{}m", code)
}

fn bg256(code: u8) -> String {
    format!("\x1b[48;5;{}m", code)
}

fn render_powerline(segments: &[Segment]) -> String {
    let mut out = String::new();

    for (i, seg) in segments.iter().enumerate() {
        out.push_str(&fg256(seg.fg));
        out.push_str(&bg256(seg.bg));
        out.push_str(BOLD);
        out.push_str(&seg.text);
        out.push_str(RESET);

        if i < segments.len() - 1 {
            let next_bg = segments[i + 1].bg;
            out.push_str(&fg256(seg.bg));
            out.push_str(&bg256(next_bg));
            out.push_str(SEP);
            out.push_str(RESET);
        } else {
            out.push_str(&fg256(seg.bg));
            out.push_str(SEP);
            out.push_str(RESET);
        }
    }

    out
}

pub fn print_status_line(
    model_name: &str,
    version: &str,
    cwd: &str,
    context_percent: i32,
    branch: &str,
    file_changes: &str,
    line_changes: &str,
) {
    let t = get_theme();

    let (ctx_fg, ctx_bg) = if context_percent > 50 {
        (t.ctx_good[0], t.ctx_good[1])
    } else if context_percent > 20 {
        (t.ctx_warn[0], t.ctx_warn[1])
    } else {
        (t.ctx_bad[0], t.ctx_bad[1])
    };

    let dir_name = std::path::Path::new(cwd)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(cwd);

    // Line 1: model, version, context
    let line1 = vec![
        Segment { text: format!(" {} ", model_name), fg: t.model[0], bg: t.model[1] },
        Segment { text: format!(" v{} ", version), fg: t.version[0], bg: t.version[1] },
        Segment { text: format!(" \u{1F9E0} {}% ", context_percent), fg: ctx_fg, bg: ctx_bg },
    ];

    // Line 2: dir, branch, changes
    let mut line2 = vec![
        Segment { text: format!(" \u{1F4C2} {} ", dir_name), fg: t.version[0], bg: t.version[1] },
    ];

    if !branch.is_empty() {
        let mut info_parts = vec![branch.to_string()];
        if !file_changes.is_empty() {
            info_parts.push(file_changes.to_string());
        }
        if !line_changes.is_empty() {
            info_parts.push(format!("({})", line_changes));
        }
        line2.push(Segment {
            text: format!(" \u{238B} {} ", info_parts.join(" ")),
            fg: t.branch[0], bg: t.branch[1],
        });
    }

    println!("{}", render_powerline(&line1));
    println!("{}", render_powerline(&line2));
}
