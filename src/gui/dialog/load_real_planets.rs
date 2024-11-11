use super::{Dialog, DialogUpdate};
use crate::gui::{gui_widget::PADDING, message::GuiMessage};
use iced::{
    widget::{Button, Column, Text},
    Alignment, Element, Length,
};

#[derive(Debug, Clone)]
pub(crate) struct LoadRealPlanetsDialog {}

impl LoadRealPlanetsDialog {
    pub(crate) fn new() -> Self {
        LoadRealPlanetsDialog {}
    }
}

impl Dialog for LoadRealPlanetsDialog {
    fn header(&self) -> String {
        "Load Real Planets".to_string()
    }

    fn body<'a>(&'a self) -> Element<'a, GuiMessage> {
        let warning = Text::new("This will overwrite all planets in the current system.");
        let submit_button = Button::new(Text::new("Submit")).on_press(GuiMessage::DialogSubmit);
        Column::new()
            .push(warning)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn update(&mut self, _event: DialogUpdate) {}

    fn submit(&self) -> GuiMessage {
        GuiMessage::LoadRealPlanets
    }
}
