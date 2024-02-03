use super::Dialog;
use crate::gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::edit};
use astro_utils::{
    color::sRGBColor,
    coordinates::direction::Direction,
    planets::{orbit_parameters::OrbitParameters, planet_data::PlanetData},
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
    Float,
};
use iced::{
    widget::{component, Button, Column, Component, Text},
    Alignment, Element, Renderer,
};
use serde_json;

#[derive(Debug, Clone)]
pub(crate) struct PlanetDialog {
    planet: PlanetData,
    planet_index: Option<usize>,
    mass_string: String,
    radius_string: String,
    geometric_albedo_string: String,
    semi_major_axis_string: String,
    eccentricity_string: String,
    inclination_string: String,
    longitude_of_ascending_node_string: String,
    argument_of_periapsis_string: String,
    siderial_rotation_period_string: String,
    rotation_axis_string: String,
}

impl PlanetDialog {
    pub(crate) fn edit(planet: PlanetData, planet_index: usize) -> Self {
        PlanetDialog {
            planet: planet.clone(),
            planet_index: Some(planet_index),
            mass_string: planet.get_mass().as_earth_masses().to_string(),
            radius_string: planet.get_radius().as_earth_radii().to_string(),
            geometric_albedo_string: planet.get_geometric_albedo().to_string(),
            semi_major_axis_string: planet
                .get_orbital_parameters()
                .get_semi_major_axis()
                .as_astronomical_units()
                .to_string(),
            eccentricity_string: planet
                .get_orbital_parameters()
                .get_eccentricity()
                .to_string(),
            inclination_string: planet
                .get_orbital_parameters()
                .get_inclination()
                .as_degrees()
                .to_string(),
            longitude_of_ascending_node_string: planet
                .get_orbital_parameters()
                .get_longitude_of_ascending_node()
                .as_degrees()
                .to_string(),
            argument_of_periapsis_string: planet
                .get_orbital_parameters()
                .get_argument_of_periapsis()
                .as_degrees()
                .to_string(),
            siderial_rotation_period_string: planet
                .get_sideral_rotation_period()
                .as_days()
                .to_string(),
            rotation_axis_string: serde_json::to_string(planet.get_rotation_axis()).unwrap(),
        }
    }

    pub(crate) fn new() -> Self {
        PlanetDialog {
            planet: PlanetData::new(
                String::new(),
                Mass::ZERO,
                OrbitParameters::new(Length::ZERO, 0.0, Angle::ZERO, Angle::ZERO, Angle::ZERO),
                Length::ZERO,
                0.0,
                sRGBColor::from_sRGB(0., 0., 0.),
                Time::ZERO,
                Direction::Z,
            ),
            planet_index: None,
            mass_string: String::new(),
            radius_string: String::new(),
            geometric_albedo_string: String::new(),
            semi_major_axis_string: String::new(),
            eccentricity_string: String::new(),
            inclination_string: String::new(),
            longitude_of_ascending_node_string: String::new(),
            argument_of_periapsis_string: String::new(),
            siderial_rotation_period_string: String::new(),
            rotation_axis_string: serde_json::to_string(&Direction::Z).unwrap(),
        }
    }
}

impl Dialog for PlanetDialog {
    fn header(&self) -> String {
        match self.planet_index {
            Some(index) => format!("Edit Planet {}", index),
            None => "Create Planet".to_string(),
        }
    }

    fn body<'a>(&self) -> Element<'a, GuiMessage> {
        component(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum PlanetDialogEvent {
    NameChanged(String),
    MassChanged(String),
    RadiusChanged(String),
    GeometricAlbedoChanged(String),
    SemiMajorAxisChanged(String),
    EccentricityChanged(String),
    InclinationChanged(String),
    LongitudeOfAscendingNodeChanged(String),
    ArgumentOfPeriapsisChanged(String),
    SiderialRotationPeriodChanged(String),
    RotationAxisChanged(String),
    Submit,
}

impl Component<GuiMessage, Renderer> for PlanetDialog {
    type State = ();

    type Event = PlanetDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        match event {
            PlanetDialogEvent::NameChanged(name) => {
                self.planet.set_name(name);
            }
            PlanetDialogEvent::MassChanged(mass_string) => {
                if let Ok(mass) = mass_string.parse::<Float>() {
                    self.planet.set_mass(Mass::from_earth_masses(mass));
                    self.mass_string = mass_string;
                }
            }
            PlanetDialogEvent::RadiusChanged(radius_string) => {
                if let Ok(radius) = radius_string.parse::<Float>() {
                    self.planet.set_radius(Length::from_earth_radii(radius));
                    self.radius_string = radius_string;
                }
            }
            PlanetDialogEvent::GeometricAlbedoChanged(geometric_albedo_string) => {
                if let Ok(geometric_albedo) = geometric_albedo_string.parse::<Float>() {
                    self.planet.set_geometric_albedo(geometric_albedo);
                    self.geometric_albedo_string = geometric_albedo_string;
                }
            }
            PlanetDialogEvent::SemiMajorAxisChanged(semi_major_axis_string) => {
                if let Ok(semi_major_axis) = semi_major_axis_string.parse::<Float>() {
                    self.planet
                        .set_semi_major_axis(Length::from_astronomical_units(semi_major_axis));
                    self.semi_major_axis_string = semi_major_axis_string;
                }
            }
            PlanetDialogEvent::EccentricityChanged(eccentricity_string) => {
                if let Ok(eccentricity) = eccentricity_string.parse::<Float>() {
                    self.planet.set_eccentricity(eccentricity);
                    self.eccentricity_string = eccentricity_string;
                }
            }
            PlanetDialogEvent::InclinationChanged(inclination_string) => {
                if let Ok(inclination) = inclination_string.parse::<Float>() {
                    self.planet
                        .set_inclination(Angle::from_degrees(inclination));
                    self.inclination_string = inclination_string;
                }
            }
            PlanetDialogEvent::LongitudeOfAscendingNodeChanged(
                longitude_of_ascending_node_string,
            ) => {
                if let Ok(longitude_of_ascending_node) =
                    longitude_of_ascending_node_string.parse::<Float>()
                {
                    self.planet
                        .set_longitude_of_ascending_node(Angle::from_degrees(
                            longitude_of_ascending_node,
                        ));
                    self.longitude_of_ascending_node_string = longitude_of_ascending_node_string;
                }
            }
            PlanetDialogEvent::ArgumentOfPeriapsisChanged(argument_of_periapsis_string) => {
                if let Ok(argument_of_periapsis) = argument_of_periapsis_string.parse::<Float>() {
                    self.planet
                        .set_argument_of_periapsis(Angle::from_degrees(argument_of_periapsis));
                    self.argument_of_periapsis_string = argument_of_periapsis_string;
                }
            }
            PlanetDialogEvent::SiderialRotationPeriodChanged(siderial_rotation_period_string) => {
                if let Ok(siderial_rotation_period) =
                    siderial_rotation_period_string.parse::<Float>()
                {
                    self.planet
                        .set_sideral_rotation_period(Time::from_days(siderial_rotation_period));
                    self.siderial_rotation_period_string = siderial_rotation_period_string;
                }
            }
            PlanetDialogEvent::RotationAxisChanged(rotation_axis_string) => {
                if let Ok(axis) = serde_json::from_str::<Direction>(&rotation_axis_string) {
                    if let Ok(rotation_axis) = Direction::new(axis.x(), axis.y(), axis.z()) {
                        self.planet.set_rotation_axis(rotation_axis);
                    }
                }
                self.rotation_axis_string = rotation_axis_string;
            }
            PlanetDialogEvent::Submit => match self.planet_index {
                Some(index) => {
                    return Some(GuiMessage::PlanetEdited(index, self.planet.clone()));
                }
                None => {
                    return Some(GuiMessage::NewPlanet(self.planet.clone()));
                }
            },
        }
        None
    }

    fn view(&self, _state: &Self::State) -> iced::Element<'_, Self::Event> {
        let name = edit(
            "Name",
            self.planet.get_name(),
            "",
            |t| PlanetDialogEvent::NameChanged(t),
            &Some(self.planet.get_name()),
        );
        let mass = edit(
            "Mass",
            &self.mass_string,
            "Earth Masses",
            |t| PlanetDialogEvent::MassChanged(t),
            &Some(self.planet.get_mass()),
        );
        let radius = edit(
            "Radius",
            &self.radius_string,
            "Earth Radii",
            |t| PlanetDialogEvent::RadiusChanged(t),
            &Some(self.planet.get_radius()),
        );
        let geometric_albedo = edit(
            "Geometric Albedo",
            &self.geometric_albedo_string,
            "",
            |t| PlanetDialogEvent::GeometricAlbedoChanged(t),
            &Some(self.planet.get_geometric_albedo()),
        );
        let semi_major_axis = edit(
            "Semi Major Axis",
            &self.semi_major_axis_string,
            "AU",
            |t| PlanetDialogEvent::SemiMajorAxisChanged(t),
            &Some(self.planet.get_orbital_parameters().get_semi_major_axis()),
        );
        let eccentricity = edit(
            "Eccentricity",
            &self.eccentricity_string,
            "",
            |t| PlanetDialogEvent::EccentricityChanged(t),
            &Some(self.planet.get_orbital_parameters().get_eccentricity()),
        );
        let inclination = edit(
            "Inclination",
            &self.inclination_string,
            "°",
            |t| PlanetDialogEvent::InclinationChanged(t),
            &Some(self.planet.get_orbital_parameters().get_inclination()),
        );
        let longitude_of_ascending_node = edit(
            "Longitude of Ascending Node",
            &self.longitude_of_ascending_node_string,
            "°",
            |t| PlanetDialogEvent::LongitudeOfAscendingNodeChanged(t),
            &Some(
                self.planet
                    .get_orbital_parameters()
                    .get_longitude_of_ascending_node(),
            ),
        );
        let argument_of_periapsis = edit(
            "Argument of Periapsis",
            &self.argument_of_periapsis_string,
            "°",
            |t| PlanetDialogEvent::ArgumentOfPeriapsisChanged(t),
            &Some(
                self.planet
                    .get_orbital_parameters()
                    .get_argument_of_periapsis(),
            ),
        );
        let siderial_rotation_period = edit(
            "Siderial Rotation Period",
            &self.siderial_rotation_period_string,
            "Earth Days",
            |t| PlanetDialogEvent::SiderialRotationPeriodChanged(t),
            &Some(self.planet.get_sideral_rotation_period()),
        );
        let rotation_axis = edit(
            "Rotation Axis",
            &self.rotation_axis_string,
            "",
            |t| PlanetDialogEvent::RotationAxisChanged(t),
            &Some(self.planet.get_rotation_axis()),
        );

        let submit_button = Button::new(Text::new("Submit")).on_press(PlanetDialogEvent::Submit);

        Column::new()
            .push(name)
            .push(mass)
            .push(radius)
            .push(geometric_albedo)
            .push(semi_major_axis)
            .push(eccentricity)
            .push(inclination)
            .push(longitude_of_ascending_node)
            .push(argument_of_periapsis)
            .push(siderial_rotation_period)
            .push(rotation_axis)
            .push(submit_button)
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
