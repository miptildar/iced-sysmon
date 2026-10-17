use iced::Element;
use iced::widget::column;

use super::ui;
use crate::format;
use crate::metrics::Snapshot;

pub fn view<'a, M: 'a>(s: &'a Snapshot) -> Element<'a, M> {
    let m = &s.memory;
    let used_pct = format::percent(m.used, m.total);
    let swap_pct = format::percent(m.swap_used, m.swap_total);

    column![
        ui::title("Memory"),
        ui::subtitle("RAM"),
        ui::field("All", format::bytes(m.total)),
        ui::field("Used", format!("{} ({used_pct:.1} %)", format::bytes(m.used))),
        ui::field("Available", format::bytes(m.available)),
        ui::subtitle("Swap"),
        ui::field("Total", format::bytes(m.swap_total)),
        ui::field("Taken", format!("{} ({swap_pct:.1} %)", format::bytes(m.swap_used))),
    ]
        .spacing(10)
        .into()
}