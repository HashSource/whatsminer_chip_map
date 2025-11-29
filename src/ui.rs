use iced::{
    Alignment, Element, Length, Point,
    widget::{
        Column, Row, Space, column, container, mouse_area, row, scrollable, text, tooltip,
        tooltip::Position,
    },
};

use crate::Message;
use crate::analysis::{self, ChipAnalysis};
use crate::config;
use crate::models::{Chip, ColorMode, MinerData, Slot, SystemInfo};
use crate::theme;

const CHIP_SIZE: f32 = 55.0; // Square aspect ratio
const CHIP_SPACING: u16 = 3;

pub fn miner_view<'a>(
    data: &'a MinerData,
    system_info: Option<&'a SystemInfo>,
    sidebar_width: f32,
    dragging: bool,
    color_mode: ColorMode,
) -> Element<'a, Message> {
    // Look up miner config based on model name for physical layout
    let miner_config = system_info.and_then(|info| config::lookup(&info.model));

    // Determine chips_per_domain (consistent across all slots for cross-slot comparison)
    let chips_per_domain = miner_config
        .map(|cfg| cfg.chips_per_domain as usize)
        .unwrap_or_else(|| {
            data.slots
                .first()
                .map(|s| infer_chips_per_domain(s.chips.len()))
                .unwrap_or(3)
        });

    // Compute cross-slot analysis for gradient/outlier/nonce modes
    let all_analysis = analysis::analyze_all_slots(&data.slots, chips_per_domain);

    let sidebar = sidebar(data, system_info, &all_analysis);

    let grids = data.slots.iter().zip(all_analysis.iter()).fold(
        Column::new().spacing(25).width(Length::Shrink),
        |col, (slot, slot_analysis)| {
            col.push(slot_grid(slot, color_mode, chips_per_domain, slot_analysis))
        },
    );

    let divider = mouse_area(
        container(text("⋮").size(14).center())
            .width(10)
            .height(Length::Fill)
            .center_x(Length::Shrink)
            .center_y(Length::Shrink)
            .style(|_| theme::divider_style()),
    )
    .on_press(Message::DividerDragStart)
    .on_release(Message::DividerDragEnd);

    let content: Element<'_, Message> = row![
        container(scrollable(sidebar).height(Length::Fill).width(Length::Fill))
            .width(sidebar_width)
            .height(Length::Fill)
            .style(|_| theme::sidebar_container()),
        divider,
        scrollable(grids.padding(15))
            .direction(iced::widget::scrollable::Direction::Both {
                vertical: iced::widget::scrollable::Scrollbar::default(),
                horizontal: iced::widget::scrollable::Scrollbar::default(),
            })
            .height(Length::Fill)
            .width(Length::Fill)
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into();

    if dragging {
        mouse_area(content)
            .on_move(|p: Point| Message::DividerDrag(p.x))
            .on_release(Message::DividerDragEnd)
            .into()
    } else {
        content
    }
}

fn sidebar<'a>(
    data: &'a MinerData,
    system_info: Option<&'a SystemInfo>,
    all_analysis: &[Vec<ChipAnalysis>],
) -> Column<'a, Message> {
    let mut col = Column::new().spacing(2).padding(5).width(Length::Fill);

    // System info section
    if let Some(info) = system_info {
        col = col
            .push(
                text("── System Info ──")
                    .size(13)
                    .color(theme::BRAND_ORANGE),
            )
            .push(text(&info.model).size(12))
            .push(text(&info.hardware_info).size(11))
            .push(text(format!("FW: {}", info.firmware_version)).size(11))
            .push(Space::with_height(8)); // spacer
    }

    for (slot_idx, slot) in data.slots.iter().enumerate() {
        col = col.push(
            text(format!("── Slot {} ──", slot.id))
                .size(13)
                .color(theme::BRAND_ORANGE),
        );

        let slot_analysis = all_analysis.get(slot_idx);

        for (chip_idx, chip) in slot.chips.iter().enumerate() {
            let nonce_deficit = slot_analysis
                .and_then(|a| a.get(chip_idx))
                .map_or(0.0, |a| a.nonce_deficit);

            col = col.push(
                row![
                    text(format!("C{:<3}", chip.id)).size(12),
                    text(format!("freq:{:<3}", chip.freq)).size(12),
                    text(format!("vol:{:<3}", chip.vol)).size(12),
                    text("temp:").size(12),
                    text(format!("{:<2}", chip.temp))
                        .size(12)
                        .color(theme::color_for_chip_temp(chip.temp)),
                    text("nonce:").size(12),
                    text(format!("{:<6}", chip.nonce))
                        .size(12)
                        .color(theme::color_for_nonce_deficit(nonce_deficit)),
                    text(format!(
                        "errors:{} crc:{} x:{} repeat:{} pct:{:.0}%/{:.0}%",
                        chip.errors, chip.crc, chip.x, chip.repeat, chip.pct1, chip.pct2,
                    ))
                    .size(12),
                ]
                .spacing(2),
            );
        }
    }

    col
}

/// Infer chips_per_domain from chip count using common domain sizes
fn infer_chips_per_domain(chip_count: usize) -> usize {
    // Common chips_per_domain values in WhatsMiner boards: 2, 3, 4, 5, 6
    // Pick the smallest that divides evenly and gives reasonable domain count
    for cpd in [3, 2, 4, 5, 6] {
        if chip_count.is_multiple_of(cpd) {
            let domains = chip_count / cpd;
            // Reasonable domain count: 20-80 for most boards
            if (20..=100).contains(&domains) {
                return cpd;
            }
        }
    }
    // Fallback for smaller boards or unusual counts
    for cpd in [2, 3, 4, 5, 6] {
        if chip_count.is_multiple_of(cpd) {
            return cpd;
        }
    }
    3 // Default fallback
}

fn slot_grid<'a>(
    slot: &'a Slot,
    color_mode: ColorMode,
    chips_per_domain: usize,
    analysis: &[ChipAnalysis],
) -> Element<'a, Message> {
    // Calculate domains (columns) for this slot
    let domains = if chips_per_domain > 0 {
        slot.chips.len().div_ceil(chips_per_domain)
    } else {
        1
    };

    // Calculate section split for layout info (must match chip_grid logic)
    // First domain sticks out, then split remaining in half
    let remaining = domains.saturating_sub(1);
    let bottom_domains = 1 + remaining / 2;
    let top_domains = remaining - (remaining / 2);

    let layout_info = format!(
        "{}d × {}c/d  [{}+{} snake]",
        domains, chips_per_domain, bottom_domains, top_domains
    );

    let header = row![
        text(format!("Slot {}", slot.id)).size(18),
        text(format!("{}MHz", slot.freq)).size(14),
        text(format!("{:.1}°C", slot.temp))
            .size(14)
            .color(theme::color_for_board_temp(slot.temp)),
        text(format!("{} chips", slot.chips.len())).size(14),
        text(layout_info).size(12),
    ]
    .spacing(20);

    container(
        column![
            header,
            chip_grid(&slot.chips, color_mode, chips_per_domain, analysis)
        ]
        .spacing(10),
    )
    .padding(15)
    .width(Length::Shrink)
    .style(|_| theme::slot_container())
    .into()
}

fn chip_grid<'a>(
    chips: &'a [Chip],
    color_mode: ColorMode,
    chips_per_domain: usize,
    analysis: &[ChipAnalysis],
) -> Column<'a, Message> {
    // Physical layout: chips are arranged in domains (vertical stacks)
    // Board is split into 2 sections with snake pattern
    let num_domains = if chips_per_domain > 0 {
        chips.len().div_ceil(chips_per_domain)
    } else {
        1
    };

    // Split into 2 sections (bottom/top halves of the physical board)
    // First domain sticks out from pattern, then split remaining in half
    // Bottom section = first domain + half of remaining
    let remaining = num_domains.saturating_sub(1);
    let bottom_domains = 1 + remaining / 2;
    let top_domains = remaining - (remaining / 2);

    let mut grid = Column::new()
        .spacing(CHIP_SPACING * 4)
        .width(Length::Shrink);

    // Top section first (displayed at top): domains bottom_domains to num_domains-1
    // Left to right for snake pattern continuing from bottom section
    if top_domains > 0 {
        let top_section = render_section(
            chips,
            color_mode,
            chips_per_domain,
            bottom_domains,
            num_domains,
            false, // left to right: continues from left after snake
            analysis,
        );
        grid = grid.push(top_section);
    }

    // Bottom section (displayed at bottom): domains 0 to bottom_domains-1
    // Right to left, D0/C0 at bottom-right corner
    let bottom_section = render_section(
        chips,
        color_mode,
        chips_per_domain,
        0,
        bottom_domains,
        true, // reversed: D0 on right
        analysis,
    );
    grid = grid.push(bottom_section);

    grid
}

/// Render a section of domains as rows of chips
fn render_section<'a>(
    chips: &'a [Chip],
    color_mode: ColorMode,
    chips_per_domain: usize,
    start_domain: usize,
    end_domain: usize,
    reversed: bool,
    analysis: &[ChipAnalysis],
) -> Column<'a, Message> {
    let domain_count = end_domain - start_domain;
    let mut section = Column::new().spacing(CHIP_SPACING).width(Length::Shrink);

    for row_idx in 0..chips_per_domain {
        let mut r = Row::new().spacing(CHIP_SPACING).width(Length::Shrink);

        for i in 0..domain_count {
            let domain_idx = if reversed {
                end_domain - 1 - i
            } else {
                start_domain + i
            };
            let chip_idx = domain_idx * chips_per_domain + row_idx;
            if chip_idx < chips.len() {
                let chip_analysis = analysis.get(chip_idx).copied();
                r = r.push(chip_cell(&chips[chip_idx], color_mode, chip_analysis));
            } else {
                r = r.push(Space::new(CHIP_SIZE, CHIP_SIZE));
            }
        }
        section = section.push(r);
    }

    section
}

fn chip_cell(
    chip: &Chip,
    color_mode: ColorMode,
    analysis: Option<ChipAnalysis>,
) -> Element<'_, Message> {
    let Chip {
        id,
        freq,
        vol,
        temp,
        errors,
        crc,
        x,
        repeat,
        ..
    } = *chip;

    let content = column![
        row![text(freq).size(10), text(vol).size(10)].spacing(6),
        text(temp).size(20),
        row![
            text(errors).size(9),
            text(crc).size(9),
            text(x).size(9),
            text(repeat).size(9)
        ]
        .spacing(3),
    ]
    .align_x(Alignment::Center)
    .spacing(1);

    let cell = container(content)
        .width(Length::Fixed(CHIP_SIZE))
        .height(Length::Fixed(CHIP_SIZE))
        .padding(2)
        .center_x(Length::Fixed(CHIP_SIZE))
        .center_y(Length::Fixed(CHIP_SIZE))
        .style(move |_| theme::chip_cell(temp, errors, crc, color_mode, analysis));

    tooltip(cell, text(format!("C{id}")).size(12), Position::Top)
        .gap(5)
        .style(|_| theme::tooltip_style())
        .into()
}
