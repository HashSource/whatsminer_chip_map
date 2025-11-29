use iced::{Background, Border, Color, color, widget::container};

use crate::models::ColorMode;

// Brand colors
pub const BRAND_ORANGE: Color = color!(0xF7, 0x93, 0x1A);

// Base colors
const BG_DARK: Color = color!(0x0D, 0x0D, 0x0D);
const BG_PANEL: Color = color!(0x1A, 0x1A, 0x1A);
const BORDER_SUBTLE: Color = color!(0x3A, 0x3A, 0x3A);
const BORDER_ACCENT: Color = color!(0x4A, 0x4A, 0x4A);

// Gradient ranges (min, max) for each mode
const TEMP_RANGE: (f32, f32) = (40.0, 100.0);
const ERROR_RANGE: (f32, f32) = (0.0, 150.0);
const CRC_RANGE: (f32, f32) = (0.0, 15.0);

// Board temperature range for sidebar
const BOARD_TEMP_RANGE: (f32, f32) = (30.0, 90.0);

/// Gradient color stops: Green → Yellow → Orange → Red
/// Each stop is (position, background, border)
const GRADIENT_STOPS: [(f32, Color, Color); 4] = [
    (0.0, color!(0x16, 0x4E, 0x32), color!(0x22, 0xC5, 0x5E)), // Green
    (0.4, color!(0x71, 0x5B, 0x0B), color!(0xF5, 0xCE, 0x0B)), // Yellow
    (0.7, color!(0x7C, 0x2D, 0x12), color!(0xEA, 0x58, 0x0C)), // Orange
    (1.0, color!(0x7F, 0x1D, 0x1D), color!(0xDC, 0x26, 0x26)), // Red
];

/// Text color gradient stops
const TEXT_GRADIENT_STOPS: [(f32, Color); 4] = [
    (0.0, color!(0x4A, 0xDE, 0x80)), // Green
    (0.4, color!(0xFB, 0xCF, 0x24)), // Yellow
    (0.7, color!(0xF9, 0x73, 0x16)), // Orange
    (1.0, color!(0xEF, 0x44, 0x44)), // Red
];

/// Linearly interpolate between two colors
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

/// Normalize value to 0.0-1.0 range
fn normalize(value: f32, min: f32, max: f32) -> f32 {
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

/// Get gradient color pair (background, border) for normalized position
fn gradient_colors(t: f32) -> (Color, Color) {
    for i in 1..GRADIENT_STOPS.len() {
        let (pos_a, bg_a, border_a) = GRADIENT_STOPS[i - 1];
        let (pos_b, bg_b, border_b) = GRADIENT_STOPS[i];
        if t <= pos_b {
            let local_t = (t - pos_a) / (pos_b - pos_a);
            return (
                lerp_color(bg_a, bg_b, local_t),
                lerp_color(border_a, border_b, local_t),
            );
        }
    }
    let last = GRADIENT_STOPS.last().unwrap();
    (last.1, last.2)
}

/// Get gradient text color for normalized position
fn gradient_text_color(t: f32) -> Color {
    for i in 1..TEXT_GRADIENT_STOPS.len() {
        let (pos_a, color_a) = TEXT_GRADIENT_STOPS[i - 1];
        let (pos_b, color_b) = TEXT_GRADIENT_STOPS[i];
        if t <= pos_b {
            let local_t = (t - pos_a) / (pos_b - pos_a);
            return lerp_color(color_a, color_b, local_t);
        }
    }
    TEXT_GRADIENT_STOPS.last().unwrap().1
}

/// Text color for chip temperature display (gradient)
#[allow(clippy::cast_precision_loss)] // temp values fit in f32
pub fn color_for_chip_temp(temp: i32) -> Color {
    let t = normalize(temp as f32, TEMP_RANGE.0, TEMP_RANGE.1);
    gradient_text_color(t)
}

/// Text color for board temperature display (gradient)
#[allow(clippy::cast_possible_truncation)] // temp values fit in f32
pub fn color_for_board_temp(temp: f64) -> Color {
    let t = normalize(temp as f32, BOARD_TEMP_RANGE.0, BOARD_TEMP_RANGE.1);
    gradient_text_color(t)
}

/// Chip cell style with gradient coloring based on mode
#[allow(clippy::cast_precision_loss)] // small integer values fit in f32
pub fn chip_cell(temp: i32, errors: i32, crc: i32, mode: ColorMode) -> container::Style {
    let t = match mode {
        ColorMode::Temperature => normalize(temp as f32, TEMP_RANGE.0, TEMP_RANGE.1),
        ColorMode::Errors => normalize(errors as f32, ERROR_RANGE.0, ERROR_RANGE.1),
        ColorMode::Crc => normalize(crc as f32, CRC_RANGE.0, CRC_RANGE.1),
    };
    let (bg, border) = gradient_colors(t);

    container::Style {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(bg)),
        border: Border {
            color: border,
            width: 1.5,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

pub fn slot_container() -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_PANEL)),
        border: Border {
            color: BORDER_ACCENT,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn sidebar_container() -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_DARK)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

pub fn divider_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(BORDER_ACCENT)),
        ..Default::default()
    }
}

pub fn tooltip_style() -> container::Style {
    container::Style {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(BG_PANEL)),
        border: Border {
            color: BRAND_ORANGE,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}
