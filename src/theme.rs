use iced::{Background, Border, Color, color, widget::container};

// WhatsMiner brand colors
pub const BRAND_ORANGE: Color = color!(0xF7, 0x93, 0x1A);

// Base colors
const BG_DARK: Color = color!(0x0D, 0x0D, 0x0D);
const BG_PANEL: Color = color!(0x1A, 0x1A, 0x1A);
const BORDER_SUBTLE: Color = color!(0x3A, 0x3A, 0x3A);
const BORDER_ACCENT: Color = color!(0x4A, 0x4A, 0x4A);

// Temperature text colors (for sidebar)
pub const TEMP_COOL: Color = color!(0x4A, 0xDE, 0x80); // Green
pub const TEMP_WARM: Color = color!(0xFB, 0xBF, 0x24); // Amber
pub const TEMP_HOT: Color = color!(0xF9, 0x73, 0x16); // Orange
pub const TEMP_CRIT: Color = color!(0xEF, 0x44, 0x44); // Red

// Chip background colors - richer, more saturated
const CHIP_BG_COOL: Color = color!(0x16, 0x4E, 0x32); // Deep green
const CHIP_BG_WARM: Color = color!(0x71, 0x4B, 0x0B); // Deep amber
const CHIP_BG_HOT: Color = color!(0x7C, 0x2D, 0x12); // Deep orange
const CHIP_BG_CRIT: Color = color!(0x7F, 0x1D, 0x1D); // Deep red
const CHIP_BG_ERROR: Color = color!(0x99, 0x1B, 0x1B); // Error red

// Chip border colors
const CHIP_BORDER_COOL: Color = color!(0x22, 0xC5, 0x5E); // Bright green
const CHIP_BORDER_WARM: Color = color!(0xF5, 0x9E, 0x0B); // Bright amber
const CHIP_BORDER_HOT: Color = color!(0xEA, 0x58, 0x0C); // Bright orange
const CHIP_BORDER_CRIT: Color = color!(0xDC, 0x26, 0x26); // Bright red

pub fn color_for_temp(temp: i32) -> Color {
    match temp {
        t if t >= 80 => TEMP_CRIT,
        t if t >= 70 => TEMP_HOT,
        t if t >= 60 => TEMP_WARM,
        _ => TEMP_COOL,
    }
}

fn chip_colors_for_temp(temp: i32) -> (Color, Color) {
    match temp {
        t if t >= 80 => (CHIP_BG_CRIT, CHIP_BORDER_CRIT),
        t if t >= 70 => (CHIP_BG_HOT, CHIP_BORDER_HOT),
        t if t >= 60 => (CHIP_BG_WARM, CHIP_BORDER_WARM),
        _ => (CHIP_BG_COOL, CHIP_BORDER_COOL),
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

pub fn chip_cell(temp: i32, errors: i32) -> container::Style {
    let (bg, border_color) = if errors > 0 {
        (CHIP_BG_ERROR, TEMP_CRIT)
    } else {
        chip_colors_for_temp(temp)
    };

    container::Style {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(bg)),
        border: Border {
            color: border_color,
            width: 1.5,
            radius: 4.0.into(),
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
