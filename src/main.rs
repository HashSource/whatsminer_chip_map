mod analysis;
mod api;
mod config;
mod models;
mod theme;
mod ui;

use iced::{
    Element, Length, Task, Theme,
    widget::{button, column, container, pick_list, row, text, text_input},
};

use models::{ColorMode, MinerData, SystemInfo};

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
    Fetched(Result<(MinerData, SystemInfo), String>),
    DividerDragStart,
    DividerDragEnd,
    DividerDrag(f32),
    ColorModeChanged(ColorMode),
}

#[derive(Default)]
struct App {
    ip: String,
    user: String,
    pass: String,
    status: String,
    data: Option<MinerData>,
    system_info: Option<SystemInfo>,
    loading: bool,
    sidebar_width: f32,
    dragging: bool,
    color_mode: ColorMode,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                ip: "192.7.1.193".into(),
                user: "admin".into(),
                pass: "admin".into(),
                status: "Ready".into(),
                sidebar_width: 500.0,
                ..Default::default()
            },
            Task::none(),
        )
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
                    async move { api::fetch_all(&ip, &user, &pass).await },
                    Message::Fetched,
                );
            }
            Message::Fetched(Ok((data, info))) => {
                self.loading = false;
                self.status = format!("{} slots, {} chips", data.slots.len(), data.total_chips());
                self.data = Some(data);
                self.system_info = Some(info);
            }
            Message::Fetched(Err(e)) => {
                self.loading = false;
                self.status = format!("Error: {e}");
                self.data = None;
                self.system_info = None;
            }
            Message::DividerDragStart => self.dragging = true,
            Message::DividerDragEnd => self.dragging = false,
            Message::DividerDrag(x) if self.dragging => {
                self.sidebar_width = x.clamp(150.0, 500.0);
            }
            Message::DividerDrag(_) => {}
            Message::ColorModeChanged(mode) => self.color_mode = mode,
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
                Message::ColorModeChanged
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

        let content = match &self.data {
            Some(data) => ui::miner_view(
                data,
                self.system_info.as_ref(),
                self.sidebar_width,
                self.dragging,
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
