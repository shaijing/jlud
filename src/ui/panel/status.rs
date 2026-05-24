use crate::ui::{AppState, ConnectionState};
use crate::ui::theme;

pub fn view(state: &AppState) -> iced::Element<'_, crate::ui::Message> {
    use iced::{Alignment, Length};
    use iced_widget::{column, container, row, scrollable, text, Space};

    // === Status Badge ===

    let (status_color, status_label) = match state.connection_state {
        ConnectionState::Connected    => (theme::CONNECTED_GREEN,    "Connected"),
        ConnectionState::Connecting   => (theme::WARN_YELLOW,       "Connecting"),
        ConnectionState::Error        => (theme::ERROR_RED,         "Error"),
        ConnectionState::Disconnected => (theme::TEXT_SECONDARY, "Disconnected"),
    };

    let badge = container(
        row![
            container(Space::new().width(Length::Fixed(10.0)).height(Length::Fixed(10.0)))
                .style(move |_| iced_widget::container::Style {
                    background: Some(iced::Background::Color(status_color)),
                    border: iced::Border { radius: 5.0.into(), ..Default::default() },
                    ..Default::default()
                }),
            text(status_label).size(14),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    )
    .style(theme::card_style())
    .padding([10, 16])
    .width(Length::Fill);

    // === Live Logs ===

    let log_items: Vec<iced::Element<_>> = state.logs.iter().rev().take(100).map(|entry| {
        let color = match entry.level.as_str() {
            "warn" | "warning" => theme::WARN_YELLOW,
            "error"            => theme::ERROR_RED,
            "info"             => theme::CONNECTED_GREEN,
            _                  => iced::Color::from_rgb(0.7, 0.75, 0.85),
        };
        let ts = chrono_or_empty();
        row![
            text(ts).size(11).color(theme::TEXT_SECONDARY),
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

    let logs = container(
        column![
            scrollable(container(log_area).padding(12))
                .height(Length::Fill),
        ]
        .height(Length::Fill)
    )
    .style(theme::log_container_style())
    .padding(0)
    .width(Length::Fill)
    .height(Length::Fill);

    // === Layout ===

    container(
        column![
            row![
                text("Connection Status").size(16),
                Space::new().width(Length::Fill),
                badge,
            ]
            .align_y(Alignment::Center),
            Space::new().height(Length::Fixed(12.0)),
            text("Live Logs").size(14).color(theme::TEXT_SECONDARY),
            Space::new().height(Length::Fixed(8.0)),
            logs,
        ]
        .spacing(0)
    )
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn chrono_or_empty() -> String {
    // Try to get current time; fallback to empty if time crate not available
    String::new()
}
