use super::Dialog;
use crate::{
    error::ElenathError,
    gui::{
        gui_widget::{PADDING, SMALL_COLUMN_WIDTH},
        message::GuiMessage,
        shared_widgets::edit,
    },
    model::{
        celestial_system::{CelestialSystem, SystemType},
        new_celestial_system::{generated_system, solar_system, GeneratedCentralBody},
    },
};
use astro_utils::units::distance::DISTANCE_ZERO;
use iced::{
    widget::{component, Button, Column, Component, Radio, Row, Text, Toggler},
    Alignment, Element, Renderer,
};
use simple_si_units::base::Distance;

#[derive(Debug, Clone)]
pub(crate) struct NewSystemDialog {
    system_type: SystemType,
    load_gaia_data: bool,
    generated_central_body: GeneratedCentralBody,
    max_generation_distance_text: String,
    max_generation_distance: Distance<f64>,
}

impl NewSystemDialog {
    pub(crate) fn new() -> Self {
        NewSystemDialog {
            system_type: SystemType::Real,
            load_gaia_data: false,
            generated_central_body: GeneratedCentralBody::Sun,
            max_generation_distance_text: String::new(),
            max_generation_distance: DISTANCE_ZERO,
        }
    }

    fn celestial_system(&self) -> Result<CelestialSystem, ElenathError> {
        match self.system_type {
            SystemType::Real => solar_system(self.load_gaia_data),
            SystemType::Generated => {
                generated_system(&self.generated_central_body, self.max_generation_distance)
            }
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
    GeneratedCentralBodySelected(GeneratedCentralBody),
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
            NewSystemDialogEvent::GeneratedCentralBodySelected(generated_central_body) => {
                self.generated_central_body = generated_central_body;
            }
            NewSystemDialogEvent::MaxGenerationDistanceChanged(max_generation_distance_text) => {
                if let Ok(max_generation_distance) = max_generation_distance_text.parse::<f64>() {
                    self.max_generation_distance_text = max_generation_distance_text;
                    self.max_generation_distance = Distance::from_lyr(max_generation_distance);
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
        )
        .width(SMALL_COLUMN_WIDTH);
        let generated_system_type_radio = Radio::new(
            "Generated",
            SystemType::Generated,
            Some(self.system_type),
            NewSystemDialogEvent::SystemTypeSelected,
        )
        .width(SMALL_COLUMN_WIDTH);
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
                )
                .width(2. * SMALL_COLUMN_WIDTH);
                col = col.push(load_gaia_data_toggler);
            }
            SystemType::Generated => {
                let sun_radio = Radio::new(
                    "Use the Sun as Central Body",
                    GeneratedCentralBody::Sun,
                    Some(self.generated_central_body),
                    NewSystemDialogEvent::GeneratedCentralBodySelected,
                )
                .width(SMALL_COLUMN_WIDTH);
                let random_star_radio = Radio::new(
                    "Generate Random Central Body",
                    GeneratedCentralBody::RandomStar,
                    Some(self.generated_central_body),
                    NewSystemDialogEvent::GeneratedCentralBodySelected,
                )
                .width(SMALL_COLUMN_WIDTH);
                let central_body_row = Row::new()
                    .push(sun_radio)
                    .push(random_star_radio)
                    .padding(PADDING)
                    .spacing(PADDING);

                let max_generation_distance_input = edit(
                    "Maximum distance",
                    &self.max_generation_distance_text,
                    "ly",
                    NewSystemDialogEvent::MaxGenerationDistanceChanged,
                    &Some(self.max_generation_distance),
                );
                col = col
                    .push(central_body_row)
                    .push(max_generation_distance_input);
            }
        }

        let submit_button = Button::new(Text::new("Submit")).on_press(NewSystemDialogEvent::Submit);
        col.push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
