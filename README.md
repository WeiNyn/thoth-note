# Thoth 📝

A powerful terminal-based note-taking application built in Rust with full Markdown support, live preview, and syntax highlighting.

## 🔮 Why Thoth?

Thoth was the ancient Egyptian deity of wisdom, writing, hieroglyphs, science, and magic. As the divine scribe, he was said to have invented writing, making him the natural namesake for this powerful note-taking application. Just as Thoth was the keeper of divine records and cosmic knowledge, this application serves as your personal scribe in the digital age.

## ✨ Features

- **Live Markdown Preview**: See your formatting changes in real-time
- **Syntax Highlighting**: Support for multiple programming languages in code blocks
- **Three-Panel Interface**:
  - Note List (left): Browse and manage your notes
  - Editor (center): Write and edit content
  - Preview (right): See rendered Markdown
- **File Management**: Create, edit, delete, and reorder notes
- **Keyboard-Centric Navigation**: Fast and efficient workflows
- **Theme Support**: Beautiful Catppuccin theme integration
- **Full Markdown Support**: 
  - Headers, lists, and tables
  - Code blocks with syntax highlighting
  - Text formatting (bold, italic, strikethrough)
  - Links and images
  - Blockquotes

## 🚀 Installation

### Prerequisites

- Rust toolchain (1.70.0 or later)

### Building from Source

```bash
git clone https://github.com/WeiNyn/thoth
cd thoth
cargo build --release
```

The binary will be available at `target/release/thoth`

## 🎮 Usage

1. Launch Thoth:
   ```bash
   ./thoth
   ```
2. Create your first note with `Ctrl+N`
3. Start writing in Markdown
4. Toggle live preview with `Ctrl+L`

## ⌨️ Key Bindings

| Shortcut | Action |
|----------|--------|
| `Ctrl+L` | Toggle Live Preview |
| `Ctrl+E` | Switch to Editor |
| `Ctrl+P` | Switch to Preview |
| `Ctrl+N` | Create new note |
| `Ctrl+S` | Save current note |
| `Ctrl+D` | Delete current note |
| `Ctrl+R` | Rename note |
| `Ctrl+Up/Down` | Navigate between notes |
| `Alt+Up/Down` | Reorder notes |
| `Ctrl+J/K` | Scroll preview |
| `Ctrl+Q` | Quit application |

## 🛠️ Dependencies

- [ratatui](https://crates.io/crates/ratatui) - Terminal UI framework
- [crossterm](https://crates.io/crates/crossterm) - Terminal manipulation
- [edtui](https://crates.io/crates/edtui) - Text editor widget
- [pulldown-cmark](https://crates.io/crates/pulldown-cmark) - Markdown parsing
- [syntect](https://crates.io/crates/syntect) - Syntax highlighting
- [catppuccin](https://crates.io/crates/catppuccin) - Theme support

## 🗺️ Roadmap

- [ ] Search functionality
- [ ] Tags and categories
- [ ] Export to different formats
- [ ] Custom themes
- [ ] Vim keybindings

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

- [@WeiNyn](https://github.com/WeiNyn)

## ☕ Support

If you find Thoth useful, you can [buy me a coffee](https://buymeacoffee.com/weinyn)!
