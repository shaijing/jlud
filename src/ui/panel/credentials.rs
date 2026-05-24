use crate::ui::{AppState, CredentialsTab, Message};
use crate::ui::theme;

pub fn view(state: &AppState) -> iced::Element<'_, Message> {
    use iced::Length;
    use iced_widget::{button, column, container, row, text, Space};

    // === Tab Bar ===

    let tab_button = |label: &'static str, tab: CredentialsTab| -> iced::Element<'_, Message> {
        let active = state.active_credentials_tab == tab;
        if active {
            container(
                text(label).size(13).color(theme::PRIMARY_BLUE)
            )
            .padding([8u16, 20])
            .style(move |_: &iced::Theme| iced_widget::container::Style {
                background: Some(iced::Background::Color(theme::CARD_BG)),
                border: iced::Border {
                    radius: 6.0.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow: iced::Shadow {
                    offset: iced::Vector { x: 0.0, y: 1.0 },
                    blur_radius: 2.0,
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                },
                ..Default::default()
            })
            .into()
        } else {
            button(text(label).size(13).color(theme::TEXT_SECONDARY))
                .style(iced_widget::button::text)
                .on_press(Message::SetCredentialsTab(tab))
                .padding([8, 20])
                .into()
        }
    };

    let tab_bar = row![
        tab_button("Create", CredentialsTab::New),
        Space::new().width(Length::Fixed(4.0)),
        tab_button("Load",   CredentialsTab::Load),
        Space::new().width(Length::Fixed(4.0)),
        tab_button("Inspect", CredentialsTab::Inspect),
    ]
    .spacing(0);

    // === Tab Content ===

    let content = match state.active_credentials_tab {
        CredentialsTab::New     => new_tab(state),
        CredentialsTab::Load    => load_tab(state),
        CredentialsTab::Inspect => inspect_tab(state),
    };

    let body = container(content)
        .style(theme::card_style())
        .padding(24)
        .width(Length::Fill);

    container(
        column![tab_bar, Space::new().height(Length::Fixed(16.0)), body, Space::new().height(Length::Fill)]
    )
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

// === New Tab ===

fn new_tab(state: &AppState) -> iced::Element<'_, Message> {
    use iced_widget::{button, column, text, text_input, Space};

    fn field<'a>(label: &'a str, placeholder: &'a str, value: &'a str, msg: fn(String) -> Message) -> iced::Element<'a, Message> {
        column![
            text(label).size(12).color(theme::TEXT_SECONDARY),
            Space::new().height(4.0),
            text_input(placeholder, value)
                .on_input(msg)
                .size(14)
                .width(iced::Length::Fill),
        ]
        .spacing(0)
        .into()
    }

    column![
        text("Create User File").size(16),
        Space::new().height(16.0),
        field("Username",    "username",           &state.form_username, Message::UpdateUsername),
        Space::new().height(12.0),
        // Password field: masked input
        column![
            text("Password").size(12).color(theme::TEXT_SECONDARY),
            Space::new().height(4.0),
            text_input("password", &state.form_password)
                .on_input(Message::UpdatePassword)
                .secure(true)
                .size(14)
                .width(iced::Length::Fill),
        ]
        .spacing(0),
        Space::new().height(12.0),
        field("MAC Address", "00:11:22:33:44:55",  &state.form_mac,      Message::UpdateMac),
        Space::new().height(12.0),
        field("Output File", "/path/to/output.usr", &state.form_usr_path, Message::UpdateUsrPath),
        Space::new().height(20.0),
        button(text("Create .usr File").size(14))
            .style(theme::primary_button_style)
            .on_press(Message::CreateUsr {
                username: state.form_username.clone(),
                password: state.form_password.clone(),
                mac:      state.form_mac.clone(),
                path:     state.form_usr_path.clone(),
            })
            .width(iced::Length::Fixed(180.0))
            .height(iced::Length::Fixed(38.0)),
    ]
    .spacing(0)
    .into()
}

// === Load Tab ===

fn load_tab(state: &AppState) -> iced::Element<'_, Message> {
    use iced_widget::{button, column, row, text, text_input, Space};

    let feedback: iced::Element<_> = if let Some(ref f) = state.usr_file {
        if std::path::Path::new(f).exists() {
            row![
                text("✓").size(14).color(theme::CONNECTED_GREEN),
                text(format!("Loaded: {}", f)).size(13).color(theme::TEXT_PRIMARY),
            ]
            .spacing(8)
            .into()
        } else {
            text(format!("File not found: {}", f)).size(13).color(theme::ERROR_RED).into()
        }
    } else {
        text("No file loaded yet").size(13).color(theme::TEXT_SECONDARY).into()
    };

    column![
        text("Load Authentication File").size(16),
        Space::new().height(16.0),
        text("File Path").size(12).color(theme::TEXT_SECONDARY),
        Space::new().height(4.0),
        text_input("path/to/file.usr", &state.form_usr_path)
            .on_input(Message::UpdateUsrPath)
            .size(14)
            .width(iced::Length::Fill),
        Space::new().height(12.0),
        row![
            button(text("Load File").size(14))
                .style(theme::primary_button_style)
                .on_press(Message::LoadUsrFile(state.form_usr_path.clone()))
                .width(iced::Length::Fixed(140.0))
                .height(iced::Length::Fixed(38.0)),
            Space::new().width(12.0),
            feedback,
        ]
        .align_y(iced::Alignment::Center),
    ]
    .spacing(0)
    .into()
}

// === Inspect Tab ===

fn inspect_tab(state: &AppState) -> iced::Element<'_, Message> {
    use iced_widget::{button, column, row, text, text_input, Space};

    let user_section: iced::Element<_> = if let Some(ref user) = state.decrypted_user {
        column![
            text("Decrypted User Data").size(13).color(theme::TEXT_SECONDARY),
            Space::new().height(12.0),
            row![text("Username:").size(13).color(theme::TEXT_SECONDARY), Space::new().width(8.0), text(user.username.as_str()).size(14)].spacing(0),
            row![
                text("MAC:").size(13).color(theme::TEXT_SECONDARY),
                Space::new().width(8.0),
                text(format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    user.mac[0], user.mac[1], user.mac[2],
                    user.mac[3], user.mac[4], user.mac[5],
                )).size(14),
            ].spacing(0),
        ]
        .spacing(4)
        .into()
    } else {
        text("No user data loaded").size(13).color(theme::TEXT_SECONDARY).into()
    };

    column![
        text("Inspect User File").size(16),
        Space::new().height(16.0),
        text("File Path").size(12).color(theme::TEXT_SECONDARY),
        Space::new().height(4.0),
        text_input("path/to/file.usr", &state.form_usr_path)
            .on_input(Message::UpdateUsrPath)
            .size(14)
            .width(iced::Length::Fill),
        Space::new().height(12.0),
        row![
            button(text("Load & Decrypt").size(14))
                .style(theme::primary_button_style)
                .on_press(Message::InspectUsrFile(state.form_usr_path.clone()))
                .width(iced::Length::Fixed(180.0))
                .height(iced::Length::Fixed(38.0)),
            Space::new().width(16.0),
            if let Some(ref path) = state.usr_file {
                text(format!("File: {}", path)).size(13).color(theme::TEXT_PRIMARY)
            } else {
                text("").size(13)
            },
        ]
        .align_y(iced::Alignment::Center),
        Space::new().height(20.0),
        user_section,
    ]
    .spacing(0)
    .into()
}
