# 5Menu - Modern Application Launcher

A modern, customizable application launcher similar to Rofi, built with Rust and Iced GUI framework.

## Features

- **Fast Application Search**: Fuzzy search through installed applications
- **Mathematical Calculator**: Built-in calculator for simple math expressions
- **Customizable Themes**: Multiple themes with easy switching
- **Settings Interface**: In-app settings accessible via `> Settings` command
- **Keyboard Navigation**: Full keyboard control with arrow keys
- **Fixed Layout**: Consistent window size and layout

## Installation

1. Clone the repository:
```bash
git clone <https://github.com/destro1t/5Menu>
cd 5Menu
```

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --bin 5menu
```

## Usage

### Basic Usage
- **Launch**: Run the application to see all available programs
- **Search**: Start typing to filter applications
- **Navigate**: Use ↑/↓ arrow keys to select items
- **Execute**: Press Enter to launch selected application
- **Exit**: Press Escape to close

### Mathematical Calculator
Enter mathematical expressions directly:
- `1 + 1` → `Answer: 2`
- `10 - 5` → `Answer: 5`
- `3 * 4` → `Answer: 12`
- `15 / 3` → `Answer: 5`

### Settings
Access settings by typing `> Settings`:
- **Change Theme**: Select from available themes
- **View Current Settings**: See current configuration
- **Return**: Select "Back to Main" to return to application list

## Themes

### Available Themes

1. **default** - Dark theme with blue accents
2. **ketputin** - Red and gold theme with Russian-inspired colors
3. **matrix** - Green on black Matrix-style theme
4. **ocean** - Blue ocean-inspired theme

### Creating Custom Themes

Create a new `.toml` file in `config/themes/` directory:

```toml
name = "mytheme"
background_color = "#1a1b26"
text_color = "#a9b1d6"
selected_background_color = "#7aa2f7"
selected_text_color = "#1a1b26"
border_color = "#414868"
border_width = 2.0
border_radius = 8.0
padding = 12.0
```

## Configuration

The main configuration file is located at `~/.config/5menu/config.toml`:

```toml
theme = "default"
width = 900
height = 600
font_size = 14
max_entries = 15
terminal = "xterm"
search_paths = ["/usr/bin", "/usr/local/bin"]
hide_on_lose_focus = true
case_sensitive = false
```

### Configuration Options

- `theme`: Name of the theme to use
- `width/height`: Window dimensions in pixels
- `font_size`: Text font size
- `max_entries`: Maximum number of entries to display
- `terminal`: Default terminal emulator
- `search_paths`: Directories to scan for applications
- `hide_on_lose_focus`: Hide window when it loses focus
- `case_sensitive`: Enable case-sensitive search

## Keyboard Shortcuts

- `↑/↓`: Navigate through entries
- `Enter`: Execute selected item
- `Escape`: Exit application
- `Mouse Wheel`: Scroll through long lists

## Dependencies

- Rust 1.70+
- iced = "0.10"
- tokio = "1.32"
- serde = "1.0"
- dirs = "5.0"
- fuzzy-matcher = "0.3"

## Building from Source

```bash
git clone <repository-url>
cd 5Menu
cargo build --release
```

The binary will be available at `target/release/5menu`.

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]