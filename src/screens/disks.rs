use iced::widget::{column, scrollable, text};
use iced::{Element, Length};

use super::ui;
use crate::format;
use crate::metrics::{DiskInfo, Snapshot};

pub fn view<'a, M: 'a>(s: &'a Snapshot) -> Element<'a, M> {
    let content: Element<'a, M> = if s.disks.is_empty() {
        text("Disks not found").into()
    } else {
        column(s.disks.iter().map(disk_block)).spacing(20).into()
    };

    column![ui::title("Disks"), scrollable(content).height(Length::Fill)]
        .spacing(10)
        .into()
}

fn disk_block<'a, M: 'a>(d: &'a DiskInfo) -> Element<'a, M> {
    let used = d.total.saturating_sub(d.available);
    let used_pct = format::percent(used, d.total);

    column![
        ui::subtitle(d.mount.as_str()),
        ui::field("Device", d.name.clone()),
        ui::field("File system", d.fs.clone()),
        ui::field("Volume", format::bytes(d.total)),
        ui::field("Free", format::bytes(d.available)),
        ui::field("Occupied", format!("{used_pct:.1} %")),
    ]
        .spacing(6)
        .into()
}