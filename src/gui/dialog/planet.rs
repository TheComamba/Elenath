use super::Dialog;
use crate::gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::edit};
use astro_utils::{
    color::sRGBColor,
    coordinates::direction::Direction,
    planets::{orbit_parameters::OrbitParameters, planet_data::PlanetData},
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
};
use iced::{
    widget::{component, Column, Component, TextInput},
    Element, Renderer,
};

#[derive(Debug, Clone)]
pub(crate) struct PlanetDialog {
    planet: PlanetData,
    mass_string: String,
    radius_string: String,
    semi_major_axis_string: String,
    eccentricity_string: String,
    inclination_string: String,
    longitude_of_ascending_node_string: String,
    argument_of_periapsis_string: String,
    siderial_rotation_period_string: String,
}

impl PlanetDialog {
    pub(crate) fn edit(planet: PlanetData) -> Self {
        PlanetDialog {
            planet: planet.clone(),
            mass_string: planet.get_mass().as_earth_masses().to_string(),
            radius_string: planet.get_radius().as_earth_radii().to_string(),
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
            mass_string: String::new(),
            radius_string: String::new(),
            semi_major_axis_string: String::new(),
            eccentricity_string: String::new(),
            inclination_string: String::new(),
            longitude_of_ascending_node_string: String::new(),
            argument_of_periapsis_string: String::new(),
            siderial_rotation_period_string: String::new(),
        }
    }
}

impl Dialog for PlanetDialog {
    fn header(&self) -> String {
        "Create/Edit Planet".to_string()
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
    SemiMajorAxisChanged(String),
    EccentricityChanged(String),
    InclinationChanged(String),
    LongitudeOfAscendingNodeChanged(String),
    ArgumentOfPeriapsisChanged(String),
    SiderialRotationPeriodChanged(String),
    Submit,
}

impl Component<GuiMessage, Renderer> for PlanetDialog {
    type State = ();

    type Event = PlanetDialogEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMessage> {
        None
    }

    fn view(&self, _state: &Self::State) -> iced::Element<'_, Self::Event> {
        let name = edit("Name", self.planet.get_name(), "", "", |t| {
            PlanetDialogEvent::NameChanged(t)
        });
        let mass = edit("Mass", &self.mass_string, "", "Earth Masses", |t| {
            PlanetDialogEvent::MassChanged(t)
        });
        let radius = edit("Radius", &self.radius_string, "", "Earth Radii", |t| {
            PlanetDialogEvent::RadiusChanged(t)
        });
        let semi_major_axis = edit(
            "Semi Major Axis",
            &self.semi_major_axis_string,
            "",
            "AU",
            |t| PlanetDialogEvent::SemiMajorAxisChanged(t),
        );
        let eccentricity = edit("Eccentricity", &self.eccentricity_string, "", "", |t| {
            PlanetDialogEvent::EccentricityChanged(t)
        });
        let inclination = edit("Inclination", &self.inclination_string, "", "°", |t| {
            PlanetDialogEvent::InclinationChanged(t)
        });
        let longitude_of_ascending_node = edit(
            "Longitude of Ascending Node",
            &self.longitude_of_ascending_node_string,
            "",
            "°",
            |t| PlanetDialogEvent::LongitudeOfAscendingNodeChanged(t),
        );
        let argument_of_periapsis = edit(
            "Argument of Periapsis",
            &self.argument_of_periapsis_string,
            "",
            "°",
            |t| PlanetDialogEvent::ArgumentOfPeriapsisChanged(t),
        );
        let siderial_rotation_period = edit(
            "Siderial Rotation Period",
            &self.siderial_rotation_period_string,
            "",
            "Earth Days",
            |t| PlanetDialogEvent::SiderialRotationPeriodChanged(t),
        );

        Column::new()
            .push(name)
            .push(mass)
            .push(radius)
            .push(semi_major_axis)
            .push(eccentricity)
            .push(inclination)
            .push(longitude_of_ascending_node)
            .push(argument_of_periapsis)
            .push(siderial_rotation_period)
            .spacing(PADDING)
            .into()
    }
}
