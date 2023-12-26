use iced::{widget::Column, Length, Sandbox};

pub(crate) struct Gui {}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui {}
    }

    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GuiMessage {}
