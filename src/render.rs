use crate::types::{Segment, Theme};
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

fn visible_length(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        let cp = c as u32;
        if (0x1F300..=0x1FAFF).contains(&cp) {
            count += 2;
        } else if (0x3000..=0x9FFF).contains(&cp) {
            count += 2;
        } else {
            count += 1;
        }
    }
    count
}

fn get_context_colors(t: &Theme, percent: i32) -> (u8, u8) {
    if percent > 50 {
        (t.ctx_good[0], t.ctx_good[1])
    } else if percent > 20 {
        (t.ctx_warn[0], t.ctx_warn[1])
    } else {
        (t.ctx_bad[0], t.ctx_bad[1])
    }
}

fn get_terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}

pub fn print_status_line(
    version: &str,
    cwd: &str,
    context_percent: i32,
    branch: &str,
    file_changes: &str,
    line_changes: &str,
) {
    let t = get_theme();
    let term_width = get_terminal_width();

    let reserved_width = 40;
    let effective_width = if term_width > reserved_width + 40 {
        term_width - reserved_width
    } else {
        40
    };

    // Line 1: Dir + Version
    let dir_name = std::path::Path::new(cwd)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(cwd);

    let line1 = vec![
        Segment { text: format!(" \u{1F4C2} {} ", dir_name), fg: t.model[0], bg: t.model[1] },
        Segment { text: format!(" v{} ", version), fg: t.version[0], bg: t.version[1] },
    ];

    println!("{}", render_powerline(&line1));

    // Line 2: Branch + File changes + Line changes + Context
    let (ctx_fg, ctx_bg) = get_context_colors(&t, context_percent);

    if !branch.is_empty() {
        let mut info_parts = vec![branch.to_string()];
        if !file_changes.is_empty() {
            info_parts.push(file_changes.to_string());
        }
        if !line_changes.is_empty() {
            info_parts.push(format!("({})", line_changes));
        }

        let branch_text = format!(" \u{238B} {} ", info_parts.join(" "));

        let mut line2 = vec![
            Segment { text: branch_text, fg: t.branch[0], bg: t.branch[1] },
            Segment { text: format!(" \u{1F9E0} {}% ", context_percent), fg: ctx_fg, bg: ctx_bg },
        ];

        // 幅に収まらない場合、段階的に省略
        if segments_width_refs(&line2.clone_refs()) > effective_width {
            let mut short_parts = vec![branch.to_string()];
            if !file_changes.is_empty() {
                short_parts.push(file_changes.to_string());
            }
            line2 = vec![
                Segment { text: format!(" \u{238B} {} ", short_parts.join(" ")), fg: t.branch[0], bg: t.branch[1] },
                Segment { text: format!(" \u{1F9E0} {}% ", context_percent), fg: ctx_fg, bg: ctx_bg },
            ];
        }
        if segments_width_refs(&line2.clone_refs()) > effective_width {
            line2 = vec![
                Segment { text: format!(" \u{238B} {} ", branch), fg: t.branch[0], bg: t.branch[1] },
                Segment { text: format!(" \u{1F9E0} {}% ", context_percent), fg: ctx_fg, bg: ctx_bg },
            ];
        }

        println!("{}", render_powerline(&line2));
    } else {
        let line2 = vec![
            Segment { text: format!(" \u{1F9E0} {}% ", context_percent), fg: ctx_fg, bg: ctx_bg },
        ];
        println!("{}", render_powerline(&line2));
    }
}

trait CloneRefs {
    fn clone_refs(&self) -> Vec<&Segment>;
}

impl CloneRefs for Vec<Segment> {
    fn clone_refs(&self) -> Vec<&Segment> {
        self.iter().collect()
    }
}

fn segments_width_refs(segs: &[&Segment]) -> usize {
    let mut width: usize = 0;
    for seg in segs {
        width += visible_length(&seg.text);
    }
    width += segs.len();
    width
}
