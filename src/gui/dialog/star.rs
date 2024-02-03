use astro_utils::{
    coordinates::direction::Direction,
    stars::{random_stars::generate_random_star, star_data::StarData},
    units::{
        length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature, time::Time,
    },
    Float,
};
use iced::{
    widget::{component, Button, Column, Component, Row, Text},
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
        let mut dialog = StarDialog {
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
        };
        dialog.fill_string_members();
        dialog
    }

    pub(crate) fn edit(star: StarData, star_index: usize) -> Self {
        let mut dialog = StarDialog {
            star,
            star_index: Some(star_index),
            mass_string: String::new(),
            radius_string: String::new(),
            luminosity_string: String::new(),
            temperature_string: String::new(),
            age_string: String::new(),
            distance_string: String::new(),
            direction_string: String::new(),
        };
        dialog.fill_string_members();
        dialog
    }

    fn fill_string_members(&mut self) {
        self.mass_string = self
            .star
            .get_mass()
            .map(|mass| mass.as_solar_masses().to_string())
            .unwrap_or(String::new());
        self.radius_string = self
            .star
            .get_radius()
            .map(|radius| radius.as_sun_radii().to_string())
            .unwrap_or(String::new());
        self.luminosity_string = self
            .star
            .get_luminosity()
            .map(|luminosity| luminosity.as_absolute_magnitude().to_string())
            .unwrap_or(String::new());
        self.temperature_string = self
            .star
            .get_temperature()
            .map(|temperature| temperature.as_kelvin().to_string())
            .unwrap_or(String::new());
        self.age_string = self
            .star
            .get_age()
            .map(|age| age.as_billion_years().to_string())
            .unwrap_or(String::new());
        self.distance_string = self
            .star
            .get_distance()
            .map(|distance| distance.as_light_years().to_string())
            .unwrap_or(String::new());
        self.direction_string =
            serde_json::to_string(self.star.get_direction_in_ecliptic()).unwrap();
    }

    fn edit_column(&self) -> Element<'_, StarDialogEvent> {
        let randomize_button =
            Button::new(Text::new("Randomize")).on_press(StarDialogEvent::Randomize);

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
            "Solar Masses",
            |t| StarDialogEvent::MassChanged(t),
            self.star.get_mass(),
        );
        let radius = edit(
            "Radius",
            &self.radius_string,
            "Solar Radii",
            |t| StarDialogEvent::RadiusChanged(t),
            self.star.get_radius(),
        );
        let luminosity = edit(
            "Luminosity",
            &self.luminosity_string,
            "mag",
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
            "Gyr",
            |t| StarDialogEvent::AgeChanged(t),
            self.star.get_age(),
        );
        let distance = edit(
            "Distance",
            &self.distance_string,
            "ly",
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

        let submit_button = Button::new(Text::new("Submit")).on_press(StarDialogEvent::Submit);

        Column::new()
            .push(randomize_button)
            .push(name)
            .push(mass)
            .push(radius)
            .push(luminosity)
            .push(temperature)
            .push(age)
            .push(distance)
            .push(direction)
            .push(submit_button)
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn additional_info_column(&self) -> Element<'_, StarDialogEvent> {
        let appearance = self.star.to_star_appearance();

        let illuminance = Text::new(format!("Illuminance: {}", appearance.get_illuminance()));

        let color = Text::new(format!("Color: {}", appearance.get_color()));

        Column::new()
            .push(illuminance)
            .push(color)
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
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
    Randomize,
    Submit,
}

impl Component<GuiMessage, Renderer> for StarDialog {
    type State = ();

    type Event = StarDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            StarDialogEvent::NameChanged(name) => {
                self.star.set_name(name);
            }
            StarDialogEvent::MassChanged(mass_string) => {
                if let Ok(mass) = mass_string.parse::<Float>() {
                    self.star.set_mass(Some(Mass::from_solar_masses(mass)));
                }
                self.mass_string = mass_string;
            }
            StarDialogEvent::RadiusChanged(radius_string) => {
                if let Ok(radius) = radius_string.parse::<Float>() {
                    self.star.set_radius(Some(Length::from_sun_radii(radius)));
                }
                self.radius_string = radius_string;
            }
            StarDialogEvent::LuminosityChanged(luminosity_string) => {
                if let Ok(luminosity) = luminosity_string.parse::<Float>() {
                    self.star
                        .set_luminosity(Some(Luminosity::from_absolute_magnitude(luminosity)));
                }
                self.luminosity_string = luminosity_string;
            }
            StarDialogEvent::TemperatureChanged(temperature_string) => {
                if let Ok(temperature) = temperature_string.parse::<Float>() {
                    self.star
                        .set_temperature(Some(Temperature::from_kelvin(temperature)));
                }
                self.temperature_string = temperature_string;
            }
            StarDialogEvent::AgeChanged(age_string) => {
                if let Ok(age) = age_string.parse::<Float>() {
                    self.star.set_age(Some(Time::from_billion_years(age)));
                }
                self.age_string = age_string;
            }
            StarDialogEvent::DistanceChanged(distance_string) => {
                if let Ok(distance) = distance_string.parse::<Float>() {
                    self.star
                        .set_distance(Some(Length::from_light_years(distance)));
                }
                self.distance_string = distance_string;
            }
            StarDialogEvent::DirectionChanged(direction_string) => {
                if let Ok(dir) = serde_json::from_str::<Direction>(&direction_string) {
                    if let Ok(dir) = Direction::new(dir.x(), dir.y(), dir.z()) {
                        self.star.set_direction_in_ecliptic(dir);
                    }
                }
                self.direction_string = direction_string;
            }
            StarDialogEvent::Randomize => {
                let max_distance = Length::from_light_years(2000.);
                self.star = match generate_random_star(Some(max_distance)) {
                    Ok(star) => star,
                    Err(e) => {
                        return Some(GuiMessage::ErrorEncountered(e.into()));
                    }
                };
                self.fill_string_members();
            }
            StarDialogEvent::Submit => match self.star_index {
                Some(index) => return Some(GuiMessage::StarEdited(index, self.star.clone())),
                None => return Some(GuiMessage::NewStar(self.star.clone())),
            },
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        Row::new()
            .push(self.edit_column())
            .push(self.additional_info_column())
            .into()
    }
}
