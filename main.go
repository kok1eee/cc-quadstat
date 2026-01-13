package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"time"

	"github.com/99designs/keyring"
	"golang.org/x/term"
)

const (
	Reset = "\033[0m"
	Bold  = "\033[1m"
	Sep   = "\uE0B0"
)

func fg256(code int) string { return fmt.Sprintf("\033[38;5;%dm", code) }
func bg256(code int) string { return fmt.Sprintf("\033[48;5;%dm", code) }

type Theme struct {
	Name         string
	Model        [2]int
	Version      [2]int
	Branch       [2]int
	CtxGood      [2]int
	CtxWarn      [2]int
	CtxBad       [2]int
	UsageGood    [2]int
	UsageWarn    [2]int
	UsageBad     [2]int
	UsageUnknown [2]int
}

var themes = map[string]Theme{
	"tokyo-night": {
		Name:         "Tokyo Night",
		Model:        [2]int{15, 57},
		Version:      [2]int{189, 60},
		Branch:       [2]int{0, 179},
		CtxGood:      [2]int{0, 78},
		CtxWarn:      [2]int{0, 214},
		CtxBad:       [2]int{15, 197},
		UsageGood:    [2]int{0, 114},
		UsageWarn:    [2]int{0, 221},
		UsageBad:     [2]int{15, 203},
		UsageUnknown: [2]int{15, 60},
	},
	"nord": {
		Name:         "Nord",
		Model:        [2]int{0, 110},
		Version:      [2]int{254, 60},
		Branch:       [2]int{0, 179},
		CtxGood:      [2]int{0, 108},
		CtxWarn:      [2]int{0, 222},
		CtxBad:       [2]int{15, 167},
		UsageGood:    [2]int{0, 150},
		UsageWarn:    [2]int{0, 180},
		UsageBad:     [2]int{15, 131},
		UsageUnknown: [2]int{15, 60},
	},
	"dracula": {
		Name:         "Dracula",
		Model:        [2]int{15, 141},
		Version:      [2]int{231, 61},
		Branch:       [2]int{0, 228},
		CtxGood:      [2]int{0, 84},
		CtxWarn:      [2]int{0, 215},
		CtxBad:       [2]int{15, 210},
		UsageGood:    [2]int{0, 120},
		UsageWarn:    [2]int{0, 222},
		UsageBad:     [2]int{15, 204},
		UsageUnknown: [2]int{15, 61},
	},
	"gruvbox": {
		Name:         "Gruvbox",
		Model:        [2]int{230, 66},
		Version:      [2]int{223, 239},
		Branch:       [2]int{235, 214},
		CtxGood:      [2]int{235, 142},
		CtxWarn:      [2]int{235, 208},
		CtxBad:       [2]int{230, 124},
		UsageGood:    [2]int{235, 106},
		UsageWarn:    [2]int{235, 172},
		UsageBad:     [2]int{230, 167},
		UsageUnknown: [2]int{230, 239},
	},
	"catppuccin": {
		Name:         "Catppuccin",
		Model:        [2]int{0, 183},
		Version:      [2]int{189, 60},
		Branch:       [2]int{0, 223},
		CtxGood:      [2]int{0, 158},
		CtxWarn:      [2]int{0, 223},
		CtxBad:       [2]int{15, 211},
		UsageGood:    [2]int{0, 158},
		UsageWarn:    [2]int{0, 223},
		UsageBad:     [2]int{15, 211},
		UsageUnknown: [2]int{15, 60},
	},
	"monokai": {
		Name:         "Monokai",
		Model:        [2]int{15, 197},
		Version:      [2]int{231, 239},
		Branch:       [2]int{0, 186},
		CtxGood:      [2]int{0, 148},
		CtxWarn:      [2]int{0, 208},
		CtxBad:       [2]int{15, 196},
		UsageGood:    [2]int{0, 81},
		UsageWarn:    [2]int{0, 208},
		UsageBad:     [2]int{15, 196},
		UsageUnknown: [2]int{15, 239},
	},
	"solarized": {
		Name:         "Solarized",
		Model:        [2]int{230, 37},
		Version:      [2]int{230, 240},
		Branch:       [2]int{235, 136},
		CtxGood:      [2]int{230, 64},
		CtxWarn:      [2]int{235, 166},
		CtxBad:       [2]int{230, 124},
		UsageGood:    [2]int{230, 33},
		UsageWarn:    [2]int{235, 166},
		UsageBad:     [2]int{230, 160},
		UsageUnknown: [2]int{230, 240},
	},
	"default": {
		Name:         "Default",
		Model:        [2]int{0, 44},
		Version:      [2]int{0, 242},
		Branch:       [2]int{0, 178},
		CtxGood:      [2]int{0, 34},
		CtxWarn:      [2]int{0, 178},
		CtxBad:       [2]int{15, 160},
		UsageGood:    [2]int{0, 34},
		UsageWarn:    [2]int{0, 178},
		UsageBad:     [2]int{15, 160},
		UsageUnknown: [2]int{15, 25},
	},
}

func getTheme() Theme {
	// Priority: env var > config file > default
	name := os.Getenv("CC_THEME")
	if name == "" {
		name = getThemeFromConfig()
	}
	if t, ok := themes[name]; ok {
		return t
	}
	return themes["tokyo-night"]
}

type Segment struct {
	Text string
	Fg   int
	Bg   int
}

func renderPowerline(segments []Segment) string {
	var out strings.Builder

	for i, seg := range segments {
		out.WriteString(fg256(seg.Fg) + bg256(seg.Bg) + Bold + seg.Text + Reset)

		if i < len(segments)-1 {
			nextBg := segments[i+1].Bg
			out.WriteString(fg256(seg.Bg) + bg256(nextBg) + Sep + Reset)
		} else {
			out.WriteString(fg256(seg.Bg) + Sep + Reset)
		}
	}

	return out.String()
}

type HookInput struct {
	SessionID string `json:"session_id"`
	Model     struct {
		ID          string `json:"id"`
		DisplayName string `json:"display_name"`
	} `json:"model"`
	Version           string `json:"version"`
	Cwd               string `json:"cwd"`
	Exceeds200kTokens bool   `json:"exceeds_200k_tokens"`
	ContextWindow     struct {
		ContextWindowSize   int `json:"context_window_size"`
		TotalInputTokens    int `json:"total_input_tokens"`
		RemainingPercentage int `json:"remaining_percentage"`
		CurrentUsage        struct {
			InputTokens              int `json:"input_tokens"`
			OutputTokens             int `json:"output_tokens"`
			CacheCreationInputTokens int `json:"cache_creation_input_tokens"`
			CacheReadInputTokens     int `json:"cache_read_input_tokens"`
		} `json:"current_usage"`
	} `json:"context_window"`
}

type UsageResponse struct {
	FiveHour struct {
		Utilization float64 `json:"utilization"`
		ResetsAt    string  `json:"resets_at"`
	} `json:"five_hour"`
	SevenDay struct {
		Utilization float64 `json:"utilization"`
		ResetsAt    string  `json:"resets_at"`
	} `json:"seven_day"`
	SevenDaySonnet *struct {
		Utilization float64 `json:"utilization"`
		ResetsAt    string  `json:"resets_at"`
	} `json:"seven_day_sonnet"`
}

type KeychainCreds struct {
	ClaudeAiOauth struct {
		AccessToken string `json:"accessToken"`
	} `json:"claudeAiOauth"`
}

func main() {
	if len(os.Args) > 1 {
		handleCLI()
		return
	}

	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		fmt.Println("❌ Error reading stdin")
		os.Exit(1)
	}

	var hook HookInput
	if err := json.Unmarshal(input, &hook); err != nil {
		fmt.Println("❌ Error parsing JSON")
		os.Exit(1)
	}

	modelName := hook.Model.DisplayName
	if modelName == "" {
		modelName = hook.Model.ID
	}
	if modelName == "" {
		modelName = "Unknown"
	}

	contextPercent := calculateContextPercent(hook)
	branch, changes := getVCSInfo(hook.Cwd)
	usage := getUsageLimits()

	printStatusLine(modelName, hook.Version, contextPercent, branch, changes, usage)
}

func handleCLI() {
	args := os.Args[1:]

	switch args[0] {
	case "--list-themes", "-l":
		listThemes()
	case "--set-theme", "-t":
		if len(args) < 2 {
			fmt.Println("Usage: cc-quadstat --set-theme <theme>")
			os.Exit(1)
		}
		setTheme(args[1])
	case "--init":
		initConfig()
	case "--help", "-h":
		printHelp()
	default:
		fmt.Printf("Unknown option: %s\n", args[0])
		printHelp()
		os.Exit(1)
	}
}

func getConfigDir() string {
	home, _ := os.UserHomeDir()
	return filepath.Join(home, ".config", "cc-quadstat")
}

func getThemeFile() string {
	return filepath.Join(getConfigDir(), "theme")
}

func listThemes() {
	currentTheme := getThemeFromConfig()
	fmt.Println("Available themes:")
	for name, theme := range themes {
		marker := "  "
		if name == currentTheme {
			marker = "* "
		}
		fmt.Printf("%s%s - %s\n", marker, name, theme.Name)
	}
}

func setTheme(name string) {
	if _, ok := themes[name]; !ok {
		fmt.Printf("Unknown theme: %s\n", name)
		fmt.Println("Use --list-themes to see available themes")
		os.Exit(1)
	}

	configDir := getConfigDir()
	if err := os.MkdirAll(configDir, 0755); err != nil {
		fmt.Printf("Failed to create config directory: %v\n", err)
		os.Exit(1)
	}

	if err := os.WriteFile(getThemeFile(), []byte(name), 0644); err != nil {
		fmt.Printf("Failed to save theme: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("Theme set to: %s\n", name)
}

func getThemeFromConfig() string {
	data, err := os.ReadFile(getThemeFile())
	if err != nil {
		return "tokyo-night"
	}
	return strings.TrimSpace(string(data))
}

func initConfig() {
	home, _ := os.UserHomeDir()
	settingsPath := filepath.Join(home, ".claude", "settings.json")

	// Check if cc-quadstat binary exists
	binaryPath := filepath.Join(home, ".claude", "scripts", "cc-quadstat")
	if _, err := os.Stat(binaryPath); os.IsNotExist(err) {
		fmt.Printf("cc-quadstat binary not found at %s\n", binaryPath)
		fmt.Println("Please copy the binary first:")
		fmt.Printf("  mkdir -p ~/.claude/scripts && cp cc-quadstat %s\n", binaryPath)
		os.Exit(1)
	}

	// Read existing settings
	var settings map[string]interface{}
	data, err := os.ReadFile(settingsPath)
	if err != nil {
		settings = make(map[string]interface{})
	} else {
		if err := json.Unmarshal(data, &settings); err != nil {
			fmt.Printf("Failed to parse settings.json: %v\n", err)
			os.Exit(1)
		}
	}

	// Update statusLine
	settings["statusLine"] = map[string]interface{}{
		"type":    "command",
		"command": binaryPath,
	}

	// Write back
	output, err := json.MarshalIndent(settings, "", "  ")
	if err != nil {
		fmt.Printf("Failed to serialize settings: %v\n", err)
		os.Exit(1)
	}

	if err := os.WriteFile(settingsPath, output, 0644); err != nil {
		fmt.Printf("Failed to write settings.json: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("✅ Claude Code settings updated")
	fmt.Printf("   statusLine command: %s\n", binaryPath)
	fmt.Println("\nRestart Claude Code to apply changes")
}

func printHelp() {
	fmt.Println("cc-quadstat - Status line for Claude Code")
	fmt.Println()
	fmt.Println("Usage:")
	fmt.Println("  cc-quadstat                    Run as statusLine (reads JSON from stdin)")
	fmt.Println("  cc-quadstat --list-themes      List available themes")
	fmt.Println("  cc-quadstat --set-theme <name> Set theme")
	fmt.Println("  cc-quadstat --init             Initialize Claude Code settings")
	fmt.Println("  cc-quadstat --help             Show this help")
}

func calculateContextPercent(hook HookInput) int {
	remaining := hook.ContextWindow.RemainingPercentage

	if remaining < 0 {
		remaining = 0
	}
	if remaining > 100 {
		remaining = 100
	}

	return remaining
}

type UsageLimits struct {
	FiveHour      int
	FiveHourReset string
	Weekly        int
	WeeklyReset   string
	Sonnet        int
	SonnetReset   string
}

func getUsageLimits() UsageLimits {
	result := UsageLimits{
		FiveHour: -1,
		Weekly:   -1,
		Sonnet:   -1,
	}

	creds := getKeychainCredentials()
	if creds == "" {
		return result
	}

	var keychainData KeychainCreds
	if err := json.Unmarshal([]byte(creds), &keychainData); err != nil {
		return result
	}

	token := keychainData.ClaudeAiOauth.AccessToken
	if token == "" {
		return result
	}

	cmd := exec.Command("curl", "-s", "--max-time", "3",
		"-H", "Authorization: Bearer "+token,
		"-H", "anthropic-beta: oauth-2025-04-20",
		"https://api.anthropic.com/api/oauth/usage")

	out, err := cmd.Output()
	if err != nil {
		return result
	}

	var usage UsageResponse
	if err := json.Unmarshal(out, &usage); err != nil {
		return result
	}

	result.FiveHour = int(usage.FiveHour.Utilization)
	if usage.FiveHour.ResetsAt != "" {
		result.FiveHourReset = calculateResetTime(usage.FiveHour.ResetsAt)
	}

	result.Weekly = int(usage.SevenDay.Utilization)
	if usage.SevenDay.ResetsAt != "" {
		result.WeeklyReset = formatResetDay(usage.SevenDay.ResetsAt)
	}

	if usage.SevenDaySonnet != nil {
		result.Sonnet = int(usage.SevenDaySonnet.Utilization)
		if usage.SevenDaySonnet.ResetsAt != "" {
			result.SonnetReset = formatResetDay(usage.SevenDaySonnet.ResetsAt)
		}
	}

	return result
}

func getKeychainCredentials() string {
	// Try cross-platform keyring first
	ring, err := keyring.Open(keyring.Config{
		ServiceName: "Claude Code-credentials",
		// Linux: use SecretService (GNOME Keyring / KWallet)
		// macOS: use Keychain
		// Windows: use Windows Credential Manager
	})
	if err == nil {
		item, err := ring.Get("credentials")
		if err == nil {
			return string(item.Data)
		}
	}

	// Fallback for macOS: try security command directly
	// (Claude Code stores credentials differently on macOS)
	if runtime.GOOS == "darwin" {
		cmd := exec.Command("security", "find-generic-password", "-s", "Claude Code-credentials", "-w")
		out, err := cmd.Output()
		if err != nil {
			return ""
		}
		return strings.TrimSpace(string(out))
	}

	return ""
}

func parseResetTime(resetsAt string) (time.Time, error) {
	resetTime, err := time.Parse(time.RFC3339, resetsAt)
	if err != nil {
		parts := strings.Split(resetsAt, ".")
		if len(parts) > 0 {
			resetTime, err = time.Parse("2006-01-02T15:04:05", parts[0])
		}
	}
	return resetTime, err
}

func calculateResetTime(resetsAt string) string {
	resetTime, err := parseResetTime(resetsAt)
	if err != nil {
		return ""
	}

	now := time.Now().UTC()
	diff := resetTime.Sub(now)

	if diff <= 0 {
		return "soon"
	}

	hours := int(diff.Hours())
	mins := int(diff.Minutes()) % 60

	if hours > 0 {
		return fmt.Sprintf("%dh%dm", hours, mins)
	}
	return fmt.Sprintf("%dm", mins)
}

func formatResetDay(resetsAt string) string {
	resetTime, err := parseResetTime(resetsAt)
	if err != nil {
		return ""
	}

	local := resetTime.In(time.Local)

	weekdays := []string{"日", "月", "火", "水", "木", "金", "土"}
	weekday := weekdays[local.Weekday()]

	return fmt.Sprintf("%s%d:%02d", weekday, local.Hour(), local.Minute())
}

func getVCSInfo(cwd string) (branch string, changes string) {
	if cwd == "" {
		cwd = "."
	}

	jjDir := filepath.Join(cwd, ".jj")
	if _, err := os.Stat(jjDir); err == nil {
		return getJJInfo(cwd)
	}

	gitDir := filepath.Join(cwd, ".git")
	if _, err := os.Stat(gitDir); err == nil {
		return getGitInfo(cwd)
	}

	return "", ""
}

func getJJInfo(cwd string) (string, string) {
	cmd := exec.Command("jj", "log", "-r", "@", "--no-graph", "-T", "if(bookmarks, bookmarks.join(\" \"), change_id.shortest())")
	cmd.Dir = cwd
	out, err := cmd.Output()
	if err != nil {
		return "@", ""
	}

	branch := strings.TrimSpace(strings.Split(string(out), "\n")[0])
	if branch == "" {
		branch = "@"
	}

	cmd = exec.Command("jj", "diff", "--stat")
	cmd.Dir = cwd
	out, err = cmd.Output()
	if err != nil {
		return branch, ""
	}

	lines := strings.Split(strings.TrimSpace(string(out)), "\n")
	if len(lines) == 0 {
		return branch, ""
	}

	return branch, parseStatLine(lines[len(lines)-1])
}

func getGitInfo(cwd string) (string, string) {
	cmd := exec.Command("git", "branch", "--show-current")
	cmd.Dir = cwd
	out, err := cmd.Output()
	if err != nil {
		return "detached", ""
	}

	branch := strings.TrimSpace(string(out))
	if branch == "" {
		branch = "detached"
	}

	cmd = exec.Command("git", "diff", "--stat")
	cmd.Dir = cwd
	out, err = cmd.Output()
	if err != nil {
		return branch, ""
	}

	lines := strings.Split(strings.TrimSpace(string(out)), "\n")
	if len(lines) == 0 {
		return branch, ""
	}

	return branch, parseStatLine(lines[len(lines)-1])
}

func parseStatLine(line string) string {
	if line == "" || !strings.Contains(line, "changed") {
		return ""
	}

	var insertions, deletions int
	parts := strings.Split(line, ",")
	for _, part := range parts {
		part = strings.TrimSpace(part)
		if strings.Contains(part, "insertion") {
			fmt.Sscanf(part, "%d", &insertions)
		} else if strings.Contains(part, "deletion") {
			fmt.Sscanf(part, "%d", &deletions)
		}
	}

	if insertions == 0 && deletions == 0 {
		return ""
	}

	return fmt.Sprintf("(+%d,-%d)", insertions, deletions)
}

func getContextColors(t Theme, percent int) (int, int) {
	if percent > 50 {
		return t.CtxGood[0], t.CtxGood[1]
	} else if percent > 20 {
		return t.CtxWarn[0], t.CtxWarn[1]
	}
	return t.CtxBad[0], t.CtxBad[1]
}

func getUsageColors(t Theme, percent int) (int, int) {
	if percent < 0 {
		return t.UsageUnknown[0], t.UsageUnknown[1]
	}
	if percent < 50 {
		return t.UsageGood[0], t.UsageGood[1]
	} else if percent < 80 {
		return t.UsageWarn[0], t.UsageWarn[1]
	}
	return t.UsageBad[0], t.UsageBad[1]
}

func getTerminalWidth() int {
	width, _, err := term.GetSize(int(os.Stdout.Fd()))
	if err != nil || width <= 0 {
		return 80 // default
	}
	return width
}

func visibleLength(s string) int {
	// Count visible characters (rough estimate for emoji and CJK)
	count := 0
	for _, r := range s {
		if r >= 0x1F300 && r <= 0x1FAFF { // Emoji range
			count += 2
		} else if r >= 0x3000 && r <= 0x9FFF { // CJK
			count += 2
		} else {
			count++
		}
	}
	return count
}

func segmentsWidth(segs []Segment) int {
	width := 0
	for _, seg := range segs {
		width += visibleLength(seg.Text)
	}
	// Add separator widths
	width += len(segs)
	return width
}

func printStatusLine(model, version string, contextPercent int, branch, changes string, usage UsageLimits) {
	t := getTheme()
	termWidth := getTerminalWidth()

	// Reserve space for Claude Code's right-side text
	reservedWidth := 40
	effectiveWidth := termWidth - reservedWidth
	if effectiveWidth < 40 {
		effectiveWidth = 40
	}

	// Line 1: Model + Version + Branch
	var line1Segs []Segment
	line1Segs = append(line1Segs, Segment{Text: " " + model + " ", Fg: t.Model[0], Bg: t.Model[1]})
	line1Segs = append(line1Segs, Segment{Text: " v" + version + " ", Fg: t.Version[0], Bg: t.Version[1]})

	if branch != "" {
		// Try with changes first
		branchTextFull := " ⎇ " + branch + " "
		if changes != "" {
			branchTextFull = " ⎇ " + branch + " " + changes + " "
		}
		branchSeg := Segment{Text: branchTextFull, Fg: t.Branch[0], Bg: t.Branch[1]}

		if segmentsWidth(append(line1Segs, branchSeg)) <= effectiveWidth {
			line1Segs = append(line1Segs, branchSeg)
		} else if changes != "" {
			// Try without changes
			branchTextShort := " ⎇ " + branch + " "
			branchSeg = Segment{Text: branchTextShort, Fg: t.Branch[0], Bg: t.Branch[1]}
			if segmentsWidth(append(line1Segs, branchSeg)) <= effectiveWidth {
				line1Segs = append(line1Segs, branchSeg)
			}
			// else: skip branch entirely
		}
	}
	fmt.Println(renderPowerline(line1Segs))

	// Line 2: Context + 5h usage
	var line2Segs []Segment
	ctxFg, ctxBg := getContextColors(t, contextPercent)
	line2Segs = append(line2Segs, Segment{
		Text: fmt.Sprintf(" 🧠 %d%% ", contextPercent),
		Fg:   ctxFg,
		Bg:   ctxBg,
	})

	if usage.FiveHour >= 0 {
		// Try with reset time first
		textFull := fmt.Sprintf(" ⏱ 5h: %d%% ", usage.FiveHour)
		if usage.FiveHourReset != "" {
			textFull = fmt.Sprintf(" ⏱ 5h: %d%% (%s) ", usage.FiveHour, usage.FiveHourReset)
		}
		fg, bg := getUsageColors(t, usage.FiveHour)
		fiveHourSeg := Segment{Text: textFull, Fg: fg, Bg: bg}

		if segmentsWidth(append(line2Segs, fiveHourSeg)) <= effectiveWidth {
			line2Segs = append(line2Segs, fiveHourSeg)
		} else {
			// Try without reset time
			textShort := fmt.Sprintf(" ⏱ 5h: %d%% ", usage.FiveHour)
			fiveHourSeg = Segment{Text: textShort, Fg: fg, Bg: bg}
			if segmentsWidth(append(line2Segs, fiveHourSeg)) <= effectiveWidth {
				line2Segs = append(line2Segs, fiveHourSeg)
			}
		}
	}
	fmt.Println(renderPowerline(line2Segs))

	// Line 3: Weekly All + Sonnet
	var line3Segs []Segment

	if usage.Weekly >= 0 {
		textFull := fmt.Sprintf(" 📅 All: %d%% ", usage.Weekly)
		if usage.WeeklyReset != "" {
			textFull = fmt.Sprintf(" 📅 All: %d%% (%s) ", usage.Weekly, usage.WeeklyReset)
		}
		fg, bg := getUsageColors(t, usage.Weekly)
		weeklySeg := Segment{Text: textFull, Fg: fg, Bg: bg}

		if segmentsWidth(append(line3Segs, weeklySeg)) <= effectiveWidth {
			line3Segs = append(line3Segs, weeklySeg)
		} else {
			// Try without reset time
			textShort := fmt.Sprintf(" 📅 All: %d%% ", usage.Weekly)
			weeklySeg = Segment{Text: textShort, Fg: fg, Bg: bg}
			line3Segs = append(line3Segs, weeklySeg)
		}
	}

	if usage.Sonnet >= 0 {
		textFull := fmt.Sprintf(" 💬 Sonnet: %d%% ", usage.Sonnet)
		if usage.SonnetReset != "" {
			textFull = fmt.Sprintf(" 💬 Sonnet: %d%% (%s) ", usage.Sonnet, usage.SonnetReset)
		}
		fg, bg := getUsageColors(t, usage.Sonnet)
		sonnetSeg := Segment{Text: textFull, Fg: fg, Bg: bg}

		if segmentsWidth(append(line3Segs, sonnetSeg)) <= effectiveWidth {
			line3Segs = append(line3Segs, sonnetSeg)
		} else {
			// Try without reset time
			textShort := fmt.Sprintf(" 💬 Sonnet: %d%% ", usage.Sonnet)
			sonnetSeg = Segment{Text: textShort, Fg: fg, Bg: bg}
			if segmentsWidth(append(line3Segs, sonnetSeg)) <= effectiveWidth {
				line3Segs = append(line3Segs, sonnetSeg)
			}
			// else: skip sonnet entirely
		}
	}

	if len(line3Segs) > 0 {
		fmt.Println(renderPowerline(line3Segs))
	}
}
