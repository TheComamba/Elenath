use self::topview::TopViewState;
use crate::model::{celestial_body::CelestialBody, example::solar_system_example};
use iced::{widget::canvas, Sandbox};

mod topview;

pub(crate) struct Gui {
    topview_state: TopViewState,
    celestial_bodies: Vec<CelestialBody>,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        let celestial_bodies = solar_system_example();
        Gui {
            topview_state: TopViewState::new(celestial_bodies.clone()),
            celestial_bodies,
        }
    }

    fn title(&self) -> String {
        String::from("Elenath - Imaginary Skies")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        canvas(&self.topview_state)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GuiMessage {}
