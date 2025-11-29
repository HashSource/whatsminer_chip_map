//! Internationalization support

use std::fmt;

/// Supported languages
/// Note: Persian/Chinese removed - RTL bug and missing font glyphs respectively
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Language {
    #[default]
    English,
    Russian,
    Spanish,
}

impl Language {
    pub const ALL: &[Self] = &[Self::English, Self::Russian, Self::Spanish];
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::English => "en",
            Self::Russian => "ru",
            Self::Spanish => "es",
        })
    }
}

/// Translation strings
pub struct Tr;

impl Tr {
    // Window (kept for potential future dynamic title support)
    #[allow(dead_code)]
    pub fn app_title(lang: Language) -> &'static str {
        match lang {
            Language::English => "WhatsMiner Chip Map",
            Language::Russian => "Карта чипов WhatsMiner",
            Language::Spanish => "Mapa de chips WhatsMiner",
        }
    }

    // Status messages
    pub fn ready(lang: Language) -> &'static str {
        match lang {
            Language::English => "Ready",
            Language::Russian => "Готово",
            Language::Spanish => "Listo",
        }
    }

    pub fn connecting(lang: Language) -> &'static str {
        match lang {
            Language::English => "Connecting...",
            Language::Russian => "Подключение...",
            Language::Spanish => "Conectando...",
        }
    }

    pub fn error(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish => "Error",
            Language::Russian => "Ошибка",
        }
    }

    // Input placeholders
    pub fn ip(_lang: Language) -> &'static str {
        "IP"
    }

    pub fn user(lang: Language) -> &'static str {
        match lang {
            Language::English => "User",
            Language::Russian => "Пользователь",
            Language::Spanish => "Usuario",
        }
    }

    pub fn pass(lang: Language) -> &'static str {
        match lang {
            Language::English => "Pass",
            Language::Russian => "Пароль",
            Language::Spanish => "Clave",
        }
    }

    // Buttons
    pub fn fetch(lang: Language) -> &'static str {
        match lang {
            Language::English => "Fetch",
            Language::Russian => "Загрузить",
            Language::Spanish => "Obtener",
        }
    }

    pub fn loading(lang: Language) -> &'static str {
        match lang {
            Language::English => "Loading...",
            Language::Russian => "Загрузка...",
            Language::Spanish => "Cargando...",
        }
    }

    // Labels
    pub fn color(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish => "Color:",
            Language::Russian => "Цвет:",
        }
    }

    pub fn lang(lang: Language) -> &'static str {
        match lang {
            Language::English => "Lang:",
            Language::Russian => "Язык:",
            Language::Spanish => "Idioma:",
        }
    }

    pub fn click_fetch(lang: Language) -> &'static str {
        match lang {
            Language::English => "Click 'Fetch' to load miner data",
            Language::Russian => "Нажмите 'Загрузить' для получения данных",
            Language::Spanish => "Haga clic en 'Obtener' para cargar datos",
        }
    }

    // Sidebar
    pub fn system_info(lang: Language) -> &'static str {
        match lang {
            Language::English => "── System Info ──",
            Language::Russian => "── Информация ──",
            Language::Spanish => "── Info Sistema ──",
        }
    }

    pub fn firmware(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish => "FW",
            Language::Russian => "ПО",
        }
    }

    pub fn slot(lang: Language) -> &'static str {
        match lang {
            Language::English => "Slot",
            Language::Russian => "Слот",
            Language::Spanish => "Ranura",
        }
    }

    pub fn chips(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish => "chips",
            Language::Russian => "чипов",
        }
    }

    pub fn slots(lang: Language) -> &'static str {
        match lang {
            Language::English => "slots",
            Language::Russian => "слотов",
            Language::Spanish => "ranuras",
        }
    }

    // ColorMode translations
    pub fn color_mode_temperature(lang: Language) -> &'static str {
        match lang {
            Language::English => "Temperature",
            Language::Russian => "Температура",
            Language::Spanish => "Temperatura",
        }
    }

    pub fn color_mode_errors(lang: Language) -> &'static str {
        match lang {
            Language::English => "Errors",
            Language::Russian => "Ошибки",
            Language::Spanish => "Errores",
        }
    }

    pub fn color_mode_crc(_lang: Language) -> &'static str {
        "CRC"
    }

    pub fn color_mode_gradient(lang: Language) -> &'static str {
        match lang {
            Language::English => "Gradient",
            Language::Russian => "Градиент",
            Language::Spanish => "Gradiente",
        }
    }

    pub fn color_mode_outliers(lang: Language) -> &'static str {
        match lang {
            Language::English => "Outliers",
            Language::Russian => "Выбросы",
            Language::Spanish => "Atípicos",
        }
    }

    pub fn color_mode_nonce(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish => "Nonce",
            Language::Russian => "Нонс",
        }
    }
}

/// Localized ColorMode for display in picker
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalizedColorMode {
    pub mode: crate::models::ColorMode,
    pub lang: Language,
}

impl LocalizedColorMode {
    pub fn all(lang: Language) -> Vec<Self> {
        crate::models::ColorMode::ALL
            .iter()
            .map(|&mode| Self { mode, lang })
            .collect()
    }
}

impl fmt::Display for LocalizedColorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::models::ColorMode;
        f.write_str(match self.mode {
            ColorMode::Temperature => Tr::color_mode_temperature(self.lang),
            ColorMode::Errors => Tr::color_mode_errors(self.lang),
            ColorMode::Crc => Tr::color_mode_crc(self.lang),
            ColorMode::Gradient => Tr::color_mode_gradient(self.lang),
            ColorMode::Outliers => Tr::color_mode_outliers(self.lang),
            ColorMode::Nonce => Tr::color_mode_nonce(self.lang),
        })
    }
}
