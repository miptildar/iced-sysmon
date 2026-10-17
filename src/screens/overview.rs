use iced::Element;
use iced::widget::column;

use super::ui;
use crate::format;
use crate::metrics::Snapshot;

pub fn view<'a, M: 'a>(s: &'a Snapshot) -> Element<'a, M> {
    let os = &s.os;

    column![
        ui::title("Overview"),
        ui::field("System", os.name.clone()),
        ui::field("Version", os.version.clone()),
        ui::field("Kernel", os.kernel.clone()),
        ui::field("Host", os.host.clone()),
        ui::field("Architecture", os.arch.clone()),
        ui::field("Uptime", format::uptime(os.uptime_secs)),
        ui::field("Processes", s.processes.len().to_string()),
    ]
        .spacing(10)
        .into()
}