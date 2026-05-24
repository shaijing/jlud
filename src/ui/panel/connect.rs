use crate::ui::theme;
use crate::ui::{AppState, ConnectionState};

pub fn view(state: &AppState) -> iced::Element<'_, crate::ui::Message> {
    use iced::{Alignment, Length};
    use iced_widget::{button, column, container, row, scrollable, text, Space};

    // === Status Card ===

    let status_color = match state.connection_state {
        ConnectionState::Connected    => theme::CONNECTED_GREEN,
        ConnectionState::Connecting   => theme::WARN_YELLOW,
        ConnectionState::Error        => theme::ERROR_RED,
        ConnectionState::Disconnected => theme::TEXT_SECONDARY,
    };

    let (status_label, status_detail) = match state.connection_state {
        ConnectionState::Connected    => ("Connected",        "Connection is active"),
        ConnectionState::Connecting   => ("Connecting...",    "Authenticating with server"),
        ConnectionState::Error        => ("Error",            "Authentication failed"),
        ConnectionState::Disconnected => ("Disconnected",     "Not connected"),
    };

    let status_dot = container(container(Space::new().width(Length::Fixed(16.0)).height(Length::Fixed(16.0))))
        .style(move |_| iced_widget::container::Style {
            background: Some(iced::Background::Color(status_color)),
            border: iced::Border { radius: 8.0.into(), ..Default::default() },
            ..Default::default()
        });

    let status_card = container(
        row![
            status_dot,
            column![
                text(status_label).size(18).color(theme::TEXT_PRIMARY),
                text(status_detail).size(11).color(theme::TEXT_SECONDARY),
            ]
            .spacing(3)
        ]
        .spacing(14)
        .align_y(Alignment::Center)
    )
    .style(theme::card_style())
    .padding(16)
    .width(Length::Fill);

    // === Action Button ===

    let button_text = match state.connection_state {
        ConnectionState::Connected    => "Disconnect",
        ConnectionState::Connecting   => "Connecting...",
        _                             => "Connect",
    };

    let is_connecting = state.connection_state == ConnectionState::Connecting;
    let is_connected  = state.connection_state == ConnectionState::Connected;

    let action_msg = if is_connected || is_connecting {
        crate::ui::Message::Disconnect
    } else {
        crate::ui::Message::Connect
    };

    let style_fn: fn(&iced::Theme, iced_widget::button::Status) -> iced_widget::button::Style =
        if is_connected {
            theme::danger_button_style
        } else if is_connecting {
            theme::secondary_button_style
        } else {
            theme::primary_button_style
        };

    let action_btn = button(text(button_text).size(14))
        .style(style_fn)
        .on_press(action_msg)
        .width(Length::Fixed(200.0))
        .height(Length::Fixed(38.0));

    // === File Card ===

    let file_info: iced::Element<'_, crate::ui::Message> = if let Some(ref path) = state.usr_file {
        let exists = std::path::Path::new(path).exists();
        let icon = if exists { "✓" } else { "✗" };
        let color = if exists { theme::CONNECTED_GREEN } else { theme::ERROR_RED };
        row![
            text(icon).size(16).color(color),
            text(path.as_str()).size(13)
                .color(if exists { theme::TEXT_PRIMARY } else { theme::ERROR_RED }),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
    } else {
        row![
            text("⚠").size(16).color(theme::WARN_YELLOW),
            text("No file loaded").size(13).color(theme::WARN_YELLOW),
            text("—").size(13).color(theme::TEXT_SECONDARY),
            text("Go to Credentials → Load").size(13).color(theme::TEXT_SECONDARY),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
    };

    let file_card = container(
        column![
            text("Authentication File").size(11).color(theme::TEXT_SECONDARY),
            Space::new().height(Length::Fixed(6.0)),
            file_info,
        ]
    )
    .style(theme::card_style())
    .padding(14)
    .width(Length::Fill);

    // === Live Logs ===

    let log_items: Vec<iced::Element<_>> = state.logs.iter().rev().take(100).map(|entry| {
        let color = match entry.level.as_str() {
            "warn" | "warning" => theme::WARN_YELLOW,
            "error"            => theme::ERROR_RED,
            "info"             => theme::CONNECTED_GREEN,
            _                  => iced::Color::from_rgb(0.7, 0.75, 0.85),
        };
        row![
            text(format!("{:>5}", entry.level.to_uppercase())).size(11).color(color),
            text(entry.message.as_str()).size(11)
                .color(iced::Color::from_rgb(0.8, 0.85, 0.9)),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
    }).collect();

    let log_area: iced::Element<_> = if log_items.is_empty() {
        text("Waiting for events...")
            .size(12)
            .color(iced::Color::from_rgb(0.4, 0.45, 0.5))
            .into()
    } else {
        column(log_items).spacing(3).into()
    };

    let logs_panel = container(
        column![
            text("Live Logs").size(13).color(theme::TEXT_SECONDARY),
            Space::new().height(Length::Fixed(6.0)),
            container(scrollable(container(log_area).padding(10)).height(Length::Fill))
                .style(theme::log_container_style())
                .width(Length::Fill)
                .height(Length::Fill),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill);

    // === Layout ===

    container(
        column![
            status_card,
            action_btn,
            Space::new().height(Length::Fixed(10.0)),
            file_card,
            Space::new().height(Length::Fixed(12.0)),
            logs_panel,
        ]
        .spacing(0)
    )
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
