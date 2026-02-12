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

fn format_tokens(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{}M", n / 1_000_000)
    } else if n >= 1_000 {
        format!("{}k", n / 1_000)
    } else {
        format!("{}", n)
    }
}

pub fn print_status_line(
    model: &str,
    version: &str,
    context_percent: i32,
    branch: &str,
    file_changes: &str,
    total_tokens: i64,
    context_window_size: i64,
) {
    let t = get_theme();
    let term_width = get_terminal_width();

    let reserved_width = 40;
    let effective_width = if term_width > reserved_width + 40 {
        term_width - reserved_width
    } else {
        40
    };

    // Line 1: Model + Version + Branch (file changes)
    let mut line1 = vec![
        Segment { text: format!(" {} ", model), fg: t.model[0], bg: t.model[1] },
        Segment { text: format!(" v{} ", version), fg: t.version[0], bg: t.version[1] },
    ];

    if !branch.is_empty() {
        let branch_text_full = if !file_changes.is_empty() {
            format!(" \u{238B} {} {} ", branch, file_changes)
        } else {
            format!(" \u{238B} {} ", branch)
        };
        let branch_seg = Segment { text: branch_text_full.clone(), fg: t.branch[0], bg: t.branch[1] };

        let mut test = line1.clone_refs();
        test.push(&branch_seg);
        if segments_width_refs(&test) <= effective_width {
            line1.push(Segment { text: branch_text_full, fg: t.branch[0], bg: t.branch[1] });
        } else if !file_changes.is_empty() {
            let branch_text_short = format!(" \u{238B} {} ", branch);
            let short_seg = Segment { text: branch_text_short.clone(), fg: t.branch[0], bg: t.branch[1] };
            let mut test2 = line1.clone_refs();
            test2.push(&short_seg);
            if segments_width_refs(&test2) <= effective_width {
                line1.push(Segment { text: branch_text_short, fg: t.branch[0], bg: t.branch[1] });
            }
        }
    }

    println!("{}", render_powerline(&line1));

    // Line 2: Context + Tokens
    let (ctx_fg, ctx_bg) = get_context_colors(&t, context_percent);
    let mut line2 = vec![
        Segment {
            text: format!(" \u{1F9E0} {}% ", context_percent),
            fg: ctx_fg,
            bg: ctx_bg,
        },
    ];

    if total_tokens > 0 && context_window_size > 0 {
        let token_text = format!(" \u{1F4CA} {}/{} ", format_tokens(total_tokens), format_tokens(context_window_size));
        let token_seg = Segment { text: token_text.clone(), fg: t.version[0], bg: t.version[1] };
        let mut test = line2.clone_refs();
        test.push(&token_seg);
        if segments_width_refs(&test) <= effective_width {
            line2.push(Segment { text: token_text, fg: t.version[0], bg: t.version[1] });
        }
    }

    println!("{}", render_powerline(&line2));
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
