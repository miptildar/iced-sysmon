mod metrics;
mod format;
mod poller;
mod screens;

use std::sync::Arc;

use iced::widget::{button, column, container, row, text, toggler};
use iced::{Element, Length, Size, Subscription, Theme};

use metrics::Snapshot;
use screens::{cpu, disks, memory, network, overview, processes};

const INTERVALS_MS: [u64; 3] = [500, 1000, 2000];

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("SysMon")
        .theme(App::theme)
        .subscription(App::subscription)
        .window_size(Size::new(960.0, 620.0))
        .centered()
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Overview,
    Cpu,
    Memory,
    Processes,
    Disks,
    Network,
}

impl Page {
    const ALL: [Page; 6] = [
        Page::Overview,
        Page::Cpu,
        Page::Memory,
        Page::Processes,
        Page::Disks,
        Page::Network,
    ];

    fn title(self) -> &'static str {
        match self {
            Page::Overview => "Overview",
            Page::Cpu => "CPU",
            Page::Memory => "RAM",
            Page::Processes => "Processes",
            Page::Disks => "Disks",
            Page::Network => "Network",
        }
    }
}

struct App {
    page: Page,
    snapshot: Option<Arc<Snapshot>>,
    processes: processes::State,
    interval_ms: u64,
    paused: bool,
    error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            page: Page::Overview,
            snapshot: None,
            processes: processes::State::default(),
            interval_ms: 1000,
            paused: false,
            error: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    PageSelected(Page),
    Poll(poller::Event),
    Processes(processes::Message),
    IntervalChosen(u64),
    PausedToggled(bool),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::PageSelected(page) => self.page = page,
            Message::Poll(poller::Event::Snapshot(snapshot)) => {
                self.snapshot = Some(snapshot);
                self.error = None;
            }
            Message::Poll(poller::Event::Failed(error)) => self.error = Some(error),
            Message::Processes(message) => self.processes.update(message),
            Message::IntervalChosen(ms) => self.interval_ms = ms,
            Message::PausedToggled(paused) => self.paused = paused,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.paused {
            Subscription::none()
        } else {
            Subscription::run_with(self.interval_ms, poller::run)
                .map(|event| Message::Poll(event))
        }
    }

    fn view(&self) -> Element<'_, Message> {
        row![self.sidebar(), self.content()].into()
    }

    fn sidebar(&self) -> Element<'_, Message> {
        let menu = column(Page::ALL.iter().map(|&page| {
            let selected = page == self.page;
            button(text(page.title()))
                .width(Length::Fill)
                .on_press(Message::PageSelected(page))
                .style(move |theme: &Theme, status| {
                    if selected {
                        button::primary(theme, status)
                    } else {
                        button::text(theme, status)
                    }
                })
                .into()
        }))
            .spacing(4);

        let intervals = row(INTERVALS_MS.iter().map(|&ms| {
            let selected = ms == self.interval_ms;
            button(text(format!("{} с", ms as f64 / 1000.0)).size(13))
                .on_press(Message::IntervalChosen(ms))
                .style(move |theme: &Theme, status| {
                    if selected {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    }
                })
                .into()
        }))
            .spacing(4);

        let controls = column![
            toggler(self.paused).label("Pause").on_toggle(Message::PausedToggled),
            text("Poll interval").size(13),
            intervals,
        ]
            .spacing(8);

        let sidebar = column![
            text("SysMon").size(22),
            menu,
            container(controls).height(Length::Fill).align_bottom(Length::Fill),
        ]
            .spacing(16)
            .padding(16)
            .width(Length::Fixed(210.0));

        container(sidebar)
            .height(Length::Fill)
            .style(container::bordered_box)
            .into()
    }

    fn content(&self) -> Element<'_, Message> {
        let body: Element<'_, Message> = match &self.snapshot {
            None => text("Getting data…").into(),
            Some(s) => match self.page {
                Page::Overview => overview::view(s),
                Page::Cpu => cpu::view(s),
                Page::Memory => memory::view(s),
                Page::Processes => self.processes.view(s).map(Message::Processes),
                Page::Disks => disks::view(s),
                Page::Network => network::view(s),
            },
        };

        let status = match (&self.error, self.paused) {
            (Some(error), _) => format!("Ошибка опроса: {error}"),
            (None, true) => "Pull paused".to_owned(),
            (None, false) => format!("Updating once in {} ms", self.interval_ms),
        };

        column![
            container(body).height(Length::Fill),
            text(status).size(13).style(text::secondary),
        ]
            .spacing(8)
            .padding(20)
            .width(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Nord
    }
}


