use iced::widget::{row, text};
use iced::{Element, Length};

pub fn title<'a, M: 'a>(label: &'a str) -> Element<'a, M> {
    text(label).size(24).into()
}

pub fn subtitle<'a, M: 'a>(label: &'a str) -> Element<'a, M> {
    text(label).size(17).into()
}

/// Строка «подпись: значение».
pub fn field<'a, M: 'a>(label: &'a str, value: String) -> Element<'a, M> {
    row![
        text(label).width(Length::Fixed(200.0)).style(text::secondary),
        text(value),
    ]
        .spacing(12)
        .into()
}