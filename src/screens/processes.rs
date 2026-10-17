use iced::widget::{button, column, row, scrollable, text, text_input};
use iced::{Element, Length};

use super::ui;
use crate::format;
use crate::metrics::{ProcessInfo, Snapshot};

const LIMIT: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Pid,
    Name,
    Cpu,
    Memory,
}

#[derive(Debug, Clone)]
pub enum Message {
    FilterChanged(String),
    SortBy(SortKey),
}

#[derive(Debug, Clone)]
pub struct State {
    filter: String,
    sort: SortKey,
    descending: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            filter: String::new(),
            sort: SortKey::Cpu,
            descending: true,
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::FilterChanged(filter) => self.filter = filter,
            Message::SortBy(key) if key == self.sort => self.descending = !self.descending,
            Message::SortBy(key) => {
                self.sort = key;
                self.descending = matches!(key, SortKey::Cpu | SortKey::Memory);
            }
        }
    }

    pub fn visible<'a>(&self, all: &'a [ProcessInfo]) -> Vec<&'a ProcessInfo> {
        let needle = self.filter.trim().to_lowercase();

        let mut rows: Vec<&ProcessInfo> = all
            .iter()
            .filter(|p| needle.is_empty() || p.name.to_lowercase().contains(&needle))
            .collect();

        rows.sort_by(|a, b| {
            let order = match self.sort {
                SortKey::Pid => a.pid.cmp(&b.pid),
                SortKey::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortKey::Cpu => a.cpu.total_cmp(&b.cpu),
                SortKey::Memory => a.memory.cmp(&b.memory),
            };
            if self.descending { order.reverse() } else { order }
        });

        rows.truncate(LIMIT);
        rows
    }

    pub fn view<'a>(&'a self, snapshot: &'a Snapshot) -> Element<'a, Message> {
        let rows = self.visible(&snapshot.processes);
        let shown = rows.len();

        let header = row![
            self.header_cell("PID", SortKey::Pid, Length::Fixed(80.0)),
            self.header_cell("Name", SortKey::Name, Length::Fill),
            self.header_cell("CPU, %", SortKey::Cpu, Length::Fixed(90.0)),
            self.header_cell("Memory", SortKey::Memory, Length::Fixed(110.0)),
        ]
            .spacing(8);

        let body = column(rows.into_iter().map(process_row)).spacing(4);

        column![
            ui::title("Processes"),
            text_input("Filter by name", &self.filter).on_input(Message::FilterChanged),
            text(format!("Shown {shown} out of {}", snapshot.processes.len())).size(13),
            header,
            scrollable(body).height(Length::Fill),
        ]
            .spacing(12)
            .into()
    }

    fn header_cell(&self, label: &str, key: SortKey, width: Length) -> Element<'static, Message> {
        let arrow = match (self.sort == key, self.descending) {
            (true, true) => " ▼",
            (true, false) => " ▲",
            (false, _) => "",
        };

        button(text(format!("{label}{arrow}")))
            .on_press(Message::SortBy(key))
            .style(button::text)
            .padding(0)
            .width(width)
            .into()
    }
}

fn process_row(p: &ProcessInfo) -> Element<'_, Message> {
    row![
        text(p.pid).width(Length::Fixed(80.0)),
        text(p.name.as_str()).width(Length::Fill),
        text(format!("{:.1}", p.cpu)).width(Length::Fixed(90.0)),
        text(format::bytes(p.memory)).width(Length::Fixed(110.0)),
    ]
        .spacing(8)
        .into()
}
