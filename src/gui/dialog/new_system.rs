use super::Dialog;
use crate::{
    gui::gui_widget::{GuiMessage, PADDING},
    model::{
        celestial_system::{CelestialSystem, SystemType},
        new_celestial_system::{generated_system, solar_system},
    },
};
use iced::{
    widget::{component, Button, Column, Component, Radio, Row, Text, Toggler},
    Element, Renderer,
};

#[derive(Debug, Clone)]
pub(crate) struct NewSystemDialog {
    system_type: SystemType,
    load_gaia_data: bool,
}

impl NewSystemDialog {
    pub(crate) fn new() -> Self {
        NewSystemDialog {
            system_type: SystemType::Real,
            load_gaia_data: false,
        }
    }

    fn celestial_system(&self) -> CelestialSystem {
        match self.system_type {
            SystemType::Real => solar_system(self.load_gaia_data),
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
    LoadGaiaDataSelected(bool),
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
            NewSystemDialogEvent::LoadGaiaDataSelected(load_gaia_data) => {
                self.load_gaia_data = load_gaia_data;
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
        let type_row = Row::new()
            .push(real_system_type_radio)
            .push(generated_system_type_radio)
            .padding(PADDING)
            .spacing(PADDING);

        let mut col = Column::new().push(type_row);

        match self.system_type {
            SystemType::Real => {
                let load_gaia_data_toggler = Toggler::new(
                    Some("Load Gaia Data".to_string()),
                    self.load_gaia_data,
                    NewSystemDialogEvent::LoadGaiaDataSelected,
                );
                col = col.push(load_gaia_data_toggler);
            }
            SystemType::Generated => {
                // Do nothing.
            }
        }

        let submit_button = Button::new(Text::new("Submit")).on_press(NewSystemDialogEvent::Submit);
        col.push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .into()
    }
}
