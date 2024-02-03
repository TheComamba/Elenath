use astro_utils::{coordinates::direction::Direction, stars::star_data::StarData};
use iced::{
    widget::{component, Column, Component},
    Alignment, Element, Renderer,
};

use crate::gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::edit};

use super::Dialog;

#[derive(Debug, Clone)]
pub(crate) struct StarDialog {
    star: StarData,
    star_index: Option<usize>,
    mass_string: String,
    radius_string: String,
    luminosity_string: String,
    temperature_string: String,
    age_string: String,
    distance_string: String,
    direction_string: String,
}

impl StarDialog {
    pub(crate) fn new() -> Self {
        StarDialog {
            star: StarData::new(
                "".to_string(),
                None,
                None,
                None,
                None,
                None,
                None,
                Direction::Z,
            ),
            star_index: None,
            mass_string: String::new(),
            radius_string: String::new(),
            luminosity_string: String::new(),
            temperature_string: String::new(),
            age_string: String::new(),
            distance_string: String::new(),
            direction_string: String::new(),
        }
    }

    pub(crate) fn edit(star: StarData, star_index: usize) -> Self {
        let mass_string = serde_json::to_string(&star.get_mass()).unwrap_or(String::new());
        let radius_string = serde_json::to_string(&star.get_radius()).unwrap_or(String::new());
        let luminosity_string =
            serde_json::to_string(&star.get_luminosity()).unwrap_or(String::new());
        let temperature_string =
            serde_json::to_string(&star.get_temperature()).unwrap_or(String::new());
        let age_string = serde_json::to_string(&star.get_age()).unwrap_or(String::new());
        let distance_string = serde_json::to_string(&star.get_distance()).unwrap_or(String::new());
        let direction_string =
            serde_json::to_string(&star.get_direction_in_ecliptic()).unwrap_or(String::new());
        StarDialog {
            star,
            star_index: Some(star_index),
            mass_string,
            radius_string,
            luminosity_string,
            temperature_string,
            age_string,
            distance_string,
            direction_string,
        }
    }
}

impl Dialog for StarDialog {
    fn header(&self) -> String {
        match self.star_index {
            Some(index) => format!("Edit Star {}", index),
            None => "Create Star".to_string(),
        }
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum StarDialogEvent {
    NameChanged(String),
    MassChanged(String),
    RadiusChanged(String),
    LuminosityChanged(String),
    TemperatureChanged(String),
    AgeChanged(String),
    DistanceChanged(String),
    DirectionChanged(String),
    Submit,
}

impl Component<GuiMessage, Renderer> for StarDialog {
    type State = ();

    type Event = StarDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let name = edit(
            "Name",
            self.star.get_name(),
            "",
            |t| StarDialogEvent::NameChanged(t),
            &Some(self.star.get_name()),
        );
        let mass = edit(
            "Mass",
            &self.mass_string,
            "Sun Masses",
            |t| StarDialogEvent::MassChanged(t),
            self.star.get_mass(),
        );
        let radius = edit(
            "Radius",
            &self.radius_string,
            "Sun Radii",
            |t| StarDialogEvent::RadiusChanged(t),
            self.star.get_radius(),
        );
        let luminosity = edit(
            "Luminosity",
            &self.luminosity_string,
            "W",
            |t| StarDialogEvent::LuminosityChanged(t),
            self.star.get_luminosity(),
        );
        let temperature = edit(
            "Temperature",
            &self.temperature_string,
            "K",
            |t| StarDialogEvent::TemperatureChanged(t),
            self.star.get_temperature(),
        );
        let age = edit(
            "Age",
            &self.age_string,
            "years",
            |t| StarDialogEvent::AgeChanged(t),
            self.star.get_age(),
        );
        let distance = edit(
            "Distance",
            &self.distance_string,
            "m",
            |t| StarDialogEvent::DistanceChanged(t),
            self.star.get_distance(),
        );
        let direction = edit(
            "Direction",
            &self.direction_string,
            "",
            |t| StarDialogEvent::DirectionChanged(t),
            &Some(self.star.get_direction_in_ecliptic()),
        );

        Column::new()
            .push(name)
            .push(mass)
            .push(radius)
            .push(luminosity)
            .push(temperature)
            .push(age)
            .push(distance)
            .push(direction)
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
