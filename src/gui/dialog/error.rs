use super::Dialog;
use crate::gui::message::GuiMessage;
use astro_utils::error::AstroUtilError;
use iced::widget::{component, Component};
use iced::{
    widget::{Button, Column, Text},
    Element, Renderer,
};
use iced_aw::style::CardStyles;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) struct ErrorDialog {
    error_text: String,
}

#[derive(Debug)]
pub(crate) enum Error {
    AstroError(AstroUtilError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AstroError(err) => write!(f, "{}", err),
        }
    }
}

impl From<AstroUtilError> for Error {
    fn from(v: AstroUtilError) -> Self {
        Self::AstroError(v)
    }
}

impl ErrorDialog {
    pub(crate) fn new(error: Error) -> Self {
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

impl Component<GuiMessage, Renderer> for ErrorDialog {
    type State = ();

    type Event = ErrorDialogMes;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<GuiMessage> {
        Some(GuiMessage::DialogClosed)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let text = Text::new(self.error_text.clone());
        let button = Button::new(Text::new("Ok")).on_press(ErrorDialogMes::Close);
        Column::new().push(text).push(button).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ErrorDialogMes {
    Close,
}
