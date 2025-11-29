#![windows_subsystem = "windows"]

mod analysis;
mod api;
mod config;
mod i18n;
mod models;
mod theme;
mod ui;

use iced::{
    Element, Length, Task, Theme,
    widget::{button, column, container, pick_list, row, text, text_input},
};

use i18n::{Language, LocalizedColorMode, Tr};
use models::{ColorMode, MinerData, SystemInfo};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .run()
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
    ColorModeChanged(LocalizedColorMode),
    LanguageChanged(Language),
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
    language: Language,
}

impl App {
    fn title(&self) -> String {
        "WhatsMiner Chip Map".into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn new() -> (Self, Task<Message>) {
        let language = Language::default();
        (
            Self {
                ip: "192.7.1.193".into(),
                user: "admin".into(),
                pass: "admin".into(),
                status: Tr::ready(language).into(),
                sidebar_width: 500.0,
                language,
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        let lang = self.language;
        match msg {
            Message::IpChanged(v) => self.ip = v,
            Message::UserChanged(v) => self.user = v,
            Message::PassChanged(v) => self.pass = v,
            Message::Fetch => {
                self.loading = true;
                self.status = Tr::connecting(lang).into();
                let (ip, user, pass) = (self.ip.clone(), self.user.clone(), self.pass.clone());
                return Task::perform(
                    async move { api::fetch_all(&ip, &user, &pass).await },
                    Message::Fetched,
                );
            }
            Message::Fetched(Ok((data, info))) => {
                self.loading = false;
                self.status = format!(
                    "{} {}, {} {}",
                    data.slots.len(),
                    Tr::slots(lang),
                    data.total_chips(),
                    Tr::chips(lang)
                );
                self.data = Some(data);
                self.system_info = Some(info);
            }
            Message::Fetched(Err(e)) => {
                self.loading = false;
                self.status = format!("{}: {e}", Tr::error(lang));
                self.data = None;
                self.system_info = None;
            }
            Message::DividerDragStart => self.dragging = true,
            Message::DividerDragEnd => self.dragging = false,
            Message::DividerDrag(x) if self.dragging => {
                self.sidebar_width = x.clamp(150.0, 500.0);
            }
            Message::DividerDrag(_) => {}
            Message::ColorModeChanged(lcm) => self.color_mode = lcm.mode,
            Message::LanguageChanged(l) => {
                self.language = l;
                // Update status message if it's a static message
                if self.data.is_none() && !self.loading {
                    self.status = Tr::ready(l).into();
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let lang = self.language;
        let selected_color = LocalizedColorMode {
            mode: self.color_mode,
            lang,
        };

        let controls = row![
            text_input(Tr::ip(lang), &self.ip)
                .on_input(Message::IpChanged)
                .padding(10)
                .width(200),
            text_input(Tr::user(lang), &self.user)
                .on_input(Message::UserChanged)
                .padding(10)
                .width(120),
            text_input(Tr::pass(lang), &self.pass)
                .on_input(Message::PassChanged)
                .padding(10)
                .width(120)
                .secure(true),
            if self.loading {
                button(text(Tr::loading(lang))).padding(10)
            } else {
                button(text(Tr::fetch(lang)))
                    .on_press(Message::Fetch)
                    .padding(10)
            },
            text(Tr::color(lang)).size(14),
            pick_list(
                LocalizedColorMode::all(lang),
                Some(selected_color),
                Message::ColorModeChanged
            )
            .padding(8)
            .width(150),
            text(Tr::lang(lang)).size(14),
            pick_list(Language::ALL, Some(lang), Message::LanguageChanged)
                .padding(8)
                .width(100),
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
                lang,
            ),
            None => container(text(Tr::click_fetch(lang)).size(16))
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
