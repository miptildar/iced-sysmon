use iced::widget::{column, row, scrollable, text};
use iced::{Element, Length};

use super::ui;
use crate::format;
use crate::metrics::{NetInfo, Snapshot};

pub fn view<'a, M: 'a>(s: &'a Snapshot) -> Element<'a, M> {
    let header = row![
        text("Interface").width(Length::FillPortion(2)).style(text::secondary),
        text("Received").width(Length::FillPortion(2)).style(text::secondary),
        text("Transferred").width(Length::FillPortion(2)).style(text::secondary),
        text("Total received").width(Length::FillPortion(2)).style(text::secondary),
        text("Total transferred").width(Length::FillPortion(2)).style(text::secondary),
    ];

    let rows = column(s.networks.iter().map(net_row)).spacing(6);

    column![ui::title("Network"), header, scrollable(rows).height(Length::Fill)]
        .spacing(10)
        .into()
}

fn net_row<'a, M: 'a>(n: &'a NetInfo) -> Element<'a, M> {
    row![
        text(n.name.as_str()).width(Length::FillPortion(2)),
        text(format::rate(n.rx_per_sec)).width(Length::FillPortion(2)),
        text(format::rate(n.tx_per_sec)).width(Length::FillPortion(2)),
        text(format::bytes(n.total_rx)).width(Length::FillPortion(2)),
        text(format::bytes(n.total_tx)).width(Length::FillPortion(2)),
    ]
        .into()
}