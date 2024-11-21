use super::Dialog;
use crate::gui::{gui_widget::PADDING, message::GuiMessage};
use iced::{
    widget::{Button, Column, Text},
    Alignment, Element, Length,
};

#[derive(Debug, Clone)]
pub(crate) struct RandomizePlanetsDialog {}

impl RandomizePlanetsDialog {
    pub(crate) fn new() -> Self {
        RandomizePlanetsDialog {}
    }
}

impl Dialog for RandomizePlanetsDialog {
    fn header(&self) -> String {
        "Randomize Planets".to_string()
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

    fn update(&mut self, _message: super::DialogUpdate) {}

    fn on_submit(&self) -> GuiMessage {
        GuiMessage::RandomizePlanets
    }

    fn get_error(&self) -> Option<super::ElenathError> {
        None
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RandomizePlanetsDialogEvent {
    Submit,
}
