# cc-dashboard

A Powerline-style statusline for Claude Code that displays real-time usage information.

![cc-dashboard screenshot](https://raw.githubusercontent.com/tazawa-masayoshi/cc-dashboard/main/assets/screenshot.png)

## Features

- **Powerline style** - Beautiful segmented display with background colors
- **8 color themes** - Tokyo Night, Dracula, Nord, Gruvbox, and more
- **Real-time usage** - 5-hour and weekly usage limits from Anthropic API
- **Context tracking** - Remaining context window percentage with color coding
- **VCS support** - Git and Jujutsu (jj) branch names with change stats
- **Adaptive width** - Automatically adjusts content based on terminal width
- **Cross-platform** - macOS and Linux support

## Display

```
 Opus 4.5  v2.1.6  ⎇ main (+42,-10)
 🧠 78%  ⏱ 5h: 3% (4h17m) 
 📅 All: 0% (火20:00)  💬 Sonnet: 0% 
```

| Line | Content |
|------|---------|
| 1 | Model name, Claude Code version, branch name, changes |
| 2 | Context window remaining %, 5-hour usage with reset time |
| 3 | Weekly usage (All models), Sonnet-only usage |

## Installation

### From Release (Recommended)

```bash
# Download latest release for your platform
# macOS (Apple Silicon)
curl -L https://github.com/tazawa-masayoshi/cc-dashboard/releases/latest/download/cc-dashboard_darwin_arm64.tar.gz | tar xz
# macOS (Intel)
curl -L https://github.com/tazawa-masayoshi/cc-dashboard/releases/latest/download/cc-dashboard_darwin_x86_64.tar.gz | tar xz
# Linux (x86_64)
curl -L https://github.com/tazawa-masayoshi/cc-dashboard/releases/latest/download/cc-dashboard_linux_x86_64.tar.gz | tar xz

# Move to scripts directory
mkdir -p ~/.claude/scripts
mv cc-dashboard ~/.claude/scripts/
chmod +x ~/.claude/scripts/cc-dashboard
```

### From Source

```bash
go install github.com/tazawa-masayoshi/cc-dashboard@latest
# or
git clone https://github.com/tazawa-masayoshi/cc-dashboard
cd cc-dashboard
go build -o cc-dashboard
cp cc-dashboard ~/.claude/scripts/
```

### Configuration

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "CC_THEME=dracula ~/.claude/scripts/cc-dashboard"
  }
}
```

## Themes

Set theme via `CC_THEME` environment variable:

| Theme | Description |
|-------|-------------|
| `tokyo-night` | Default. Cool blue/purple tones |
| `dracula` | Purple and green, high contrast |
| `nord` | Arctic, bluish color palette |
| `gruvbox` | Retro groove with warm colors |
| `catppuccin` | Pastel colors, easy on eyes |
| `monokai` | Classic dark theme |
| `solarized` | Precision colors for readability |
| `default` | Simple, minimal colors |

Example:
```bash
CC_THEME=gruvbox ~/.claude/scripts/cc-dashboard
```

## Color Coding

### Context Window (🧠)
| Remaining | Color |
|-----------|-------|
| > 50% | Green |
| 20-50% | Yellow |
| < 20% | Red |

### Usage Limits (⏱, 📅, 💬)
| Usage | Color |
|-------|-------|
| < 50% | Green |
| 50-80% | Yellow |
| > 80% | Red |

## Requirements

- Claude Code (with OAuth authentication)
- Powerline-compatible font (for segment separators)
- Git or Jujutsu (optional, for VCS info)

### Linux Requirements

For credential access on Linux, you need one of:
- GNOME Keyring (SecretService)
- KWallet
- pass (password-store)

## How It Works

1. Receives JSON input from Claude Code hook on stdin
2. Extracts model info, context usage from hook data
3. Fetches usage limits from Anthropic OAuth API
4. Retrieves credentials from system keychain
5. Renders Powerline-style output with ANSI colors

## License

MIT
