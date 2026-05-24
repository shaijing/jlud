#![cfg_attr(windows, windows_subsystem = "windows")]
// jlud - Drcom UI
// Main entry point

mod config;
mod cygnus;
mod ui;

fn main() -> iced::Result {
    crate::ui::run()
}
