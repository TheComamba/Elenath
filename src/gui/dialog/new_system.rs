use super::Dialog;
use crate::{gui::gui_widget::GuiMessage, model::celestial_system::SystemType};
use iced::{
    widget::{component, Column, Component, Radio},
    Element, Renderer,
};

#[derive(Debug, Clone)]
pub(crate) struct NewSystemDialog {
    system_type: SystemType,
}

impl NewSystemDialog {
    pub(crate) fn new() -> Self {
        NewSystemDialog {
            system_type: SystemType::Real,
        }
    }
}

impl Dialog for NewSystemDialog {
    fn header(&self) -> String {
        "Load/Generate Celestial System".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewSystemDialogEvent {
    SystemTypeSelected(SystemType),
}

impl Component<GuiMessage, Renderer> for NewSystemDialog {
    type State = ();

    type Event = NewSystemDialogEvent;

    fn update(&mut self, _state: &mut Self::State, message: Self::Event) -> Option<GuiMessage> {
        match message {
            NewSystemDialogEvent::SystemTypeSelected(system_type) => {
                self.system_type = system_type;
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> iced::Element<'_, Self::Event> {
        let real_system_type_radio = Radio::new(
            "Real",
            SystemType::Real,
            Some(self.system_type),
            NewSystemDialogEvent::SystemTypeSelected,
        );
        let generated_system_type_radio = Radio::new(
            "Generated",
            SystemType::Generated,
            Some(self.system_type),
            NewSystemDialogEvent::SystemTypeSelected,
        );
        Column::new()
            .push(real_system_type_radio)
            .push(generated_system_type_radio)
            .into()
    }
}
