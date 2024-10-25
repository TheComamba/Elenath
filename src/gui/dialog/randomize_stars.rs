use super::Dialog;
use crate::gui::{
    gui_widget::{PADDING, SMALL_COLUMN_WIDTH},
    message::GuiMessage,
};
use astro_utils::astro_display::AstroDisplay;
use iced::{
    widget::{component, Button, Column, Component, Radio, Row, Text, Toggler},
    Alignment, Element, Length,
};
use simple_si_units::base::Distance;

#[derive(Debug, Clone)]
pub(crate) struct RandomizeStarsDialog {
    keep_central_body: bool,
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
            keep_central_body: true,
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
pub(crate) enum RandomizeStarsDialogEvent {
    KeepCentralBodySelected(bool),
    MaxGenerationDistanceChanged(GenerationDistance),
    Submit,
}

impl Component<GuiMessage> for RandomizeStarsDialog {
    type State = ();

    type Event = RandomizeStarsDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            RandomizeStarsDialogEvent::KeepCentralBodySelected(keep_central_body) => {
                self.keep_central_body = keep_central_body;
            }
            RandomizeStarsDialogEvent::MaxGenerationDistanceChanged(generation_distance) => {
                self.generation_distance = generation_distance;
            }
            RandomizeStarsDialogEvent::Submit => {
                let max_distance = max_generation_distance(self.generation_distance);
                return Some(GuiMessage::RandomizeStars(
                    self.keep_central_body,
                    max_distance,
                ));
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let warning = Text::new("This will overwrite all stars in the current system.");

        let keep_central_body_toggler = Toggler::new(
            Some("Keep Central Body".to_string()),
            self.keep_central_body,
            RandomizeStarsDialogEvent::KeepCentralBodySelected,
        )
        .width(2. * SMALL_COLUMN_WIDTH);

        let decent_distance_radio = Radio::new(
            format!(
                "Decent\n{}",
                max_generation_distance(GenerationDistance::Decent).astro_display()
            ),
            GenerationDistance::Decent,
            Some(self.generation_distance),
            RandomizeStarsDialogEvent::MaxGenerationDistanceChanged,
        )
        .width(SMALL_COLUMN_WIDTH);
        let realistic_distance_radio = Radio::new(
            format!(
                "Realistic\n{}",
                max_generation_distance(GenerationDistance::Realistic).astro_display()
            ),
            GenerationDistance::Realistic,
            Some(self.generation_distance),
            RandomizeStarsDialogEvent::MaxGenerationDistanceChanged,
        )
        .width(SMALL_COLUMN_WIDTH);
        let very_far_distance_radio = Radio::new(
            format!(
                "Very Far\n{}",
                max_generation_distance(GenerationDistance::VeryFar).astro_display()
            ),
            GenerationDistance::VeryFar,
            Some(self.generation_distance),
            RandomizeStarsDialogEvent::MaxGenerationDistanceChanged,
        )
        .width(SMALL_COLUMN_WIDTH);
        let generation_distance_row = Row::new()
            .push(decent_distance_radio)
            .push(realistic_distance_radio)
            .push(very_far_distance_radio)
            .padding(PADDING)
            .spacing(PADDING);
        let submit_button =
            Button::new(Text::new("Submit")).on_press(RandomizeStarsDialogEvent::Submit);

        Column::new()
            .push(warning)
            .push(keep_central_body_toggler)
            .push(Text::new("Maximum Generation Distance"))
            .push(generation_distance_row)
            .push(submit_button)
            .padding(PADDING)
            .spacing(PADDING)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }
}
