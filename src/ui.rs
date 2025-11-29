use iced::{
    Alignment, Element, Length, Point,
    widget::{Column, Row, Space, column, container, mouse_area, row, scrollable, text},
};

use crate::Message;
use crate::models::{Chip, ColorMode, MinerData, Slot};
use crate::theme;

const CHIPS_PER_ROW: usize = 16;
const CHIP_SIZE: (f32, f32) = (85.0, 65.0);
const CHIP_SPACING: u16 = 3;

pub fn miner_view(
    data: &MinerData,
    sidebar_width: f32,
    dragging: bool,
    color_mode: ColorMode,
) -> Element<'_, Message> {
    let sidebar = sidebar(data);

    let grids = data.slots.iter().fold(
        Column::new().spacing(25).width(Length::Shrink),
        |col, slot| col.push(slot_grid(slot, color_mode)),
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

fn sidebar(data: &MinerData) -> Column<'_, Message> {
    let mut col = Column::new().spacing(2).padding(5).width(Length::Fill);

    for slot in &data.slots {
        col = col.push(
            text(format!("── Slot {} ──", slot.id))
                .size(13)
                .color(theme::BRAND_ORANGE),
        );

        for chip in &slot.chips {
            col = col.push(
                text(format!(
                    "C{:<3} freq:{:<3} vol:{:<3} temp:{:<2} nonce:{:<6} err:{} crc:{} x:{} repeat:{} pct:{:.1}%/{:.1}%",
                    chip.id, chip.freq, chip.vol, chip.temp, chip.nonce,
                    chip.errors, chip.crc, chip.x, chip.repeat, chip.pct1, chip.pct2,
                ))
                .size(12)
                .color(theme::color_for_chip_temp(chip.temp)),
            );
        }
    }

    col
}

fn slot_grid(slot: &Slot, color_mode: ColorMode) -> Element<'_, Message> {
    let header = row![
        text(format!("Slot {}", slot.id)).size(18),
        text(format!("{}MHz", slot.freq)).size(14),
        text(format!("{:.1}°C", slot.temp))
            .size(14)
            .color(theme::color_for_board_temp(slot.temp)),
        text(format!("{} chips", slot.chips.len())).size(14),
    ]
    .spacing(20);

    container(column![header, chip_grid(&slot.chips, color_mode)].spacing(10))
        .padding(15)
        .width(Length::Shrink)
        .style(|_| theme::slot_container())
        .into()
}

fn chip_grid(chips: &[Chip], color_mode: ColorMode) -> Column<'_, Message> {
    chips.chunks(CHIPS_PER_ROW).fold(
        Column::new().spacing(CHIP_SPACING).width(Length::Shrink),
        |col, row_chips| {
            let mut r = Row::new().spacing(CHIP_SPACING).width(Length::Shrink);
            for chip in row_chips {
                r = r.push(chip_cell(chip, color_mode));
            }
            // Pad incomplete rows
            for _ in row_chips.len()..CHIPS_PER_ROW {
                r = r.push(Space::new(CHIP_SIZE.0, CHIP_SIZE.1));
            }
            col.push(r)
        },
    )
}

fn chip_cell(chip: &Chip, color_mode: ColorMode) -> Element<'_, Message> {
    let Chip {
        freq,
        vol,
        temp,
        nonce,
        pct1,
        errors,
        crc,
        ..
    } = *chip;

    let content = column![
        row![text(freq).size(11), text(nonce).size(11)].spacing(8),
        text(temp).size(22),
        row![text(format!("{pct1:.1}%")).size(11), text(vol).size(11)].spacing(8),
    ]
    .align_x(Alignment::Center)
    .spacing(2);

    container(content)
        .width(CHIP_SIZE.0)
        .height(CHIP_SIZE.1)
        .padding(4)
        .center_x(Length::Shrink)
        .center_y(Length::Shrink)
        .style(move |_| theme::chip_cell(temp, errors, crc, color_mode))
        .into()
}
