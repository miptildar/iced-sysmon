use iced::widget::{column, row, scrollable, text};
use iced::{Element, Length};

use super::ui;
use crate::metrics::{CoreInfo, Snapshot};

pub fn view<'a, M: 'a>(s: &'a Snapshot) -> Element<'a, M> {
    let cpu = &s.cpu;

    let header = row![
        text("Core").width(Length::Fixed(120.0)).style(text::secondary),
        text("Load").width(Length::Fixed(120.0)).style(text::secondary),
        text("Frequency").style(text::secondary),
    ];

    let cores = column(cpu.cores.iter().map(core_row)).spacing(6);

    column![
        ui::title("CPU"),
        ui::field("Model", cpu.brand.clone()),
        ui::field("Usage", format!("{:.1} %", cpu.usage)),
        ui::field("Logical cores", cpu.cores.len().to_string()),
        ui::subtitle("Per cores"),
        header,
        scrollable(cores).height(Length::Fill),
    ]
        .spacing(10)
        .into()
}

fn core_row<'a, M: 'a>(core: &'a CoreInfo) -> Element<'a, M> {
    row![
        text(core.name.as_str()).width(Length::Fixed(120.0)),
        text(format!("{:.1} %", core.usage)).width(Length::Fixed(120.0)),
        text(format!("{} Mhz", core.frequency_mhz)),
    ]
        .into()
}