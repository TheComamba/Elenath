use super::Dialog;
use crate::{
    error::ElenathError,
    gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::edit},
    model::{
        celestial_system::{CelestialSystem, SystemType},
        new_celestial_system::{generated_system, solar_system},
    },
};
use astro_utils::{units::length::Length, Float};
use iced::{
    widget::{component, Button, Column, Component, Radio, Row, Text, TextInput, Toggler},
    Element, Renderer,
};

#[derive(Debug, Clone)]
pub(crate) struct NewSystemDialog {
    system_type: SystemType,
    load_gaia_data: bool,
    max_generation_distance_text: String,
    max_generation_distance: Length,
}

impl NewSystemDialog {
    pub(crate) fn new() -> Self {
        NewSystemDialog {
            system_type: SystemType::Real,
            load_gaia_data: false,
            max_generation_distance_text: String::new(),
            max_generation_distance: Length::ZERO,
        }
    }

    fn celestial_system(&self) -> Result<CelestialSystem, ElenathError> {
        match self.system_type {
            SystemType::Real => solar_system(self.load_gaia_data),
            SystemType::Generated => generated_system(self.max_generation_distance),
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
    MaxGenerationDistanceChanged(String),
    Submit,
}

impl Component<GuiMessage, Renderer> for NewSystemDialog {
    type State = ();

    type Event = NewSystemDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            NewSystemDialogEvent::SystemTypeSelected(system_type) => {
                self.system_type = system_type;
            }
            NewSystemDialogEvent::LoadGaiaDataSelected(load_gaia_data) => {
                self.load_gaia_data = load_gaia_data;
            }
            NewSystemDialogEvent::MaxGenerationDistanceChanged(max_generation_distance_text) => {
                if let Ok(max_generation_distance) = max_generation_distance_text.parse::<Float>() {
                    self.max_generation_distance_text = max_generation_distance_text;
                    self.max_generation_distance =
                        Length::from_light_years(max_generation_distance);
                }
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
                let max_generation_distance_input = edit(
                    "Maximum distance at which new stars are generated:",
                    &self.max_generation_distance_text,
                    "Pick 100 for a quick test population, and 2000 for a time-consuming but realistic generation.",
                    "ly",
                    |t|NewSystemDialogEvent::MaxGenerationDistanceChanged(t),
                );
                col = col.push(max_generation_distance_input);
            }
        }

        let submit_button = Button::new(Text::new("Submit")).on_press(NewSystemDialogEvent::Submit);
        col.push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .into()
    }
}
