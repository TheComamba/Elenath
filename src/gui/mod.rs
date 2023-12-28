use self::topview::TopViewState;
use crate::model::{celestial_body::CelestialBodyData, example::sun};
use astro_utils::units::time::Time;
use iced::{widget::canvas, Sandbox};

mod topview;

pub(crate) struct Gui {
    topview_state: TopViewState,
    central_body_data: CelestialBodyData,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        let central_body_data = sun();
        Gui {
            topview_state: TopViewState::new(central_body_data.system(Time::from_days(0.0))),
            central_body_data,
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
