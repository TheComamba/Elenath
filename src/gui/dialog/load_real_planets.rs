use super::Dialog;
use crate::gui::{gui_widget::PADDING, message::GuiMessage};
use iced::{
    widget::{component, Button, Column, Component, Text},
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

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewSystemDialogEvent {
    Submit,
}

impl Component<GuiMessage> for LoadRealPlanetsDialog {
    type State = ();

    type Event = NewSystemDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            NewSystemDialogEvent::Submit => Some(GuiMessage::LoadRealPlanets),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let warning = Text::new("This will overwrite all planets in the current system.");
        let submit_button = Button::new(Text::new("Submit")).on_press(NewSystemDialogEvent::Submit);
        Column::new()
            .push(warning)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
