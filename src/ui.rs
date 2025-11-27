use iced::{
    Element, Length, Point,
    widget::{Column, Row, column as col, container, mouse_area, row, scrollable, text},
};

use crate::Message;
use crate::models::{Chip, MinerData, Slot};
use crate::theme;

const CHIPS_PER_ROW: usize = 16;
const CHIP_WIDTH: u16 = 85;
const CHIP_HEIGHT: u16 = 65;
const CHIP_SPACING: u16 = 3;

pub fn render_miner_view(
    data: &MinerData,
    sidebar_width: f32,
    dragging: bool,
) -> Element<'_, Message> {
    let sidebar = render_sidebar(data);

    let grids = data.slots.iter().fold(
        Column::new().spacing(25).width(Length::Shrink),
        |col, slot| col.push(render_slot_grid(slot)),
    );

    // Divider with drag handle
    let divider: Element<'_, Message> = mouse_area(
        container(text("⋮").size(14).center())
            .width(10)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(|_| theme::divider_style()),
    )
    .on_press(Message::DividerDragStart)
    .on_release(Message::DividerDragEnd)
    .into();

    let main_content: Element<'_, Message> = row![
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
    .spacing(0)
    .width(Length::Fill)
    .height(Length::Fill)
    .into();

    // Outer mouse_area to track movement during drag
    if dragging {
        mouse_area(main_content)
            .on_move(|point: Point| Message::DividerDrag(point.x))
            .on_release(Message::DividerDragEnd)
            .into()
    } else {
        main_content
    }
}

fn render_sidebar(data: &MinerData) -> Column<'_, Message> {
    let mut sidebar = Column::new().spacing(2).padding(5).width(Length::Fill);

    for slot in &data.slots {
        // Slot header
        sidebar = sidebar.push(
            text(format!("── Slot {} ──", slot.id))
                .size(13)
                .color(theme::BRAND_ORANGE),
        );

        for chip in &slot.chips {
            let line = format!(
                "C{:<3} freq:{:<3} vol:{:<3} temp:{:<2} nonce:{:<6} err:{} crc:{} x:{} repeat:{} pct:{:.1}%/{:.1}%",
                chip.id,
                chip.freq,
                chip.vol,
                chip.temp,
                chip.nonce,
                chip.errors,
                chip.crc,
                chip.x,
                chip.repeat,
                chip.pct1,
                chip.pct2,
            );
            sidebar = sidebar.push(text(line).size(12).color(theme::color_for_temp(chip.temp)));
        }
    }

    sidebar
}

fn render_slot_grid(slot: &Slot) -> Element<'_, Message> {
    let header = row![
        text(format!("Slot {}", slot.id)).size(18),
        text(format!("{}MHz", slot.freq)).size(14),
        text(format!("{:.1}°C", slot.temp)).size(14),
        text(format!("{} chips", slot.chips.len())).size(14),
    ]
    .spacing(20);

    let grid = render_chip_grid(&slot.chips);

    container(col![header, grid].spacing(10).width(Length::Shrink))
        .padding(15)
        .width(Length::Shrink)
        .style(|_| theme::slot_container())
        .into()
}

fn render_chip_grid(chips: &[Chip]) -> Column<'_, Message> {
    chips.chunks(CHIPS_PER_ROW).fold(
        Column::new().spacing(CHIP_SPACING).width(Length::Shrink),
        |col, row_chips| {
            let mut r = Row::new().spacing(CHIP_SPACING).width(Length::Shrink);
            for chip in row_chips {
                r = r.push(render_chip(chip));
            }
            // Pad incomplete rows with empty spacers
            for _ in row_chips.len()..CHIPS_PER_ROW {
                r = r.push(iced::widget::Space::new(
                    Length::Fixed(CHIP_WIDTH as f32),
                    Length::Fixed(CHIP_HEIGHT as f32),
                ));
            }
            col.push(r)
        },
    )
}

fn render_chip(chip: &Chip) -> Element<'_, Message> {
    let (temp, errors) = (chip.temp, chip.errors);

    let content = col![
        // Top row: freq (left) and nonce (right)
        row![
            text(format!("{}", chip.freq)).size(11),
            text(format!("{}", chip.nonce)).size(11),
        ]
        .spacing(8),
        // Center: temperature (large)
        text(format!("{}", chip.temp)).size(22),
        // Bottom row: pct (left) and vol (right)
        row![
            text(format!("{:.1}%", chip.pct1)).size(11),
            text(format!("{}", chip.vol)).size(11),
        ]
        .spacing(8),
    ]
    .align_x(iced::Alignment::Center)
    .spacing(2);

    container(content)
        .width(Length::Fixed(CHIP_WIDTH as f32))
        .height(Length::Fixed(CHIP_HEIGHT as f32))
        .padding(4)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(move |_| theme::chip_cell(temp, errors))
        .into()
}
