use crate::model::{celestial_body::CelestialBody, example::solar_system_example};
use iced::{widget::Column, Length, Sandbox};

pub(crate) struct Gui {
    celestial_bodies: Vec<CelestialBody>,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui {
            celestial_bodies: solar_system_example(),
        }
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
