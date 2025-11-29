//! Internationalization support

use std::fmt;

/// Supported languages
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Language {
    #[default]
    English,
    Russian,
    Spanish,
    Persian,
    Chinese,
    Ukrainian,
    Polish,
    Kazakh,
    Arabic,
}

impl Language {
    pub const ALL: &[Self] = &[
        Self::English,
        Self::Russian,
        Self::Spanish,
        Self::Persian,
        Self::Chinese,
        Self::Ukrainian,
        Self::Polish,
        Self::Kazakh,
        Self::Arabic,
    ];
}

impl fmt::Display for Language {
    /// Returns ISO 639-1 two-letter language codes
    /// See: https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::English => "en",   // English
            Self::Russian => "ru",   // Russian (Русский)
            Self::Spanish => "es",   // Spanish (Español)
            Self::Persian => "fa",   // Persian/Farsi (فارسی)
            Self::Chinese => "zh",   // Chinese (中文)
            Self::Ukrainian => "uk", // Ukrainian (Українська)
            Self::Polish => "pl",    // Polish (Polski)
            Self::Kazakh => "kk",    // Kazakh (Қазақша)
            Self::Arabic => "ar",    // Arabic (العربية)
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
            Language::Persian => "نقشه چیپ WhatsMiner",
            Language::Chinese => "WhatsMiner 芯片图",
            Language::Ukrainian => "Карта чіпів WhatsMiner",
            Language::Polish => "Mapa chipów WhatsMiner",
            Language::Kazakh => "WhatsMiner чип картасы",
            Language::Arabic => "خريطة شرائح WhatsMiner",
        }
    }

    // Status messages
    pub fn ready(lang: Language) -> &'static str {
        match lang {
            Language::English => "Ready",
            Language::Russian => "Готово",
            Language::Spanish => "Listo",
            Language::Persian => "آماده",
            Language::Chinese => "就绪",
            Language::Ukrainian => "Готово",
            Language::Polish => "Gotowe",
            Language::Kazakh => "Дайын",
            Language::Arabic => "جاهز",
        }
    }

    pub fn connecting(lang: Language) -> &'static str {
        match lang {
            Language::English => "Connecting...",
            Language::Russian => "Подключение...",
            Language::Spanish => "Conectando...",
            Language::Persian => "در حال اتصال...",
            Language::Chinese => "连接中...",
            Language::Ukrainian => "Підключення...",
            Language::Polish => "Łączenie...",
            Language::Kazakh => "Қосылуда...",
            Language::Arabic => "جاري الاتصال...",
        }
    }

    pub fn error(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish => "Error",
            Language::Russian => "Ошибка",
            Language::Persian => "خطا",
            Language::Chinese => "错误",
            Language::Ukrainian => "Помилка",
            Language::Polish => "Błąd",
            Language::Kazakh => "Қате",
            Language::Arabic => "خطأ",
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
            Language::Persian => "کاربر",
            Language::Chinese => "用户",
            Language::Ukrainian => "Користувач",
            Language::Polish => "Użytkownik",
            Language::Kazakh => "Пайдаланушы",
            Language::Arabic => "مستخدم",
        }
    }

    pub fn pass(lang: Language) -> &'static str {
        match lang {
            Language::English => "Pass",
            Language::Russian => "Пароль",
            Language::Spanish => "Clave",
            Language::Persian => "رمز",
            Language::Chinese => "密码",
            Language::Ukrainian => "Пароль",
            Language::Polish => "Hasło",
            Language::Kazakh => "Құпиясөз",
            Language::Arabic => "كلمة السر",
        }
    }

    // Buttons
    pub fn fetch(lang: Language) -> &'static str {
        match lang {
            Language::English => "Fetch",
            Language::Russian => "Загрузить",
            Language::Spanish => "Obtener",
            Language::Persian => "دریافت",
            Language::Chinese => "获取",
            Language::Ukrainian => "Завантажити",
            Language::Polish => "Pobierz",
            Language::Kazakh => "Жүктеу",
            Language::Arabic => "جلب",
        }
    }

    pub fn loading(lang: Language) -> &'static str {
        match lang {
            Language::English => "Loading...",
            Language::Russian => "Загрузка...",
            Language::Spanish => "Cargando...",
            Language::Persian => "بارگذاری...",
            Language::Chinese => "加载中...",
            Language::Ukrainian => "Завантаження...",
            Language::Polish => "Ładowanie...",
            Language::Kazakh => "Жүктелуде...",
            Language::Arabic => "جاري التحميل...",
        }
    }

    // Labels
    pub fn color(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish => "Color:",
            Language::Russian => "Цвет:",
            Language::Persian => "رنگ:",
            Language::Chinese => "颜色:",
            Language::Ukrainian => "Колір:",
            Language::Polish => "Kolor:",
            Language::Kazakh => "Түс:",
            Language::Arabic => "اللون:",
        }
    }

    pub fn lang(lang: Language) -> &'static str {
        match lang {
            Language::English => "Lang:",
            Language::Russian => "Язык:",
            Language::Spanish => "Idioma:",
            Language::Persian => "زبان:",
            Language::Chinese => "语言:",
            Language::Ukrainian => "Мова:",
            Language::Polish => "Język:",
            Language::Kazakh => "Тіл:",
            Language::Arabic => "اللغة:",
        }
    }

    pub fn click_fetch(lang: Language) -> &'static str {
        match lang {
            Language::English => "Click 'Fetch' to load miner data",
            Language::Russian => "Нажмите 'Загрузить' для получения данных",
            Language::Spanish => "Haga clic en 'Obtener' para cargar datos",
            Language::Persian => "برای بارگذاری داده‌ها روی 'دریافت' کلیک کنید",
            Language::Chinese => "点击'获取'加载矿机数据",
            Language::Ukrainian => "Натисніть 'Завантажити' для отримання даних",
            Language::Polish => "Kliknij 'Pobierz' aby załadować dane",
            Language::Kazakh => "Деректерді жүктеу үшін 'Жүктеу' басыңыз",
            Language::Arabic => "انقر 'جلب' لتحميل بيانات المُعدِّن",
        }
    }

    // Sidebar
    pub fn system_info(lang: Language) -> &'static str {
        match lang {
            Language::English => "── System Info ──",
            Language::Russian => "── Сист. инфо ──",
            Language::Spanish => "── Info Sistema ──",
            Language::Persian => "── اطلاعات سیستم ──",
            Language::Chinese => "── 系统信息 ──",
            Language::Ukrainian => "── Сист. інфо ──",
            Language::Polish => "── Info Systemu ──",
            Language::Kazakh => "── Жүйе ақпараты ──",
            Language::Arabic => "── معلومات النظام ──",
        }
    }

    pub fn firmware(lang: Language) -> &'static str {
        // FW = Firmware (not Software/ПО)
        match lang {
            Language::Chinese => "固件",
            _ => "FW", // International abbreviation
        }
    }

    pub fn slot(lang: Language) -> &'static str {
        match lang {
            Language::English => "Slot",
            Language::Russian => "Слот",
            Language::Spanish => "Ranura",
            Language::Persian => "اسلات",
            Language::Chinese => "槽位",
            Language::Ukrainian => "Слот",
            Language::Polish => "Slot",
            Language::Kazakh => "Слот",
            Language::Arabic => "فتحة",
        }
    }

    pub fn chips(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish => "chips",
            Language::Russian => "чипов",
            Language::Persian => "چیپ",
            Language::Chinese => "芯片",
            Language::Ukrainian => "чіпів",
            Language::Polish => "chipów",
            Language::Kazakh => "чип",
            Language::Arabic => "شريحة",
        }
    }

    pub fn slots(lang: Language) -> &'static str {
        match lang {
            Language::English => "slots",
            Language::Russian => "слотов",
            Language::Spanish => "ranuras",
            Language::Persian => "اسلات",
            Language::Chinese => "槽位",
            Language::Ukrainian => "слотів",
            Language::Polish => "slotów",
            Language::Kazakh => "слот",
            Language::Arabic => "فتحات",
        }
    }

    // ColorMode translations
    pub fn color_mode_temperature(lang: Language) -> &'static str {
        match lang {
            Language::English => "Temperature",
            Language::Russian => "Температура",
            Language::Spanish => "Temperatura",
            Language::Persian => "دما",
            Language::Chinese => "温度",
            Language::Ukrainian => "Температура",
            Language::Polish => "Temperatura",
            Language::Kazakh => "Температура",
            Language::Arabic => "الحرارة",
        }
    }

    pub fn color_mode_errors(lang: Language) -> &'static str {
        match lang {
            Language::English => "Errors",
            Language::Russian => "Ошибки",
            Language::Spanish => "Errores",
            Language::Persian => "خطاها",
            Language::Chinese => "错误",
            Language::Ukrainian => "Помилки",
            Language::Polish => "Błędy",
            Language::Kazakh => "Қателер",
            Language::Arabic => "الأخطاء",
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
            Language::Persian => "گرادیان",
            Language::Chinese => "梯度",
            Language::Ukrainian => "Градієнт",
            Language::Polish => "Gradient",
            Language::Kazakh => "Градиент",
            Language::Arabic => "التدرج",
        }
    }

    pub fn color_mode_outliers(lang: Language) -> &'static str {
        match lang {
            Language::English => "Outliers",
            Language::Russian => "Выбросы",
            Language::Spanish => "Atípicos",
            Language::Persian => "پرت‌ها",
            Language::Chinese => "异常值",
            Language::Ukrainian => "Викиди",
            Language::Polish => "Odstające",
            Language::Kazakh => "Ауытқулар",
            Language::Arabic => "القيم الشاذة",
        }
    }

    pub fn color_mode_nonce(lang: Language) -> &'static str {
        match lang {
            Language::English | Language::Spanish | Language::Polish => "Nonce",
            Language::Russian => "Нонс",
            Language::Persian => "نانس",
            Language::Chinese => "随机数",
            Language::Ukrainian => "Нонс",
            Language::Kazakh => "Нонс",
            Language::Arabic => "نونس",
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
