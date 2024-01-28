use super::Dialog;
use crate::gui::{gui_widget::PADDING, message::GuiMessage};
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
}

impl PlanetDialog {
    pub(crate) fn edit(planet: PlanetData) -> Self {
        PlanetDialog { planet }
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
        let name = TextInput::new("Name", &self.planet.get_name())
            .on_input(PlanetDialogEvent::NameChanged)
            .padding(PADDING);
        let mass = TextInput::new("Mass", &self.planet.get_mass().to_string())
            .on_input(PlanetDialogEvent::MassChanged)
            .padding(PADDING);
        let radius = TextInput::new("Radius", &self.planet.get_radius().to_string())
            .on_input(PlanetDialogEvent::RadiusChanged)
            .padding(PADDING);
        let semi_major_axis = TextInput::new(
            "Semi Major Axis",
            &self
                .planet
                .get_orbital_parameters()
                .get_semi_major_axis()
                .to_string(),
        )
        .on_input(PlanetDialogEvent::SemiMajorAxisChanged)
        .padding(PADDING);
        let eccentricity = TextInput::new(
            "Eccentricity",
            &self
                .planet
                .get_orbital_parameters()
                .get_eccentricity()
                .to_string(),
        )
        .on_input(PlanetDialogEvent::EccentricityChanged)
        .padding(PADDING);
        let inclination = TextInput::new(
            "Inclination",
            &self
                .planet
                .get_orbital_parameters()
                .get_inclination()
                .to_string(),
        )
        .on_input(PlanetDialogEvent::InclinationChanged)
        .padding(PADDING);
        let longitude_of_ascending_node = TextInput::new(
            "Longitude of Ascending Node",
            &self
                .planet
                .get_orbital_parameters()
                .get_longitude_of_ascending_node()
                .to_string(),
        )
        .on_input(PlanetDialogEvent::LongitudeOfAscendingNodeChanged)
        .padding(PADDING);
        let argument_of_periapsis = TextInput::new(
            "Argument of Periapsis",
            &self
                .planet
                .get_orbital_parameters()
                .get_argument_of_periapsis()
                .to_string(),
        )
        .on_input(PlanetDialogEvent::ArgumentOfPeriapsisChanged)
        .padding(PADDING);
        let siderial_rotation_period = TextInput::new(
            "Siderial Rotation Period",
            &self.planet.get_sideral_rotation_period().to_string(),
        )
        .on_input(PlanetDialogEvent::SiderialRotationPeriodChanged)
        .padding(PADDING);

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
