use super::{CardStyle, Dialog, DialogUpdate};
use crate::error::ElenathError;
use crate::gui::message::GuiMessage;
use iced::{
    widget::{Button, Column, Text},
    Element,
};

#[derive(Debug, Clone)]
pub(crate) struct ErrorDialog {
    error_text: String,
}

impl ErrorDialog {
    pub(crate) fn new(error: ElenathError) -> Self {
        ErrorDialog {
            error_text: error.to_string(),
        }
    }
}

impl Dialog for ErrorDialog {
    fn card_style(&self) -> CardStyle {
        CardStyle::Error
    }

    fn header(&self) -> String {
        "Error".to_string()
    }

    fn update(&mut self, _event: DialogUpdate) {}

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        let text = Text::new(self.error_text.clone());
        let button = Button::new(Text::new("Ok")).on_press(GuiMessage::DialogClosed);
        Column::new().push(text).push(button).into()
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::DialogClosed
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ErrorDialogMes {
    Close,
}
