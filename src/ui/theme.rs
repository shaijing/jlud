use iced_widget::container;

// Brand colors
pub const SIDEBAR_BG: iced::Color = iced::Color::from_rgb(0.098, 0.18, 0.357);   // #192b5b deep navy
pub const SIDEBAR_ACTIVE: iced::Color = iced::Color::from_rgb(0.22, 0.35, 0.65);  // #3859a6 highlight
pub const SIDEBAR_HOVER: iced::Color = iced::Color::from_rgb(0.18, 0.28, 0.50);   // #2e4780 hover
pub const CONTENT_BG: iced::Color = iced::Color::from_rgb(0.945, 0.949, 0.957);   // #f1f2f4 off-white
pub const CARD_BG: iced::Color = iced::Color::WHITE;
pub const TEXT_PRIMARY: iced::Color = iced::Color::from_rgb(0.13, 0.13, 0.13);
pub const TEXT_SECONDARY: iced::Color = iced::Color::from_rgb(0.4, 0.4, 0.45);

// Status colors
pub const CONNECTED_GREEN: iced::Color = iced::Color::from_rgb(0.13, 0.76, 0.38); // #22C161
pub const DISCONNECTED_GRAY: iced::Color = iced::Color::from_rgb(0.55, 0.55, 0.55);
pub const ERROR_RED: iced::Color = iced::Color::from_rgb(0.9, 0.3, 0.3);
pub const WARN_YELLOW: iced::Color = iced::Color::from_rgb(0.95, 0.72, 0.15);

// Accent
pub const PRIMARY_BLUE: iced::Color = iced::Color::from_rgb(0.2, 0.45, 0.9);
pub const LOG_BG: iced::Color = iced::Color::from_rgb(0.12, 0.12, 0.14);           // #1e1e24 dark terminal

// === Sidebar Styles ===

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
            background: Some(iced::Background::Color(SIDEBAR_HOVER.into())),
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
        background: Some(iced::Background::Color(SIDEBAR_ACTIVE.into())),
        text_color: iced::Color::WHITE,
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

// === Content / Card Styles ===

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
        text_color: Some(iced::Color::from_rgb(0.8, 0.85, 0.9)),
        ..Default::default()
    }
}

// === Button Styles ===

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
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.25, 0.5, 0.95))),
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
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.95, 0.35, 0.35))),
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
            background: Some(iced::Background::Color(iced::Color::from_rgba(0.2, 0.45, 0.9, 0.1))),
            ..base
        },
        _ => base,
    }
}
