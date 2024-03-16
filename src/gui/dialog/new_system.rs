use super::Dialog;
use crate::{
    error::ElenathError,
    gui::{
        gui_widget::{PADDING, SMALL_COLUMN_WIDTH},
        message::GuiMessage,
    },
    model::{
        celestial_system::CelestialSystem,
        new_celestial_system::{generated_system, solar_system, GeneratedCentralBody},
    },
};
use astro_utils::astro_display::AstroDisplay;
use iced::{
    widget::{component, Button, Column, Component, Radio, Row, Text, Toggler},
    Alignment, Element, Length,
};
use simple_si_units::base::Distance;

#[derive(Debug, Clone)]
pub(crate) struct NewSystemDialog {
    system_type: SystemType,
    load_gaia_data: bool,
    generated_central_body: GeneratedCentralBody,
    generation_distance: GenerationDistance,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum SystemType {
    Real,
    Generated,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum GenerationDistance {
    Decent,
    Realistic,
    VeryFar,
}

impl NewSystemDialog {
    pub(crate) fn new() -> Self {
        NewSystemDialog {
            system_type: SystemType::Real,
            load_gaia_data: false,
            generated_central_body: GeneratedCentralBody::Sun,
            generation_distance: GenerationDistance::Decent,
        }
    }

    fn celestial_system(&self) -> Result<CelestialSystem, ElenathError> {
        match self.system_type {
            SystemType::Real => solar_system(self.load_gaia_data),
            SystemType::Generated => generated_system(
                &self.generated_central_body,
                max_generation_distance(self.generation_distance),
            ),
        }
    }
}

fn max_generation_distance(distance: GenerationDistance) -> Distance<f64> {
    match distance {
        GenerationDistance::Decent => Distance::from_lyr(1000.0),
        GenerationDistance::Realistic => Distance::from_lyr(5000.0),
        GenerationDistance::VeryFar => Distance::from_lyr(25_000.0),
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
    MaxGenerationDistanceChanged(GenerationDistance),
    Submit,
}

impl Component<GuiMessage> for NewSystemDialog {
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
            NewSystemDialogEvent::MaxGenerationDistanceChanged(generation_distance) => {
                self.generation_distance = generation_distance;
            }
            NewSystemDialogEvent::Submit => {
                return Some(GuiMessage::NewSystemDialogSubmit(self.celestial_system()));
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
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

                let decent_distance_radio = Radio::new(
                    format!(
                        "Decent\n{}",
                        max_generation_distance(GenerationDistance::Decent).astro_display()
                    ),
                    GenerationDistance::Decent,
                    Some(self.generation_distance),
                    NewSystemDialogEvent::MaxGenerationDistanceChanged,
                )
                .width(SMALL_COLUMN_WIDTH);
                let realistic_distance_radio = Radio::new(
                    format!(
                        "Realistic\n{}",
                        max_generation_distance(GenerationDistance::Realistic).astro_display()
                    ),
                    GenerationDistance::Realistic,
                    Some(self.generation_distance),
                    NewSystemDialogEvent::MaxGenerationDistanceChanged,
                )
                .width(SMALL_COLUMN_WIDTH);
                let very_far_distance_radio = Radio::new(
                    format!(
                        "Very Far\n{}",
                        max_generation_distance(GenerationDistance::VeryFar).astro_display()
                    ),
                    GenerationDistance::VeryFar,
                    Some(self.generation_distance),
                    NewSystemDialogEvent::MaxGenerationDistanceChanged,
                )
                .width(SMALL_COLUMN_WIDTH);
                let generation_distance_row = Row::new()
                    .push(decent_distance_radio)
                    .push(realistic_distance_radio)
                    .push(very_far_distance_radio)
                    .padding(PADDING)
                    .spacing(PADDING);

                col = col
                    .push(central_body_row)
                    .push(Text::new("Maximum Generation Distance"))
                    .push(generation_distance_row);
            }
        }

        let submit_button = Button::new(Text::new("Submit")).on_press(NewSystemDialogEvent::Submit);
        col.push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
