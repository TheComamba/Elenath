use super::Dialog;
use crate::{
    gui::gui_widget::{GuiMessage, PADDING},
    model::{
        celestial_system::{CelestialSystem, SystemType},
        example::{generated_system, solar_system},
    },
};
use iced::{
    widget::{component, Button, Column, Component, Radio, Text},
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

    fn celestial_system(&self) -> CelestialSystem {
        match self.system_type {
            SystemType::Real => solar_system(),
            SystemType::Generated => generated_system(),
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
    Submit,
}

impl Component<GuiMessage, Renderer> for NewSystemDialog {
    type State = ();

    type Event = NewSystemDialogEvent;

    fn update(&mut self, _state: &mut Self::State, message: Self::Event) -> Option<GuiMessage> {
        match message {
            NewSystemDialogEvent::SystemTypeSelected(system_type) => {
                self.system_type = system_type;
            }
            NewSystemDialogEvent::Submit => {
                return Some(GuiMessage::NewSystemDialogSubmit(self.celestial_system()));
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
        let submit_button = Button::new(Text::new("Submit")).on_press(NewSystemDialogEvent::Submit);
        Column::new()
            .push(real_system_type_radio)
            .push(generated_system_type_radio)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .into()
    }
}
