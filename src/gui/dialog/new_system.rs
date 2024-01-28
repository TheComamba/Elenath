use super::Dialog;
use crate::{gui::gui_widget::GuiMessage, model::celestial_system::SystemType};
use iced::{
    widget::{component, Column, Component},
    Element, Renderer,
};

#[derive(Default, Debug, Clone)]
pub(crate) struct NewSystemDialog {}

#[derive(Debug, Clone)]
pub(crate) struct NewSystemDialogState {
    system_type: SystemType,
}

impl Dialog for NewSystemDialog {
    fn header(&self) -> String {
        String::from("New System")
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

impl Default for NewSystemDialogState {
    fn default() -> Self {
        Self {
            system_type: SystemType::Real,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewSystemDialogEvent {
    SystemTypeSelected(SystemType),
}

impl Component<GuiMessage, Renderer> for NewSystemDialog {
    type State = NewSystemDialogState;

    type Event = NewSystemDialogEvent;

    fn update(&mut self, state: &mut Self::State, _message: Self::Event) -> Option<GuiMessage> {
        todo!()
    }

    fn view(&self, _state: &Self::State) -> iced::Element<'_, Self::Event> {
        Column::new().into()
    }
}
