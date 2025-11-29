use iced::{Background, Border, Color, color, widget::container};

use crate::models::ColorMode;

// WhatsMiner brand colors
pub const BRAND_ORANGE: Color = color!(0xF7, 0x93, 0x1A);

// Base colors
const BG_DARK: Color = color!(0x0D, 0x0D, 0x0D);
const BG_PANEL: Color = color!(0x1A, 0x1A, 0x1A);
const BORDER_SUBTLE: Color = color!(0x3A, 0x3A, 0x3A);
const BORDER_ACCENT: Color = color!(0x4A, 0x4A, 0x4A);

// =============================================================================
// Temperature Thresholds (from WhatsMiner firmware analysis)
// =============================================================================
//
// CHIP temperatures (individual ASIC chips):
//   - ft_chip_temp_warn:    95-105°C  (warning threshold)
//   - chip_temp_protect:    110-120°C (shutdown threshold)
//   - Normal operating range is up to 95°C
//
// BOARD temperatures (hash board overall):
//   - cool_temp:            30-38°C   (target operating temp)
//   - board_temp_overheat:  80-90°C   (overheat warning)
//   - board_temp_protect:   80-90°C   (shutdown threshold)
//
// =============================================================================

// Chip temperature thresholds (°C) - from firmware chip_temp values
const CHIP_TEMP_COOL: i32 = 70; // Below this = cool (ideal)
const CHIP_TEMP_WARM: i32 = 85; // Below this = warm (normal operation)
const CHIP_TEMP_HOT: i32 = 95; // Below this = hot (approaching ft_chip_temp_warn)
// >= 95 = critical (at/above warning threshold)

// Board temperature thresholds (°C) - from firmware board_temp values
const BOARD_TEMP_COOL: i32 = 50; // Below this = cool (well under cool_temp target)
const BOARD_TEMP_WARM: i32 = 65; // Below this = warm (normal operation)
const BOARD_TEMP_HOT: i32 = 80; // Below this = hot (approaching board_temp_overheat)
// >= 80 = critical (at overheat/protect threshold)

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

// Chip border colors
const CHIP_BORDER_COOL: Color = color!(0x22, 0xC5, 0x5E); // Bright green
const CHIP_BORDER_WARM: Color = color!(0xF5, 0x9E, 0x0B); // Bright amber
const CHIP_BORDER_HOT: Color = color!(0xEA, 0x58, 0x0C); // Bright orange
const CHIP_BORDER_CRIT: Color = color!(0xDC, 0x26, 0x26); // Bright red

/// Returns a color for chip temperature (individual ASIC)
/// Based on WhatsMiner firmware thresholds:
/// - Cool:     < 70°C  (ideal operation)
/// - Warm:     70-85°C (normal operation)
/// - Hot:      85-95°C (approaching warning)
/// - Critical: >= 95°C (at ft_chip_temp_warn threshold)
pub fn color_for_chip_temp(temp: i32) -> Color {
    match temp {
        t if t >= CHIP_TEMP_HOT => TEMP_CRIT,
        t if t >= CHIP_TEMP_WARM => TEMP_HOT,
        t if t >= CHIP_TEMP_COOL => TEMP_WARM,
        _ => TEMP_COOL,
    }
}

/// Returns a color for board temperature (hash board overall)
/// Based on WhatsMiner firmware thresholds:
/// - Cool:     < 50°C  (well under target)
/// - Warm:     50-65°C (normal operation)
/// - Hot:      65-80°C (approaching overheat)
/// - Critical: >= 80°C (at board_temp_overheat threshold)
pub fn color_for_board_temp(temp: f64) -> Color {
    let t = temp as i32;
    match t {
        t if t >= BOARD_TEMP_HOT => TEMP_CRIT,
        t if t >= BOARD_TEMP_WARM => TEMP_HOT,
        t if t >= BOARD_TEMP_COOL => TEMP_WARM,
        _ => TEMP_COOL,
    }
}

/// Returns background and border colors for chip cell based on temperature
fn chip_colors_for_temp(temp: i32) -> (Color, Color) {
    match temp {
        t if t >= CHIP_TEMP_HOT => (CHIP_BG_CRIT, CHIP_BORDER_CRIT),
        t if t >= CHIP_TEMP_WARM => (CHIP_BG_HOT, CHIP_BORDER_HOT),
        t if t >= CHIP_TEMP_COOL => (CHIP_BG_WARM, CHIP_BORDER_WARM),
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

// Error thresholds for color coding
// Small error counts are normal in mining ASICs
const ERROR_LOW: i32 = 20; // Below this = cool (few errors)
const ERROR_MED: i32 = 50; // Below this = warm (moderate errors)
const ERROR_HIGH: i32 = 100; // Below this = hot (elevated errors)
// >= 100 = critical (high errors)

// CRC error thresholds (CRC errors are more serious)
const CRC_LOW: i32 = 1; // Any CRC = warm
const CRC_MED: i32 = 5; // >= 5 = hot
const CRC_HIGH: i32 = 10; // >= 10 = critical

/// Returns background and border colors for chip cell based on error count
fn chip_colors_for_errors(errors: i32) -> (Color, Color) {
    match errors {
        e if e >= ERROR_HIGH => (CHIP_BG_CRIT, CHIP_BORDER_CRIT),
        e if e >= ERROR_MED => (CHIP_BG_HOT, CHIP_BORDER_HOT),
        e if e >= ERROR_LOW => (CHIP_BG_WARM, CHIP_BORDER_WARM),
        _ => (CHIP_BG_COOL, CHIP_BORDER_COOL),
    }
}

/// Returns background and border colors for chip cell based on CRC errors
fn chip_colors_for_crc(crc: i32) -> (Color, Color) {
    match crc {
        c if c >= CRC_HIGH => (CHIP_BG_CRIT, CHIP_BORDER_CRIT),
        c if c >= CRC_MED => (CHIP_BG_HOT, CHIP_BORDER_HOT),
        c if c >= CRC_LOW => (CHIP_BG_WARM, CHIP_BORDER_WARM),
        _ => (CHIP_BG_COOL, CHIP_BORDER_COOL),
    }
}

pub fn chip_cell(temp: i32, errors: i32, crc: i32, mode: ColorMode) -> container::Style {
    // Choose colors based on selected mode
    let (bg, border_color) = match mode {
        ColorMode::Temperature => chip_colors_for_temp(temp),
        ColorMode::Errors => chip_colors_for_errors(errors),
        ColorMode::Crc => chip_colors_for_crc(crc),
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
