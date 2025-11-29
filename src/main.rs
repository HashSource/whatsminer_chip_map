mod api;
mod models;
mod theme;
mod ui;

use iced::{
    Element, Length, Task, Theme,
    widget::{button, column, container, pick_list, row, text, text_input},
};

use models::{ColorMode, MinerData};

const DEFAULT_IP: &str = "192.6.8.15";
const DEFAULT_USER: &str = "admin";
const DEFAULT_PASS: &str = "admin";

fn main() -> iced::Result {
    iced::application("WhatsMiner Chip Map", App::update, App::view)
        .theme(|_| Theme::Dark)
        .run_with(App::new)
}

#[derive(Debug, Clone)]
pub enum Message {
    IpChanged(String),
    UserChanged(String),
    PassChanged(String),
    Fetch,
    Fetched(Result<MinerData, String>),
    DividerDragStart,
    DividerDragEnd,
    DividerDrag(f32),
    ColorModeChanged(ColorMode),
}

struct App {
    ip: String,
    user: String,
    pass: String,
    status: String,
    data: Option<MinerData>,
    loading: bool,
    sidebar_width: f32,
    dragging_divider: bool,
    color_mode: ColorMode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ip: String::new(),
            user: String::new(),
            pass: String::new(),
            status: String::new(),
            data: None,
            loading: false,
            sidebar_width: 500.0,
            dragging_divider: false,
            color_mode: ColorMode::default(),
        }
    }
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let app = Self {
            ip: DEFAULT_IP.into(),
            user: DEFAULT_USER.into(),
            pass: DEFAULT_PASS.into(),
            status: "Ready".into(),
            ..Default::default()
        };
        (app, Task::none())
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::IpChanged(v) => self.ip = v,
            Message::UserChanged(v) => self.user = v,
            Message::PassChanged(v) => self.pass = v,
            Message::Fetch => {
                self.loading = true;
                self.status = "Connecting...".into();
                let (ip, user, pass) = (self.ip.clone(), self.user.clone(), self.pass.clone());
                return Task::perform(
                    async move { api::fetch(&ip, &user, &pass).await },
                    Message::Fetched,
                );
            }
            Message::Fetched(result) => {
                self.loading = false;
                match result {
                    Ok(data) => {
                        self.status =
                            format!("{} slots, {} chips", data.slots.len(), data.total_chips());
                        self.data = Some(data);
                    }
                    Err(e) => {
                        self.status = format!("Error: {e}");
                        self.data = None;
                    }
                }
            }
            Message::DividerDragStart => {
                self.dragging_divider = true;
            }
            Message::DividerDragEnd => {
                self.dragging_divider = false;
            }
            Message::DividerDrag(x) => {
                if self.dragging_divider {
                    self.sidebar_width = x.clamp(150.0, 500.0);
                }
            }
            Message::ColorModeChanged(mode) => {
                self.color_mode = mode;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            text_input("IP", &self.ip)
                .on_input(Message::IpChanged)
                .padding(10)
                .width(200),
            text_input("User", &self.user)
                .on_input(Message::UserChanged)
                .padding(10)
                .width(120),
            text_input("Pass", &self.pass)
                .on_input(Message::PassChanged)
                .padding(10)
                .width(120)
                .secure(true),
            if self.loading {
                button(text("Loading...")).padding(10)
            } else {
                button(text("Fetch")).on_press(Message::Fetch).padding(10)
            },
            text("Color:").size(14),
            pick_list(
                ColorMode::ALL,
                Some(self.color_mode),
                Message::ColorModeChanged,
            )
            .padding(8)
            .width(120),
        ]
        .spacing(10)
        .padding(10)
        .align_y(iced::Alignment::Center);

        let status = container(text(&self.status).size(14))
            .padding(10)
            .width(Length::Fill);

        let content: Element<'_, Message> = match &self.data {
            Some(data) => ui::render_miner_view(
                data,
                self.sidebar_width,
                self.dragging_divider,
                self.color_mode,
            ),
            None => container(text("Click 'Fetch' to load miner data").size(16))
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        };

        column![controls, status, content]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
