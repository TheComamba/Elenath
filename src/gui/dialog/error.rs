use super::Dialog;
use crate::error::ElenathError;
use crate::gui::message::GuiMessage;
use iced::widget::{component, Component};
use iced::{
    widget::{Button, Column, Text},
    Element,
};
use iced_aw::style::CardStyles;

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
    fn card_style(&self) -> CardStyles {
        CardStyles::Danger
    }

    fn header(&self) -> String {
        "Error".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

impl Component<GuiMessage> for ErrorDialog {
    type State = ();

    type Event = ErrorDialogMes;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<GuiMessage> {
        Some(GuiMessage::DialogClosed)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let text = Text::new(self.error_text.clone());
        let button = Button::new(Text::new("Ok")).on_press(ErrorDialogMes::Close);
        Column::new().push(text).push(button).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ErrorDialogMes {
    Close,
}
