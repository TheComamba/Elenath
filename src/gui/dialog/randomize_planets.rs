use super::Dialog;
use crate::gui::{gui_widget::PADDING, message::GuiMessage};
use iced::{
    widget::{component, Button, Column, Component, Text},
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

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RandomizePlanetsDialogEvent {
    Submit,
}

impl Component<GuiMessage> for RandomizePlanetsDialog {
    type State = ();

    type Event = RandomizePlanetsDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            RandomizePlanetsDialogEvent::Submit => Some(GuiMessage::RandomizePlanets),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let warning = Text::new("This will overwrite all planets in the current system.");
        let submit_button =
            Button::new(Text::new("Submit")).on_press(RandomizePlanetsDialogEvent::Submit);
        Column::new()
            .push(warning)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }
}
