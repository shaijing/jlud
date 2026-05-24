//! OKLCH color palette — perceptually uniform, harmonious.
//!
//! All values pre-computed from OKLCH → sRGB to avoid const-fn limitations.
//! Palette revolves around hue 265° (cool indigo-blue) with status hues.

use iced_widget::container;

// ── Brand ────────────────────────────────────────────────────────────
// Hue 265° → cool indigo-blue  (professional, calm)
// Status hues: green 145°, red 26°, yellow 90°

/// Sidebar background (very dark navy).
pub const SIDEBAR_BG: iced::Color = iced::Color::from_rgb(0.044929, 0.067721, 0.117751);
/// Sidebar nav hover.
pub const SIDEBAR_HOVER: iced::Color = iced::Color::from_rgb(0.101749, 0.139102, 0.221434);
/// Sidebar nav active / selected.
pub const SIDEBAR_ACTIVE: iced::Color = iced::Color::from_rgb(0.147267, 0.204902, 0.334407);

// ── Surface ──────────────────────────────────────────────────────────

/// Content area background — very light, subtle cool tint.
pub const CONTENT_BG: iced::Color = iced::Color::from_rgb(0.954305, 0.960989, 0.974682);
/// Card / panel surface.
pub const CARD_BG: iced::Color = iced::Color::from_rgb(1.0, 1.0, 1.0);

// ── Text ─────────────────────────────────────────────────────────────

pub const TEXT_PRIMARY: iced::Color = iced::Color::from_rgb(0.019219, 0.022434, 0.030048);
pub const TEXT_SECONDARY: iced::Color = iced::Color::from_rgb(0.249781, 0.260256, 0.281823);

// ── Accent ───────────────────────────────────────────────────────────

pub const PRIMARY_BLUE: iced::Color = iced::Color::from_rgb(0.234780, 0.384397, 0.756736);
pub const PRIMARY_BLUE_HOVER: iced::Color = iced::Color::from_rgb(0.193187, 0.336819, 0.704655);

// ── Status ───────────────────────────────────────────────────────────

pub const CONNECTED_GREEN: iced::Color = iced::Color::from_rgb(0.173017, 0.527634, 0.211100);
pub const ERROR_RED: iced::Color = iced::Color::from_rgb(0.764259, 0.224564, 0.213536);
pub const ERROR_RED_HOVER: iced::Color = iced::Color::from_rgb(0.695688, 0.152054, 0.155865);
pub const WARN_YELLOW: iced::Color = iced::Color::from_rgb(0.764510, 0.629357, 0.226361);

/// Dark terminal background for live logs.
pub const LOG_BG: iced::Color = iced::Color::from_rgb(0.026579, 0.031936, 0.044763);
/// Light text on dark log background.
pub const LOG_TEXT: iced::Color = iced::Color::from_rgb(0.750110, 0.769224, 0.808493);

/// Translucent blue overlay for secondary-button hover.
pub const BLUE_SOFT: iced::Color = iced::Color::from_rgb(0.182955, 0.324967, 0.691711);

// ── Sidebar Styles ───────────────────────────────────────────────────

pub fn sidebar_style() -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(iced::Background::Color(SIDEBAR_BG)),
        ..Default::default()
    }
}

pub fn sidebar_button_style(
    _theme: &iced::Theme,
    status: iced_widget::button::Status,
) -> iced_widget::button::Style {
    use iced_widget::button;
    let base = button::Style {
        background: None,
        text_color: iced::Color::WHITE,
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(SIDEBAR_HOVER)),
            ..base
        },
        _ => base,
    }
}

pub fn sidebar_button_active_style(
    _theme: &iced::Theme,
    _status: iced_widget::button::Status,
) -> iced_widget::button::Style {
    iced_widget::button::Style {
        background: Some(iced::Background::Color(SIDEBAR_ACTIVE)),
        text_color: iced::Color::WHITE,
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

// ── Content / Card Styles ────────────────────────────────────────────

pub fn content_style() -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(iced::Background::Color(CONTENT_BG)),
        ..Default::default()
    }
}

pub fn card_style() -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(iced::Background::Color(CARD_BG)),
        border: iced::Border {
            radius: 8.0.into(),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            offset: iced::Vector { x: 0.0, y: 1.0 },
            blur_radius: 3.0,
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.08),
        },
        ..Default::default()
    }
}

pub fn log_container_style() -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(iced::Background::Color(LOG_BG)),
        border: iced::Border {
            radius: 6.0.into(),
            width: 0.0,
            color: iced::Color::TRANSPARENT,
        },
        text_color: Some(LOG_TEXT),
        ..Default::default()
    }
}

// ── Button Styles ────────────────────────────────────────────────────

pub fn primary_button_style(
    _theme: &iced::Theme,
    status: iced_widget::button::Status,
) -> iced_widget::button::Style {
    use iced_widget::button;
    let base = button::Style {
        background: Some(iced::Background::Color(PRIMARY_BLUE)),
        text_color: iced::Color::WHITE,
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(PRIMARY_BLUE_HOVER)),
            ..base
        },
        _ => base,
    }
}

pub fn danger_button_style(
    _theme: &iced::Theme,
    status: iced_widget::button::Status,
) -> iced_widget::button::Style {
    use iced_widget::button;
    let base = button::Style {
        background: Some(iced::Background::Color(ERROR_RED)),
        text_color: iced::Color::WHITE,
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(ERROR_RED_HOVER)),
            ..base
        },
        _ => base,
    }
}

pub fn secondary_button_style(
    _theme: &iced::Theme,
    status: iced_widget::button::Status,
) -> iced_widget::button::Style {
    use iced_widget::button;
    let base = button::Style {
        background: None,
        text_color: PRIMARY_BLUE,
        border: iced::Border {
            radius: 6.0.into(),
            width: 1.0,
            color: PRIMARY_BLUE,
        },
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(iced::Background::Color(BLUE_SOFT)),
            ..base
        },
        _ => base,
    }
}
