pub mod panel;
pub mod theme;
pub mod tray;

use crate::config::AppSettings;
use iced::{Element, Length, Subscription};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::Arc;

// === Message ===

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(NavItem),
    // Connect
    Connect,
    Disconnect,
    // Poll tick
    Tick,
    // Window close → minimize to tray
    CloseRequested,
    ConfirmQuit,
    ConfirmMinimizeToTray,
    // Credentials
    CreateUsr { username: String, password: String, mac: String },
    DismissResultDialog,
    InspectUsrFile,
    // Settings
    UpdateLogLevel(String),
    UpdateTimeout(u64),
    UpdateRetryCount(Option<u64>),
    UpdateRetryDelay(u64),
    SaveSettings,
    // Credentials tab
    SetCredentialsTab(CredentialsTab),
    // Form fields
    UpdateUsername(String),
    UpdatePassword(String),
    UpdateMac(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavItem {
    Connect,
    Credentials,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialsTab {
    New,
    Inspect,
}

// === Connection State ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

// === Background Event (auth thread → UI) ===

#[derive(Debug, Clone)]
pub enum BackgroundEvent {
    Status(ConnectionState),
    Log { level: String, message: String },
    Error(String),
    UserDecrypted(crate::cygnus::user::data::User),
    UsrFileCreated(String),
    CreateFailed(String),
}

// === AppState ===

pub struct AppState {
    pub nav: NavItem,
    pub connection_state: ConnectionState,
    pub usr_file: Option<String>,
    pub logs: Vec<LogEntry>,
    pub settings: AppSettings,
    pub active_credentials_tab: CredentialsTab,
    // Form state for New tab
    pub form_username: String,
    pub form_password: String,
    pub form_mac: String,
    // Decrypted user for Inspect tab
    pub decrypted_user: Option<crate::cygnus::user::data::User>,
    // Channel receiver for background auth events
    #[allow(clippy::type_complexity)]
    event_rx: Option<mpsc::Receiver<BackgroundEvent>>,
    // Shared sender cloned by background threads
    event_tx: mpsc::Sender<BackgroundEvent>,
    // Tray
    pub visible: bool,
    pub show_close_dialog: bool,
    pub show_result_dialog: bool,
    pub result_dialog_title: String,
    pub result_dialog_message: String,
    pub result_dialog_is_error: bool,
    tray_rx: mpsc::Receiver<tray::TrayMessage>,
    #[allow(dead_code)]
    _tray_icon: Option<tray_icon::TrayIcon>,
}

pub struct LogEntry {
    pub level: String,
    pub message: String,
}

// === Tracing Layer (forwards auth logs to GUI) ===

use tracing::Subscriber;
use tracing::Event;
use tracing::field::{Visit, Field};
use tracing_subscriber::layer::Layer;

/// Captures tracing events from the auth thread and forwards them as `BackgroundEvent::Log`.
struct AuthLogLayer {
    tx: mpsc::Sender<BackgroundEvent>,
}

struct LogMessageVisitor {
    message: String,
}

impl Visit for LogMessageVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }
}

impl<S: Subscriber + 'static> Layer<S> for AuthLogLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let level = match *event.metadata().level() {
            tracing::Level::ERROR => "error",
            tracing::Level::WARN => "warn",
            tracing::Level::INFO => "info",
            tracing::Level::DEBUG => "debug",
            tracing::Level::TRACE => "trace",
        };
        let mut visitor = LogMessageVisitor { message: String::new() };
        event.record(&mut visitor);
        let _ = self.tx.send(BackgroundEvent::Log {
            level: level.to_string(),
            message: format!("[{}] {}", event.metadata().target(), visitor.message),
        });
    }
}

// === UIApp ===

pub struct UIApp {
    pub state: AppState,
    cancel_auth: Arc<AtomicBool>,
}

impl UIApp {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let settings = AppSettings::load();
        let usr_file = settings.last_usr_file.clone();

        // Create system tray
        let tray_handle = tray::TrayHandle::new();
        let _tray_icon = Some(tray_handle.keep_alive());
        let tray_rx = tray_handle.rx;

        Self {
            state: AppState {
                nav: NavItem::Connect,
                connection_state: ConnectionState::Disconnected,
                usr_file,
                logs: Vec::new(),
                settings,
                active_credentials_tab: CredentialsTab::New,
                form_username: String::new(),
                form_password: String::new(),
                form_mac: detect_wired_mac(),
                decrypted_user: None,
                event_rx: Some(rx),
                event_tx: tx,
                visible: true,
                show_close_dialog: false,
                show_result_dialog: false,
                result_dialog_title: String::new(),
                result_dialog_message: String::new(),
                result_dialog_is_error: false,
                tray_rx,
                _tray_icon,
            },
            cancel_auth: Arc::new(AtomicBool::new(false)),
        }
    }

    fn handle_background_event(&mut self, event: BackgroundEvent) {
        match event {
            BackgroundEvent::Status(state) => {
                self.state.connection_state = state;
            }
            BackgroundEvent::Log { level, message } => {
                self.state.logs.push(LogEntry { level, message });
                if self.state.logs.len() > 200 {
                    self.state.logs.remove(0);
                }
            }
            BackgroundEvent::Error(message) => {
                self.state.logs.push(LogEntry {
                    level: "error".into(),
                    message,
                });
            }
            BackgroundEvent::UserDecrypted(user) => {
                self.state.decrypted_user = Some(user);
            }
            BackgroundEvent::UsrFileCreated(path) => {
                self.state.usr_file = Some(path);
                self.state.show_result_dialog = true;
                self.state.result_dialog_title = "Success".into();
                self.state.result_dialog_message = "User file created and loaded.".into();
                self.state.result_dialog_is_error = false;
            }
            BackgroundEvent::CreateFailed(msg) => {
                self.state.show_result_dialog = true;
                self.state.result_dialog_title = "Error".into();
                self.state.result_dialog_message = msg;
                self.state.result_dialog_is_error = true;
            }
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Navigate(nav) => {
                self.state.nav = nav;
                iced::Task::none()
            }
            Message::Connect => {
                if self.state.connection_state == ConnectionState::Connected
                    || self.state.connection_state == ConnectionState::Connecting
                {
                    return iced::Task::none(); // already connected/connecting
                }
                // Check if a .usr file is selected
                let usr_file = match self.state.usr_file.clone() {
                    Some(f) => f,
                    None => {
                        self.state.logs.push(LogEntry {
                            level: "error".into(),
                            message: "No .usr file selected. Please load a file first.".into(),
                        });
                        return iced::Task::none();
                    }
                };
                // Check if the file exists on disk
                if !std::path::Path::new(&usr_file).exists() {
                    self.state.logs.push(LogEntry {
                        level: "error".into(),
                        message: format!("File not found: {}", usr_file),
                    });
                    return iced::Task::none();
                }
                let settings = self.state.settings.clone();
                let tx = self.state.event_tx.clone();
                self.state.connection_state = ConnectionState::Connecting;

                // Create a fresh cancel token for this connection attempt
                let cancel = Arc::new(AtomicBool::new(false));
                self.cancel_auth = cancel.clone();

                std::thread::spawn(move || {
                    use crate::cygnus::auth::auth_command_resolver;
                    use crate::cygnus::auth::args::{AuthArgs, LogLevel};
                    use tracing_subscriber::layer::SubscriberExt;

                    let _ = tx.send(BackgroundEvent::Status(ConnectionState::Connecting));
                    let _ = tx.send(BackgroundEvent::Log {
                        level: "info".into(),
                        message: "Starting auth...".into(),
                    });

                    let log_level = match settings.log_level.as_str() {
                        "trace" => LogLevel::Trace,
                        "debug" => LogLevel::Debug,
                        "warn" => LogLevel::Warn,
                        "error" => LogLevel::Error,
                        _ => LogLevel::Info,
                    };

                    let args = AuthArgs {
                        file: usr_file,
                        log_level,
                        timeout: settings.timeout,
                        retry: settings.retry_count,
                        delay: settings.retry_delay,
                    };

                    // Route all tracing logs from the auth code to the GUI
                    let layer = AuthLogLayer { tx: tx.clone() };
                    let subscriber = tracing_subscriber::registry().with(layer);
                    let dispatch = tracing::Dispatch::new(subscriber);

                    // Drcom keep-alive loops forever after successful login.
                    // Report "Connected" now — auth is running and maintaining the connection.
                    let _ = tx.send(BackgroundEvent::Status(ConnectionState::Connected));

                    tracing::dispatcher::with_default(&dispatch, || {
                        match auth_command_resolver(args, Some(cancel.clone())) {
                            Ok(()) => {} // unreachable: keep_alive loops forever
                            Err(e) => {
                                // If cancelled by user (disconnect), don't report as error
                                if cancel.load(std::sync::atomic::Ordering::Relaxed) {
                                    let _ = tx.send(BackgroundEvent::Log {
                                        level: "info".into(),
                                        message: "Authentication cancelled by user".into(),
                                    });
                                } else {
                                    let _ = tx.send(BackgroundEvent::Error(e.to_string()));
                                    let _ = tx.send(BackgroundEvent::Status(ConnectionState::Error));
                                }
                            }
                        }
                    });
                });
                iced::Task::none()
            }
            Message::Disconnect => {
                self.state.connection_state = ConnectionState::Disconnected;
                self.cancel_auth.store(true, std::sync::atomic::Ordering::Relaxed);
                iced::Task::none()
            }
            Message::Tick => {
                // Poll for events from background threads
                let events: Vec<BackgroundEvent> = {
                    if let Some(ref rx) = self.state.event_rx {
                        rx.try_iter().collect()
                    } else {
                        Vec::new()
                    }
                };
                for event in events {
                    self.handle_background_event(event);
                }

                // Poll for tray messages
                while let Ok(msg) = self.state.tray_rx.try_recv() {
                    match msg {
                        tray::TrayMessage::ToggleVisibility => {
                            self.state.visible = !self.state.visible;
                            let mode = if self.state.visible {
                                iced::window::Mode::Windowed
                            } else {
                                iced::window::Mode::Hidden
                            };
                            return iced::window::latest()
                                .and_then(move |id| iced::window::set_mode(id, mode));
                        }
                        tray::TrayMessage::Quit => {
                            std::process::exit(0);
                        }
                    }
                }
                iced::Task::none()
            }
            Message::CloseRequested => {
                self.state.show_close_dialog = true;
                iced::Task::none()
            }
            Message::ConfirmQuit => {
                std::process::exit(0);
            }
            Message::ConfirmMinimizeToTray => {
                self.state.show_close_dialog = false;
                self.state.visible = false;
                return iced::window::latest()
                    .and_then(|id| iced::window::set_mode(id, iced::window::Mode::Hidden));
            }
            Message::SaveSettings => {
                match self.state.settings.save() {
                    Ok(()) => {
                        self.state.logs.push(LogEntry {
                            level: "info".into(),
                            message: "Settings saved successfully".into(),
                        });
                    }
                    Err(e) => {
                        self.state.logs.push(LogEntry {
                            level: "error".into(),
                            message: format!("Failed to save settings: {}", e),
                        });
                    }
                }
                iced::Task::none()
            }
            Message::UpdateLogLevel(level) => {
                self.state.settings.log_level = level;
                let _ = self.state.settings.save();
                iced::Task::none()
            }
            Message::UpdateTimeout(timeout) => {
                self.state.settings.timeout = timeout;
                let _ = self.state.settings.save();
                iced::Task::none()
            }
            Message::UpdateRetryCount(count) => {
                self.state.settings.retry_count = count;
                let _ = self.state.settings.save();
                iced::Task::none()
            }
            Message::UpdateRetryDelay(delay) => {
                self.state.settings.retry_delay = delay;
                let _ = self.state.settings.save();
                iced::Task::none()
            }
            Message::CreateUsr { username, password, mac } => {
                let tx = self.state.event_tx.clone();
                std::thread::spawn(move || {
                    use crate::cygnus::user::user_command_resolver;
                    use crate::cygnus::user::args::{UserArgs, UserCommand, UserCreateArgs};

                    // Generate a deterministic cache path based on username hash
                    let cache_path = match crate::config::AppSettings::cache_dir() {
                        Some(dir) => {
                            let hash = format!("{:x}", md5::compute(username.as_bytes()));
                            std::fs::create_dir_all(&dir).ok();
                            dir.join(format!("usr_{}.usr", hash)).to_string_lossy().to_string()
                        }
                        None => {
                            let _ = tx.send(BackgroundEvent::Error(
                                "Failed to resolve cache directory".into(),
                            ));
                            return;
                        }
                    };

                    let args = UserArgs {
                        command: UserCommand::Create(UserCreateArgs {
                            username,
                            password,
                            mac,
                            file: cache_path.clone(),
                        }),
                    };

                    match user_command_resolver(args) {
                        Ok(()) => {
                            let _ = tx.send(BackgroundEvent::Log {
                                level: "info".into(),
                                message: "User file created successfully".into(),
                            });
                            let _ = tx.send(BackgroundEvent::UsrFileCreated(cache_path));
                        }
                        Err(e) => {
                            let _ = tx.send(BackgroundEvent::CreateFailed(
                                format!("Failed to create user file: {}", e),
                            ));
                        }
                    }
                });
                iced::Task::none()
            }
            Message::DismissResultDialog => {
                self.state.show_result_dialog = false;
                iced::Task::none()
            }
            Message::InspectUsrFile => {
                let path = match self.state.usr_file.clone() {
                    Some(p) => p,
                    None => return iced::Task::none(),
                };
                self.state.settings.last_usr_file = Some(path.clone());
                let _ = self.state.settings.save();
                if path.is_empty() {
                    return iced::Task::none();
                }
                let tx = self.state.event_tx.clone();
                std::thread::spawn(move || {
                    use crate::cygnus::user::cipher::UserCipher;
                    use std::fs::OpenOptions;

                    let fd = match OpenOptions::new().read(true).open(&path) {
                        Ok(fd) => fd,
                        Err(e) => {
                            let _ = tx.send(BackgroundEvent::Error(format!("Cannot open file: {}", e)));
                            return;
                        }
                    };

                    match UserCipher::decrypt(fd) {
                        Ok(user) => {
                            let _ = tx.send(BackgroundEvent::Log {
                                level: "info".into(),
                                message: format!("Decrypted user: {}", user.username),
                            });
                            let _ = tx.send(BackgroundEvent::UserDecrypted(user));
                        }
                        Err(e) => {
                            let _ = tx.send(BackgroundEvent::Error(format!("Decrypt failed: {}", e)));
                        }
                    }
                });
                iced::Task::none()
            }
            Message::SetCredentialsTab(tab) => {
                self.state.active_credentials_tab = tab;
                iced::Task::none()
            }
            Message::UpdateUsername(s) => {
                self.state.form_username = s;
                iced::Task::none()
            }
            Message::UpdatePassword(s) => {
                self.state.form_password = s;
                iced::Task::none()
            }
            Message::UpdateMac(s) => {
                self.state.form_mac = s;
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.state.show_close_dialog {
            return self.close_dialog_view();
        }
        if self.state.show_result_dialog {
            return self.result_dialog_view();
        }

        use crate::ui::panel::{connect, credentials, settings};
        use crate::ui::theme::{self, content_style, sidebar_style};

        use iced_widget::{button, column, container, row, text, Space};

        // === Sidebar ===

        let title = container(
            text("Cygnus Drcom")
                .size(15)
                .color(iced::Color::WHITE)
        )
        .padding(iced::Padding { top: 14.0, right: 16.0, bottom: 12.0, left: 16.0 });

        fn nav(name: &'static str, item: NavItem, active: bool) -> Element<'static, Message> {
            let lbl = text(name).size(13);
            if active {
                button(lbl)
                    .style(theme::sidebar_button_active_style)
                    .on_press(Message::Navigate(item))
                    .width(iced::Length::Fill)
                    .into()
            } else {
                button(lbl)
                    .style(theme::sidebar_button_style)
                    .on_press(Message::Navigate(item))
                    .width(iced::Length::Fill)
                    .into()
            }
        }

        let nav_col = column![
            nav("Connect",     NavItem::Connect,     self.state.nav == NavItem::Connect),
            nav("Credentials", NavItem::Credentials, self.state.nav == NavItem::Credentials),
            nav("Settings",    NavItem::Settings,    self.state.nav == NavItem::Settings),
            Space::new().height(Length::Fill),
        ]
        .spacing(2)
        .padding(iced::Padding { top: 0.0, right: 6.0, bottom: 6.0, left: 6.0 });

        let sidebar = container(
            column![title, nav_col]
        )
        .style(sidebar_style())
        .width(Length::Fixed(160.0))
        .height(Length::Fill);

        // === Content ===

        let content_panel = match self.state.nav {
            NavItem::Connect     => connect::view(&self.state),
            NavItem::Credentials => credentials::view(&self.state),
            NavItem::Settings    => settings::view(&self.state.settings),
        };

        let content = container(content_panel)
            .style(content_style())
            .width(Length::Fill)
            .height(Length::Fill);

        row![sidebar, content].into()
    }

    fn close_dialog_view(&self) -> Element<'_, Message> {
        use iced_widget::{button, column, container, row, text};
        use crate::ui::theme::card_style;

        container(
            container(
                column![
                    text("Close Cygnus Drcom?").size(18).color(theme::TEXT_PRIMARY),
                    text("Do you want to quit or minimize to the system tray?").size(13).color(theme::TEXT_PRIMARY),
                    row![
                        button(text("Minimize to Tray").size(13))
                            .on_press(Message::ConfirmMinimizeToTray)
                            .style(theme::primary_button_style),
                        button(text("Quit").size(13))
                            .on_press(Message::ConfirmQuit)
                            .style(theme::danger_button_style),
                    ]
                    .spacing(10),
                ]
                .spacing(14)
                .align_x(iced::Alignment::Center),
            )
            .padding(24)
            .style(card_style()),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(20)
        .into()
    }

    fn result_dialog_view(&self) -> Element<'_, Message> {
        use iced_widget::{button, column, container, text};
        use crate::ui::theme::card_style;

        let icon = if self.state.result_dialog_is_error { "✗" } else { "✓" };
        let icon_color = if self.state.result_dialog_is_error {
            theme::ERROR_RED
        } else {
            theme::CONNECTED_GREEN
        };

        let btn_style: fn(&iced::Theme, iced_widget::button::Status) -> iced_widget::button::Style =
            if self.state.result_dialog_is_error {
                theme::danger_button_style
            } else {
                theme::primary_button_style
            };

        container(
            container(
                column![
                    text(icon).size(32).color(icon_color),
                    text(self.state.result_dialog_title.as_str()).size(18),
                    text(self.state.result_dialog_message.as_str()).size(13)
                        .color(theme::TEXT_SECONDARY),
                    button(text("OK").size(14))
                        .style(btn_style)
                        .on_press(Message::DismissResultDialog)
                        .width(iced::Length::Fixed(100.0))
                        .height(iced::Length::Fixed(36.0)),
                ]
                .spacing(14)
                .align_x(iced::Alignment::Center),
            )
            .padding(28)
            .style(card_style()),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(20)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        use iced::window;
        Subscription::batch([
            iced::time::every(std::time::Duration::from_millis(100))
                .map(|_| Message::Tick),
            window::close_requests().map(|_id| Message::CloseRequested),
        ])
    }
}

/// Try to detect the MAC address of the wired Ethernet interface.
/// Falls back to empty string if detection fails.
fn detect_wired_mac() -> String {
    // Try the default MAC address first
    if let Ok(Some(mac)) = mac_address::get_mac_address() {
        let s = mac.to_string();
        if !s.is_empty() && s != "00:00:00:00:00:00" {
            return s;
        }
    }

    // Try common wired interface names across platforms
    for name in &["eth0", "en0", "enp0s3", "enx*"] {
        if let Ok(Some(mac)) = mac_address::mac_address_by_name(name) {
            let s = mac.to_string();
            if !s.is_empty() && s != "00:00:00:00:00:00" {
                return s;
            }
        }
    }

    String::new()
}

pub fn run() -> iced::Result {
    iced::application(
        || UIApp::new(),
        |app: &mut UIApp, msg| app.update(msg),
        UIApp::view,
    )
    .subscription(|app: &UIApp| app.subscription())
    .title("Cygnus Drcom")
    .default_font(iced::Font::with_name("Segoe UI"))
    .exit_on_close_request(false)
    .window_size(iced::Size::new(660.0, 560.0))
    .centered()
    .run()
}
