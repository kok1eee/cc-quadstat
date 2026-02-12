use serde::Deserialize;

#[derive(Deserialize)]
pub struct HookInput {
    #[allow(dead_code)]
    pub session_id: Option<String>,
    #[allow(dead_code)]
    pub model: ModelInfo,
    pub version: Option<String>,
    pub cwd: Option<String>,
    #[allow(dead_code)]
    pub exceeds_200k_tokens: Option<bool>,
    pub context_window: Option<ContextWindow>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ModelInfo {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ContextWindow {
    #[allow(dead_code)]
    pub context_window_size: Option<i64>,
    #[allow(dead_code)]
    pub total_input_tokens: Option<i64>,
    pub remaining_percentage: Option<i64>,
    #[allow(dead_code)]
    pub current_usage: Option<CurrentUsage>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct CurrentUsage {
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub cache_creation_input_tokens: Option<i64>,
    pub cache_read_input_tokens: Option<i64>,
}

pub struct Segment {
    pub text: String,
    pub fg: u8,
    pub bg: u8,
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    #[allow(dead_code)]
    pub model: [u8; 2],
    pub version: [u8; 2],
    pub branch: [u8; 2],
    pub ctx_good: [u8; 2],
    pub ctx_warn: [u8; 2],
    pub ctx_bad: [u8; 2],
}
