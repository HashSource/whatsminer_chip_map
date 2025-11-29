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
use crate::i18n::{Language, Tr};
use crate::models::{Chip, ColorMode, MinerData, Slot, SystemInfo};
use crate::theme;

const CHIP_SIZE: f32 = 55.0; // Square aspect ratio
const CHIP_SPACING: f32 = 3.0;

/// Parse slot_link config string (e.g. "0:1 2:3") into pairs of linked slot indices
fn parse_slot_links(slot_link: &str) -> Vec<(usize, usize)> {
    slot_link
        .split_whitespace()
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.split(':').collect();
            if parts.len() == 2 {
                let a = parts[0].parse().ok()?;
                let b = parts[1].parse().ok()?;
                Some((a, b))
            } else {
                None
            }
        })
        .collect()
}

pub fn miner_view<'a>(
    data: &'a MinerData,
    system_info: Option<&'a SystemInfo>,
    sidebar_width: f32,
    dragging: bool,
    color_mode: ColorMode,
    lang: Language,
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

    // Check for linked slots (hydro/immersion models)
    let slot_links = miner_config
        .and_then(|cfg| cfg.slot_link)
        .map(parse_slot_links)
        .unwrap_or_default();

    let sidebar = sidebar(data, system_info, &all_analysis, &slot_links, lang);

    // Build grids - use linked display for hydro/immersion models, normal for others
    let grids = if !slot_links.is_empty() {
        // Hydro model: display linked slots side by side
        let mut col = Column::new().spacing(25).width(Length::Shrink);
        for (left_idx, right_idx) in &slot_links {
            if let (Some(left_slot), Some(right_slot)) =
                (data.slots.get(*left_idx), data.slots.get(*right_idx))
            {
                let left_analysis = all_analysis.get(*left_idx).map(|a| a.as_slice());
                let right_analysis = all_analysis.get(*right_idx).map(|a| a.as_slice());
                col = col.push(linked_slot_grid(
                    left_slot,
                    right_slot,
                    color_mode,
                    chips_per_domain,
                    left_analysis,
                    right_analysis,
                    lang,
                ));
            }
        }
        col
    } else {
        // Normal model: display slots individually
        data.slots.iter().zip(all_analysis.iter()).fold(
            Column::new().spacing(25).width(Length::Shrink),
            |col, (slot, slot_analysis)| {
                col.push(slot_grid(
                    slot,
                    color_mode,
                    chips_per_domain,
                    slot_analysis,
                    lang,
                ))
            },
        )
    };

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
    _slot_links: &[(usize, usize)],
    lang: Language,
) -> Column<'a, Message> {
    let mut col = Column::new().spacing(2).padding(5).width(Length::Fill);

    // System info section
    if let Some(info) = system_info {
        col = col
            .push(
                text(Tr::system_info(lang))
                    .size(13)
                    .color(theme::BRAND_ORANGE),
            )
            .push(text(&info.model).size(12))
            .push(text(&info.hardware_info).size(11))
            .push(text(format!("{}: {}", Tr::firmware(lang), info.firmware_version)).size(11))
            .push(Space::new().height(8)); // spacer
    }

    // Display all slots consistently
    for (slot_idx, slot) in data.slots.iter().enumerate() {
        col = col.push(
            text(format!("── {} {} ──", Tr::slot(lang), slot.id))
                .size(13)
                .color(theme::BRAND_ORANGE),
        );

        let slot_analysis = all_analysis.get(slot_idx);

        for (chip_idx, chip) in slot.chips.iter().enumerate() {
            let nonce_deficit = slot_analysis
                .and_then(|a| a.get(chip_idx))
                .map_or(0.0, |a| a.nonce_deficit);
            col = col.push(sidebar_chip_row(chip, nonce_deficit));
        }
    }

    col
}

fn sidebar_chip_row(chip: &Chip, nonce_deficit: f32) -> Column<'_, Message> {
    column![
        row![
            text(format!("C{}", chip.id)).size(12),
            text(format!("freq:{}", chip.freq)).size(12),
            text(format!("vol:{}", chip.vol)).size(12),
            text("temp:").size(12),
            text(format!("{}", chip.temp))
                .size(12)
                .color(theme::color_for_chip_temp(chip.temp)),
            text("nonce:").size(12),
            text(format!("{}", chip.nonce))
                .size(12)
                .color(theme::color_for_nonce_deficit(nonce_deficit)),
        ]
        .spacing(4),
        row![
            Space::new().width(12),
            text(format!(
                "err:{} crc:{} x:{} repeat:{} pct:{:.1}%/{:.1}%",
                chip.errors, chip.crc, chip.x, chip.repeat, chip.pct1, chip.pct2,
            ))
            .size(12),
        ]
    ]
    .spacing(0)
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
    lang: Language,
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
        text(format!("{} {}", Tr::slot(lang), slot.id)).size(18),
        text(format!("{}MHz", slot.freq)).size(14),
        text(format!("{:.1}°C", slot.temp))
            .size(14)
            .color(theme::color_for_board_temp(slot.temp)),
        text(format!("{} {}", slot.chips.len(), Tr::chips(lang))).size(14),
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

/// Render two linked slots stacked vertically (for hydro/immersion models)
/// Physical layout: slot 0 on top, slot 1 below (stacked hashboards)
fn linked_slot_grid<'a>(
    top_slot: &'a Slot,
    bottom_slot: &'a Slot,
    color_mode: ColorMode,
    chips_per_domain: usize,
    top_analysis: Option<&[ChipAnalysis]>,
    bottom_analysis: Option<&[ChipAnalysis]>,
    lang: Language,
) -> Element<'a, Message> {
    // Calculate domains for layout info
    let top_domains = if chips_per_domain > 0 {
        top_slot.chips.len().div_ceil(chips_per_domain)
    } else {
        1
    };
    let bottom_domains = if chips_per_domain > 0 {
        bottom_slot.chips.len().div_ceil(chips_per_domain)
    } else {
        1
    };

    // Header showing both linked slots
    let header = row![
        text(format!(
            "{} {}+{}",
            Tr::slot(lang),
            top_slot.id,
            bottom_slot.id
        ))
        .size(18),
        text(format!("{}MHz / {}MHz", top_slot.freq, bottom_slot.freq)).size(14),
        text(format!(
            "{:.1}°C / {:.1}°C",
            top_slot.temp, bottom_slot.temp
        ))
        .size(14)
        .color(theme::color_for_board_temp(
            (top_slot.temp + bottom_slot.temp) / 2.0
        )),
        text(format!(
            "{}+{} {}",
            top_slot.chips.len(),
            bottom_slot.chips.len(),
            Tr::chips(lang)
        ))
        .size(14),
        text(format!(
            "[{}d+{}d × {}c/d]",
            top_domains, bottom_domains, chips_per_domain
        ))
        .size(12),
    ]
    .spacing(20);

    // Build stacked chip grids (top slot above, bottom slot below)
    let top_grid = linked_chip_grid(
        &top_slot.chips,
        color_mode,
        chips_per_domain,
        top_analysis.unwrap_or(&[]),
    );

    let bottom_grid = linked_chip_grid(
        &bottom_slot.chips,
        color_mode,
        chips_per_domain,
        bottom_analysis.unwrap_or(&[]),
    );

    // Stack vertically: top slot label, top grid, divider, bottom slot label, bottom grid
    let stacked_grids = column![
        text(format!("{} {}", Tr::slot(lang), top_slot.id))
            .size(14)
            .color(theme::BRAND_ORANGE),
        top_grid,
        // Horizontal divider between the two stacked boards
        container(Space::new().height(3)).style(|_| theme::linked_divider_style()),
        text(format!("{} {}", Tr::slot(lang), bottom_slot.id))
            .size(14)
            .color(theme::BRAND_ORANGE),
        bottom_grid,
    ]
    .spacing(8);

    container(column![header, stacked_grids].spacing(10))
        .padding(15)
        .width(Length::Shrink)
        .style(|_| theme::slot_container())
        .into()
}

/// Render chip grid for linked slot display
/// For hydro/immersion models: NO snake pattern, simple left/right split
/// - Right side: first half of domains (D0 at far right)
/// - Left side: second half of domains (also D0-ward on right)
/// Both sections display domains right-to-left (lowest domain index on right)
fn linked_chip_grid<'a>(
    chips: &'a [Chip],
    color_mode: ColorMode,
    chips_per_domain: usize,
    analysis: &[ChipAnalysis],
) -> Column<'a, Message> {
    let num_domains = if chips_per_domain > 0 {
        chips.len().div_ceil(chips_per_domain)
    } else {
        1
    };

    // Split domains in half: right side gets first half, left side gets second half
    let right_domains = (num_domains + 1) / 2; // D0 through D(mid-1) on right
    let left_domains = num_domains - right_domains; // D(mid) through D(last) on left

    let mut grid = Column::new()
        .spacing(CHIP_SPACING * 4.0)
        .width(Length::Shrink);

    // Top visual section: RIGHT side of board (D0 at far right, C0 at bottom-right)
    // Domains displayed right-to-left so D0 is on the far right
    let right_section = render_linked_section(
        chips,
        color_mode,
        chips_per_domain,
        0,
        right_domains,
        true, // reversed: D0 on far right
        analysis,
    );
    grid = grid.push(right_section);

    // Bottom visual section: LEFT side of board (higher domain numbers)
    // Last chip should be at top-right, so use normal row order (not reversed)
    // Domains displayed left-to-right so highest domain (last chip) is on the right
    if left_domains > 0 {
        let left_section = render_section(
            chips,
            color_mode,
            chips_per_domain,
            right_domains, // start from middle
            num_domains,   // to end
            false,         // not reversed: highest domain index on right
            analysis,
        );
        grid = grid.push(left_section);
    }

    grid
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
        .spacing(CHIP_SPACING * 4.0)
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

/// Render a section of domains as rows of chips (top-to-bottom row order)
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
                r = r.push(Space::new().width(CHIP_SIZE).height(CHIP_SIZE));
            }
        }
        section = section.push(r);
    }

    section
}

/// Render a section for linked slots (bottom-to-top row order: C0 at bottom)
fn render_linked_section<'a>(
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

    // Render rows in reverse order: highest row_idx first (top), row_idx=0 last (bottom)
    for row_idx in (0..chips_per_domain).rev() {
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
                r = r.push(Space::new().width(CHIP_SIZE).height(CHIP_SIZE));
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
