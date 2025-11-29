use iced::{Background, Border, Color, color, widget::container};

use crate::models::ColorMode;

// Brand colors
pub const BRAND_ORANGE: Color = color!(0xF7, 0x93, 0x1A);

// Base colors
const BG_DARK: Color = color!(0x0D, 0x0D, 0x0D);
const BG_PANEL: Color = color!(0x1A, 0x1A, 0x1A);
const BORDER_SUBTLE: Color = color!(0x3A, 0x3A, 0x3A);
const BORDER_ACCENT: Color = color!(0x4A, 0x4A, 0x4A);

// Temperature thresholds from WhatsMiner firmware:
// - Chip: ft_chip_temp_warn 95-105°C, chip_temp_protect 110-120°C
// - Board: cool_temp 30-38°C, board_temp_overheat/protect 80-90°C
const CHIP_THRESHOLDS: [i32; 3] = [70, 85, 95]; // cool, warm, hot
const BOARD_THRESHOLDS: [i32; 3] = [50, 65, 80];
const ERROR_THRESHOLDS: [i32; 3] = [20, 50, 100];
const CRC_THRESHOLDS: [i32; 3] = [1, 5, 10];

/// Severity level for threshold-based coloring
#[derive(Clone, Copy)]
enum Severity {
    Cool,
    Warm,
    Hot,
    Critical,
}

impl Severity {
    fn from_value(value: i32, thresholds: [i32; 3]) -> Self {
        match value {
            v if v >= thresholds[2] => Self::Critical,
            v if v >= thresholds[1] => Self::Hot,
            v if v >= thresholds[0] => Self::Warm,
            _ => Self::Cool,
        }
    }

    const fn text_color(self) -> Color {
        match self {
            Self::Cool => color!(0x4A, 0xDE, 0x80),     // Green
            Self::Warm => color!(0xFB, 0xBF, 0x24),     // Amber
            Self::Hot => color!(0xF9, 0x73, 0x16),      // Orange
            Self::Critical => color!(0xEF, 0x44, 0x44), // Red
        }
    }

    const fn chip_colors(self) -> (Color, Color) {
        match self {
            Self::Cool => (color!(0x16, 0x4E, 0x32), color!(0x22, 0xC5, 0x5E)),
            Self::Warm => (color!(0x71, 0x4B, 0x0B), color!(0xF5, 0x9E, 0x0B)),
            Self::Hot => (color!(0x7C, 0x2D, 0x12), color!(0xEA, 0x58, 0x0C)),
            Self::Critical => (color!(0x7F, 0x1D, 0x1D), color!(0xDC, 0x26, 0x26)),
        }
    }
}

/// Text color for chip temperature display
pub fn color_for_chip_temp(temp: i32) -> Color {
    Severity::from_value(temp, CHIP_THRESHOLDS).text_color()
}

/// Text color for board temperature display
pub fn color_for_board_temp(temp: f64) -> Color {
    Severity::from_value(temp as i32, BOARD_THRESHOLDS).text_color()
}

/// Chip cell style based on color mode
pub fn chip_cell(temp: i32, errors: i32, crc: i32, mode: ColorMode) -> container::Style {
    let severity = match mode {
        ColorMode::Temperature => Severity::from_value(temp, CHIP_THRESHOLDS),
        ColorMode::Errors => Severity::from_value(errors, ERROR_THRESHOLDS),
        ColorMode::Crc => Severity::from_value(crc, CRC_THRESHOLDS),
    };
    let (bg, border) = severity.chip_colors();

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
