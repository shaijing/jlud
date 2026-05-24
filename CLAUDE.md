# JLUD - Iced Counter App

## Overview

Simple counter UI built with iced 0.14 framework (Rust 2024 edition).

## Running

```bash
cargo run
```

## Project Structure

```
jlud/
├── Cargo.toml
└── src/
    └── main.rs
```

## Tech Stack

- **iced 0.14** - Cross-platform GUI framework
- **iced_widget 0.14** - Built-in widgets
- **iced_graphics 0.14** - 2D graphics renderer
- **iced_tiny_skia 0.14** - Tiny Skia renderer backend
- Rust 2024 edition

## Code Pattern

```rust
use iced::widget::{button, column, text, row};
use iced::Alignment;
use iced::Application;

pub fn main() -> iced::Result {
    iced::application(|| initial_state, update, view)
        .title("App Title")
        .run()
}
```

## Key Widgets

- `column!` / `column()` - Vertical layout
- `row!` / `row()` - Horizontal layout
- `button()` - Clickable button
- `text!()` - Text display
- `text_input()` - Text input field
- `slider()` - Numeric slider