//! System tray integration for Cygnus Drcom UI.

use std::sync::mpsc;
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayMessage {
    ToggleVisibility,
    Quit,
}

pub struct TrayHandle {
    pub rx: mpsc::Receiver<TrayMessage>,
    _tray: TrayIcon,
}

impl TrayHandle {
    pub fn new() -> Self {
        let icon = create_tray_icon();

        let menu = Menu::new();
        let show_hide_item = MenuItem::new("Show/Hide", true, None);
        let quit_item = MenuItem::new("Quit", true, None);
        let _ = menu.append_items(&[
            &show_hide_item,
            &PredefinedMenuItem::separator(),
            &quit_item,
        ]);

        let show_hide_id = show_hide_item.id().clone();
        let quit_id = quit_item.id().clone();

        let (tx, rx) = mpsc::channel::<TrayMessage>();
        MenuEvent::set_event_handler(Some(move |event: tray_icon::menu::MenuEvent| {
            let msg = if event.id == show_hide_id {
                TrayMessage::ToggleVisibility
            } else if event.id == quit_id {
                TrayMessage::Quit
            } else {
                return;
            };
            let _ = tx.send(msg);
        }));

        let _tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Cygnus Drcom")
            .with_icon(icon)
            .build()
            .expect("Failed to create tray icon");

        Self { rx, _tray }
    }

    /// Returns a clone of the tray icon to keep it alive after the handle drops.
    pub fn keep_alive(&self) -> TrayIcon {
        self._tray.clone()
    }
}

fn create_tray_icon() -> Icon {
    // Try to load the PNG icon from assets, fall back to solid blue
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("icon")
        .join("jlud.png");

    if let Ok(img) = image::open(&path) {
        let img = img.into_rgba8();
        let (width, height) = img.dimensions();
        let rgba = img.into_raw();
        match Icon::from_rgba(rgba, width, height) {
            Ok(icon) => return icon,
            Err(e) => {
                eprintln!("Failed to create tray icon from PNG: {}", e);
            }
        }
    }

    // Fallback: solid blue square
    let width: u32 = 32;
    let height: u32 = 32;
    let pixel_count = (width * height) as usize;
    let mut rgba = Vec::with_capacity(pixel_count * 4);
    for _ in 0..pixel_count {
        rgba.extend_from_slice(&[37, 99, 235, 255]);
    }
    Icon::from_rgba(rgba, width, height).expect("Failed to create tray icon from RGBA data")
}
