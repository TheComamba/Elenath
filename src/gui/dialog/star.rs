use astro_utils::{
    astro_display::AstroDisplay,
    coordinates::ecliptic::EclipticCoordinates,
    stars::{
        random_stars::generate_random_star, star_data::StarData,
        star_data_evolution::StarDataEvolution,
    },
    units::{
        distance::{distance_to_sun_radii, DISTANCE_ZERO, SOLAR_RADIUS},
        luminous_intensity::{
            absolute_magnitude_to_luminous_intensity, luminous_intensity_to_absolute_magnitude,
        },
        mass::SOLAR_MASS,
        temperature::TEMPERATURE_ZERO,
    },
};
use iced::{
    widget::{component, Button, Column, Component, Row, Text},
    Alignment, Element, Renderer,
};
use simple_si_units::{
    base::{Distance, Temperature, Time},
    geometry::Angle,
};

use crate::gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::edit};

use super::Dialog;

#[derive(Debug, Clone)]
pub(crate) struct StarDialog {
    star_dialog_type: StarDialogType,
    star: StarData,
    star_index: Option<usize>,
    mass_string: String,
    radius_string: String,
    luminosity_string: String,
    temperature_string: String,
    age_string: String,
    distance_string: String,
    longitude_string: String,
    latitude_string: String,
}

#[derive(Debug, Clone, PartialEq)]
enum StarDialogType {
    New,
    Edit,
}

impl StarDialog {
    pub(crate) fn new() -> Self {
        let mut dialog = StarDialog {
            star_dialog_type: StarDialogType::New,
            star: StarData::new(
                String::new(),
                None,
                None,
                None,
                None,
                TEMPERATURE_ZERO,
                None,
                DISTANCE_ZERO,
                EclipticCoordinates::Z_DIRECTION,
                StarDataEvolution::NONE,
            ),
            star_index: None,
            mass_string: String::new(),
            radius_string: String::new(),
            luminosity_string: String::new(),
            temperature_string: String::new(),
            age_string: String::new(),
            distance_string: String::new(),
            longitude_string: String::new(),
            latitude_string: String::new(),
        };
        dialog.fill_string_members();
        dialog
    }

    pub(crate) fn edit(star: StarData, star_index: Option<usize>) -> Self {
        let mut dialog = StarDialog {
            star_dialog_type: StarDialogType::Edit,
            star,
            star_index,
            mass_string: String::new(),
            radius_string: String::new(),
            luminosity_string: String::new(),
            temperature_string: String::new(),
            age_string: String::new(),
            distance_string: String::new(),
            longitude_string: String::new(),
            latitude_string: String::new(),
        };
        dialog.fill_string_members();
        dialog
    }

    fn fill_string_members(&mut self) {
        self.mass_string = self
            .star
            .get_mass_at_epoch()
            .map(|mass| format!("{:.2}", mass.to_solar_mass()))
            .unwrap_or_default();
        self.radius_string = self
            .star
            .get_radius_at_epoch()
            .map(|radius| format!("{:.2}", distance_to_sun_radii(&radius)))
            .unwrap_or_default();
        self.luminosity_string = self
            .star
            .get_luminous_intensity_at_epoch()
            .map(|luminosity| {
                format!(
                    "{:.2}",
                    luminous_intensity_to_absolute_magnitude(luminosity)
                )
            })
            .unwrap_or_default();
        self.temperature_string = format!("{:.0}", self.star.get_temperature_at_epoch().to_K());
        self.age_string = self
            .star
            .get_age_at_epoch()
            .map(|age| format!("{:.2}", age.to_Gyr()))
            .unwrap_or_default();
        self.distance_string = format!("{:.2}", self.star.get_distance_at_epoch().to_lyr());
        self.longitude_string = format!(
            "{:.2}",
            self.star.get_pos_at_epoch().get_longitude().to_degrees()
        );
        self.latitude_string = format!(
            "{:.2}",
            self.star.get_pos_at_epoch().get_latitude().to_degrees()
        );
    }

    fn edit_column(&self) -> Element<'_, StarDialogEvent> {
        let randomize_button =
            Button::new(Text::new("Randomize")).on_press(StarDialogEvent::Randomize);

        let name = edit(
            "Name",
            self.star.get_name(),
            "",
            StarDialogEvent::NameChanged,
            &Some(self.star.get_name()),
        );
        let mass = edit(
            "Mass",
            &self.mass_string,
            "Solar Masses",
            StarDialogEvent::MassChanged,
            self.star.get_mass_at_epoch(),
        );
        let radius = edit(
            "Radius",
            &self.radius_string,
            "Solar Radii",
            StarDialogEvent::RadiusChanged,
            self.star.get_radius_at_epoch(),
        );
        let luminosity = edit(
            "Luminosity",
            &self.luminosity_string,
            "mag",
            StarDialogEvent::LuminosityChanged,
            self.star.get_luminous_intensity_at_epoch(),
        );
        let temperature = edit(
            "Temperature",
            &self.temperature_string,
            "K",
            StarDialogEvent::TemperatureChanged,
            &Some(self.star.get_temperature_at_epoch()),
        );
        let age = edit(
            "Age",
            &self.age_string,
            "Gyr",
            StarDialogEvent::AgeChanged,
            self.star.get_age_at_epoch(),
        );
        let distance = edit(
            "Distance",
            &self.distance_string,
            "ly",
            StarDialogEvent::DistanceChanged,
            &Some(self.star.get_distance_at_epoch()),
        );
        let longitude = edit(
            "Longitude",
            &self.longitude_string,
            "°",
            StarDialogEvent::LongitudeChanged,
            &Some(self.star.get_pos_at_epoch().get_longitude()),
        );
        let latitude = edit(
            "Latitude",
            &self.latitude_string,
            "°",
            StarDialogEvent::LatitudeChanged,
            &Some(self.star.get_pos_at_epoch().get_latitude()),
        );
        let constellation = edit(
            "Constellation",
            &self.star.get_constellation().clone().unwrap_or_default(),
            "",
            StarDialogEvent::ConstellationChanged,
            &self.star.get_constellation(),
        );

        let submit_button = Button::new(Text::new("Submit")).on_press(StarDialogEvent::Submit);

        let mut col = Column::new()
            .push(randomize_button)
            .push(name)
            .push(mass)
            .push(radius)
            .push(luminosity)
            .push(temperature)
            .push(age);
        if !self.is_central_body() {
            col = col
                .push(distance)
                .push(longitude)
                .push(latitude)
                .push(constellation);
        }
        col.push(submit_button)
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn additional_info_column(&self) -> Element<'_, StarDialogEvent> {
        let appearance = self.star.to_star_appearance();

        let illuminance = Text::new(
            "Illuminance: ".to_string() + &appearance.get_illuminance_at_epoch().astro_display(),
        );

        let color =
            Text::new("Color: ".to_string() + &appearance.get_color_at_epoch().astro_display());

        Column::new()
            .push(illuminance)
            .push(color)
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn is_central_body(&self) -> bool {
        self.star_dialog_type == StarDialogType::Edit && self.star_index == None
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
    LongitudeChanged(String),
    LatitudeChanged(String),
    ConstellationChanged(String),
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
                if let Ok(mass) = mass_string.parse::<f64>() {
                    self.star.set_mass_at_epoch(Some(mass * SOLAR_MASS));
                }
                self.mass_string = mass_string;
            }
            StarDialogEvent::RadiusChanged(radius_string) => {
                if let Ok(radius) = radius_string.parse::<f64>() {
                    self.star.set_radius_at_epoch(Some(radius * SOLAR_RADIUS));
                }
                self.radius_string = radius_string;
            }
            StarDialogEvent::LuminosityChanged(luminosity_string) => {
                if let Ok(luminosity) = luminosity_string.parse::<f64>() {
                    self.star.set_luminous_intensity_at_epoch(Some(
                        absolute_magnitude_to_luminous_intensity(luminosity),
                    ));
                }
                self.luminosity_string = luminosity_string;
            }
            StarDialogEvent::TemperatureChanged(temperature_string) => {
                if let Ok(temperature) = temperature_string.parse::<f64>() {
                    self.star
                        .set_temperature_at_epoch(Temperature::from_K(temperature));
                }
                self.temperature_string = temperature_string;
            }
            StarDialogEvent::AgeChanged(age_string) => {
                if let Ok(age) = age_string.parse::<f64>() {
                    self.star.set_age_at_epoch(Some(Time::from_Gyr(age)));
                }
                self.age_string = age_string;
            }
            StarDialogEvent::DistanceChanged(distance_string) => {
                if let Ok(distance) = distance_string.parse::<f64>() {
                    self.star
                        .set_distance_at_epoch(Distance::from_lyr(distance));
                }
                self.distance_string = distance_string;
            }
            StarDialogEvent::LongitudeChanged(longitude_string) => {
                if let Ok(longitude) = longitude_string.parse::<f64>() {
                    let mut pos = self.star.get_pos_at_epoch().clone();
                    pos.set_longitude(Angle::from_degrees(longitude));
                    self.star.set_pos_at_epoch(pos);
                }
                self.longitude_string = longitude_string;
            }
            StarDialogEvent::LatitudeChanged(latitude_string) => {
                if let Ok(latitude) = latitude_string.parse::<f64>() {
                    let mut pos = self.star.get_pos_at_epoch().clone();
                    pos.set_latitude(Angle::from_degrees(latitude));
                    self.star.set_pos_at_epoch(pos);
                }
                self.latitude_string = latitude_string;
            }
            StarDialogEvent::ConstellationChanged(constellation) => {
                if constellation.is_empty() {
                    self.star.set_constellation(None);
                } else {
                    self.star.set_constellation(Some(constellation));
                }
            }
            StarDialogEvent::Randomize => {
                let max_distance = Distance::from_lyr(2000.);
                let name = self.star.get_name().clone();
                self.star = match generate_random_star(Some(max_distance)) {
                    Ok(star) => star,
                    Err(e) => {
                        return Some(GuiMessage::ErrorEncountered(e.into()));
                    }
                };
                self.star.set_name(name);
                self.fill_string_members();
            }
            StarDialogEvent::Submit => match self.star_dialog_type {
                StarDialogType::Edit => {
                    let mut star = self.star.clone();
                    if self.is_central_body() {
                        star.set_distance_at_epoch(DISTANCE_ZERO);
                    }
                    return Some(GuiMessage::StarEdited(self.star_index, star));
                }
                StarDialogType::New => return Some(GuiMessage::NewStar(self.star.clone())),
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
