use super::Dialog;
use crate::gui::{gui_widget::PADDING, message::GuiMessage};
use iced::{
    widget::{Button, Column, Text},
    Alignment, Element, Length,
};

#[derive(Debug, Clone)]
pub(crate) struct NewSystemDialog {}

impl NewSystemDialog {
    pub(crate) fn new() -> Self {
        NewSystemDialog {}
    }
}

impl Dialog for NewSystemDialog {
    fn header(&self) -> String {
        "Create new Celestial System".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        let warning = Text::new("This will overwrite the current celestial system.");
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

    fn submit(&self) -> GuiMessage {
        GuiMessage::NewSystem
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewSystemDialogEvent {
    Submit,
}
