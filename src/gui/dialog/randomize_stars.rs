use super::Dialog;
use crate::{
    gui::{
        gui_widget::{PADDING, SMALL_COLUMN_WIDTH},
        message::GuiMessage,
    },
    model::new_celestial_system::{generated_system, GeneratedCentralBody},
};
use astro_utils::astro_display::AstroDisplay;
use iced::{
    widget::{component, Button, Column, Component, Radio, Row, Text},
    Alignment, Element, Length,
};
use simple_si_units::base::Distance;

#[derive(Debug, Clone)]
pub(crate) struct RandomizeStarsDialog {
    generated_central_body: GeneratedCentralBody,
    generation_distance: GenerationDistance,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum GenerationDistance {
    Decent,
    Realistic,
    VeryFar,
}

impl RandomizeStarsDialog {
    pub(crate) fn new() -> Self {
        RandomizeStarsDialog {
            generated_central_body: GeneratedCentralBody::Sun,
            generation_distance: GenerationDistance::Decent,
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

impl Dialog for RandomizeStarsDialog {
    fn header(&self) -> String {
        "Generate Random Stars".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NewSystemDialogEvent {
    GeneratedCentralBodySelected(GeneratedCentralBody),
    MaxGenerationDistanceChanged(GenerationDistance),
    Submit,
}

impl Component<GuiMessage> for RandomizeStarsDialog {
    type State = ();

    type Event = NewSystemDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            NewSystemDialogEvent::GeneratedCentralBodySelected(generated_central_body) => {
                self.generated_central_body = generated_central_body;
            }
            NewSystemDialogEvent::MaxGenerationDistanceChanged(generation_distance) => {
                self.generation_distance = generation_distance;
            }
            NewSystemDialogEvent::Submit => {
                let system = generated_system(
                    &self.generated_central_body,
                    max_generation_distance(self.generation_distance),
                );
                return Some(GuiMessage::NewSystem(system));
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let warning = Text::new("This will overwrite all stars in the current system.");

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
        let submit_button = Button::new(Text::new("Submit")).on_press(NewSystemDialogEvent::Submit);

        Column::new()
            .push(warning)
            .push(central_body_row)
            .push(Text::new("Maximum Generation Distance"))
            .push(generation_distance_row)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
