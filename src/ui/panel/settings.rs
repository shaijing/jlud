use crate::config::AppSettings;
use crate::ui::theme;

fn card_section<'a>(
    title: &'static str,
    body: iced::Element<'a, crate::ui::Message>,
) -> iced::Element<'a, crate::ui::Message> {
    use iced::Length;
    use iced_widget::{column, container, text, Space};

    container(
        column![
            text(title).size(14),
            Space::new().height(10.0),
            body,
        ]
    )
    .style(theme::card_style())
    .padding(16)
    .width(Length::Fill)
    .into()
}

pub fn view(settings: &AppSettings) -> iced::Element<'_, crate::ui::Message> {
    use iced::{Alignment, Length};
    use iced_widget::{button, column, container, pick_list, row, text, text_input, Space};

    // Log Level
    let log_levels: Vec<String> = ["trace", "debug", "info", "warn", "error"]
        .iter().map(|s| s.to_string()).collect();
    let selected = log_levels.iter().find(|s| s == &&settings.log_level).cloned();
    let log_body: iced::Element<_> = row![
        text("Log Level").size(13).color(theme::TEXT_SECONDARY).width(Length::Fixed(120.0)),
        pick_list(log_levels, selected, crate::ui::Message::UpdateLogLevel)
            .width(Length::Fixed(140.0)),
        Space::new().width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .into();

    // Timeout
    let timeout = settings.timeout;
    let timeout_body: iced::Element<_> = row![
        text("Timeout (s)").size(13).color(theme::TEXT_SECONDARY).width(Length::Fixed(120.0)),
        text_input("5", &timeout.to_string())
            .on_input(move |s| s.parse::<u64>()
                .map(crate::ui::Message::UpdateTimeout)
                .unwrap_or(crate::ui::Message::UpdateTimeout(timeout)))
            .size(14)
            .width(Length::Fixed(100.0)),
        Space::new().width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .into();

    // Retry Count
    let retry_count = settings.retry_count;
    let retry_body: iced::Element<_> = row![
        text("Retry Count").size(13).color(theme::TEXT_SECONDARY).width(Length::Fixed(120.0)),
        text_input("empty = infinite", &retry_count.map(|n| n.to_string()).unwrap_or_default())
            .on_input(move |s| crate::ui::Message::UpdateRetryCount(
                if s.is_empty() { None } else { s.parse::<u64>().ok() }))
            .size(14)
            .width(Length::Fixed(100.0)),
        Space::new().width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .into();

    // Retry Delay
    let retry_delay = settings.retry_delay;
    let delay_body: iced::Element<_> = row![
        text("Retry Delay (ms)").size(13).color(theme::TEXT_SECONDARY).width(Length::Fixed(120.0)),
        text_input("500", &retry_delay.to_string())
            .on_input(move |s| s.parse::<u64>()
                .map(crate::ui::Message::UpdateRetryDelay)
                .unwrap_or(crate::ui::Message::UpdateRetryDelay(retry_delay)))
            .size(14)
            .width(Length::Fixed(100.0)),
        Space::new().width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .into();

    let save_btn = row![
        Space::new().width(Length::Fill),
        button(text("Save Settings").size(14))
            .style(theme::primary_button_style)
            .on_press(crate::ui::Message::SaveSettings)
            .width(iced::Length::Fixed(140.0))
            .height(iced::Length::Fixed(38.0)),
    ];

    container(
        column![
            text("Settings").size(20),
            Space::new().height(20.0),
            card_section("Connection", column![log_body, Space::new().height(12.0), timeout_body].into()),
            Space::new().height(10.0),
            card_section("Retry Policy", column![retry_body, Space::new().height(12.0), delay_body].into()),
            Space::new().height(14.0),
            save_btn,
        ]
    )
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
