# 🦥 Boomodoro

A minimal TUI pomodoro timer built with Rust, made for focus.

![ss](https://github.com/user-attachments/assets/575e2cc2-f406-41d4-aa8d-cc5b9e5a2d23)

## Features

- 25/5/15 minute work and break cycles
- Desktop notifications on stage change
- Sound effects on key actions
- Session progress dots
- Single clean interface

## Keybindings

| Key | Action |
|-----|--------|
| `p` | Start / Stop |
| `j` | Skip to next stage |
| `r` | Reset |
| `q` | Quit |

## Installation

Download the latest binary from [Releases](../../releases) and run:

```bash
chmod +x boomodoro
./boomodoro
```

Or add it to your PATH:

```bash
sudo cp boomodoro /usr/local/bin/
```

## Building from source

```bash
git clone git@github.com:yourusername/boomodoro.git
cd boomodoro
cargo build --release
./target/release/boomodoro
```

## Windows Support

Use exe from release.

## Built with

- [ratatui](https://github.com/ratatui-org/ratatui) — TUI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — terminal control
- [rodio](https://github.com/RustAudio/rodio) — audio
- [notify-rust](https://github.com/hoodie/notify-rust) — desktop notifications
