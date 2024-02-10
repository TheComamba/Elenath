use super::Dialog;
use crate::gui::{gui_widget::PADDING, message::GuiMessage, shared_widgets::edit};
use astro_utils::{
    color::sRGBColor,
    coordinates::direction::Direction,
    planets::{
        orbit_parameters::OrbitParameters, planet_data::PlanetData,
        random_planets::generate_random_planet,
    },
    units::{
        angle::ANGLE_ZERO,
        distance::{distance_to_earth_radii, DISTANCE_ZERO, EARTH_RADIUS},
        mass::{EARTH_MASS, MASS_ZERO},
        time::TIME_ZERO,
    },
};
use iced::{
    widget::{component, Button, Column, Component, Row, Text},
    Alignment, Element, Renderer,
};
use simple_si_units::{
    base::{Distance, Time},
    geometry::Angle,
};

#[derive(Debug, Clone)]
pub(crate) struct PlanetDialog {
    planet: PlanetData,
    planet_index: Option<usize>,
    mass_string: String,
    radius_string: String,
    color_string: String,
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
        let mut dialog = PlanetDialog {
            planet: planet.clone(),
            planet_index: Some(planet_index),
            mass_string: String::new(),
            radius_string: String::new(),
            color_string: String::new(),
            geometric_albedo_string: String::new(),
            semi_major_axis_string: String::new(),
            eccentricity_string: String::new(),
            inclination_string: String::new(),
            longitude_of_ascending_node_string: String::new(),
            argument_of_periapsis_string: String::new(),
            siderial_rotation_period_string: String::new(),
            rotation_axis_string: String::new(),
        };
        dialog.fill_string_members();
        dialog
    }

    pub(crate) fn new() -> Self {
        let mut dialog = PlanetDialog {
            planet: PlanetData::new(
                String::new(),
                MASS_ZERO,
                DISTANCE_ZERO,
                0.0,
                sRGBColor::from_sRGB(0., 0., 0.),
                TIME_ZERO,
                OrbitParameters::new(DISTANCE_ZERO, 0.0, ANGLE_ZERO, ANGLE_ZERO, ANGLE_ZERO),
                Direction::Z,
            ),
            planet_index: None,
            mass_string: String::new(),
            radius_string: String::new(),
            color_string: String::new(),
            geometric_albedo_string: String::new(),
            semi_major_axis_string: String::new(),
            eccentricity_string: String::new(),
            inclination_string: String::new(),
            longitude_of_ascending_node_string: String::new(),
            argument_of_periapsis_string: String::new(),
            siderial_rotation_period_string: String::new(),
            rotation_axis_string: String::new(),
        };
        dialog.fill_string_members();
        dialog
    }

    fn fill_string_members(&mut self) {
        self.mass_string = self.planet.get_mass().to_earth_mass().to_string();
        self.radius_string = distance_to_earth_radii(&self.planet.get_radius()).to_string();
        self.color_string = serde_json::to_string(self.planet.get_color()).unwrap();
        self.geometric_albedo_string = self.planet.get_geometric_albedo().to_string();
        self.semi_major_axis_string = self
            .planet
            .get_orbital_parameters()
            .get_semi_major_axis()
            .to_au()
            .to_string();
        self.eccentricity_string = self
            .planet
            .get_orbital_parameters()
            .get_eccentricity()
            .to_string();
        self.inclination_string = self
            .planet
            .get_orbital_parameters()
            .get_inclination()
            .to_degrees()
            .to_string();
        self.longitude_of_ascending_node_string = self
            .planet
            .get_orbital_parameters()
            .get_longitude_of_ascending_node()
            .to_degrees()
            .to_string();
        self.argument_of_periapsis_string = self
            .planet
            .get_orbital_parameters()
            .get_argument_of_periapsis()
            .to_degrees()
            .to_string();
        self.siderial_rotation_period_string = self
            .planet
            .get_sideral_rotation_period()
            .to_days()
            .to_string();
        self.rotation_axis_string = serde_json::to_string(self.planet.get_rotation_axis()).unwrap();
    }

    fn edit_column(&self) -> Element<'_, PlanetDialogEvent> {
        let randomize_button =
            Button::new(Text::new("Randomize")).on_press(PlanetDialogEvent::Randomize);

        let name = edit(
            "Name",
            self.planet.get_name(),
            "",
            PlanetDialogEvent::NameChanged,
            &Some(self.planet.get_name()),
        );
        let mass = edit(
            "Mass",
            &self.mass_string,
            "Earth Masses",
            PlanetDialogEvent::MassChanged,
            &Some(self.planet.get_mass()),
        );
        let radius = edit(
            "Radius",
            &self.radius_string,
            "Earth Radii",
            PlanetDialogEvent::RadiusChanged,
            &Some(self.planet.get_radius()),
        );
        let color = edit(
            "Color",
            &self.color_string,
            "",
            PlanetDialogEvent::ColorChanged,
            &Some(self.planet.get_color()),
        );
        let geometric_albedo = edit(
            "Geometric Albedo",
            &self.geometric_albedo_string,
            "",
            PlanetDialogEvent::GeometricAlbedoChanged,
            &Some(self.planet.get_geometric_albedo()),
        );
        let semi_major_axis = edit(
            "Semi-major Axis",
            &self.semi_major_axis_string,
            "AU",
            PlanetDialogEvent::SemiMajorAxisChanged,
            &Some(self.planet.get_orbital_parameters().get_semi_major_axis()),
        );
        let eccentricity = edit(
            "Eccentricity",
            &self.eccentricity_string,
            "",
            PlanetDialogEvent::EccentricityChanged,
            &Some(self.planet.get_orbital_parameters().get_eccentricity()),
        );
        let inclination = edit(
            "Inclination",
            &self.inclination_string,
            "°",
            PlanetDialogEvent::InclinationChanged,
            &Some(self.planet.get_orbital_parameters().get_inclination()),
        );
        let longitude_of_ascending_node = edit(
            "Ascending Node",
            &self.longitude_of_ascending_node_string,
            "°",
            PlanetDialogEvent::LongitudeOfAscendingNodeChanged,
            &Some(
                self.planet
                    .get_orbital_parameters()
                    .get_longitude_of_ascending_node(),
            ),
        );
        let argument_of_periapsis = edit(
            "Arg. of Periapsis",
            &self.argument_of_periapsis_string,
            "°",
            PlanetDialogEvent::ArgumentOfPeriapsisChanged,
            &Some(
                self.planet
                    .get_orbital_parameters()
                    .get_argument_of_periapsis(),
            ),
        );
        let siderial_rotation_period = edit(
            "Siderial Day",
            &self.siderial_rotation_period_string,
            "Earth Days",
            PlanetDialogEvent::SiderialRotationPeriodChanged,
            &Some(self.planet.get_sideral_rotation_period()),
        );
        let rotation_axis = edit(
            "Rotation Axis",
            &self.rotation_axis_string,
            "",
            PlanetDialogEvent::RotationAxisChanged,
            &Some(self.planet.get_rotation_axis()),
        );

        let submit_button = Button::new(Text::new("Submit")).on_press(PlanetDialogEvent::Submit);

        Column::new()
            .push(randomize_button)
            .push(name)
            .push(mass)
            .push(radius)
            .push(color)
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

    fn additional_info_column(&self) -> Element<'_, PlanetDialogEvent> {
        Column::new()
            .spacing(PADDING)
            .width(iced::Length::Fill)
            .align_items(Alignment::Center)
            .into()
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
    ColorChanged(String),
    GeometricAlbedoChanged(String),
    SemiMajorAxisChanged(String),
    EccentricityChanged(String),
    InclinationChanged(String),
    LongitudeOfAscendingNodeChanged(String),
    ArgumentOfPeriapsisChanged(String),
    SiderialRotationPeriodChanged(String),
    RotationAxisChanged(String),
    Randomize,
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
                if let Ok(mass) = mass_string.parse::<f64>() {
                    self.planet.set_mass(mass * EARTH_MASS);
                    self.mass_string = mass_string;
                }
            }
            PlanetDialogEvent::RadiusChanged(radius_string) => {
                if let Ok(radius) = radius_string.parse::<f64>() {
                    self.planet.set_radius(radius * EARTH_RADIUS);
                    self.radius_string = radius_string;
                }
            }
            PlanetDialogEvent::ColorChanged(color_string) => {
                if let Ok(color) = serde_json::from_str::<sRGBColor>(&color_string) {
                    self.planet.set_color(color);
                }
                self.color_string = color_string;
            }
            PlanetDialogEvent::GeometricAlbedoChanged(geometric_albedo_string) => {
                if let Ok(geometric_albedo) = geometric_albedo_string.parse::<f64>() {
                    self.planet.set_geometric_albedo(geometric_albedo);
                    self.geometric_albedo_string = geometric_albedo_string;
                }
            }
            PlanetDialogEvent::SemiMajorAxisChanged(semi_major_axis_string) => {
                if let Ok(semi_major_axis) = semi_major_axis_string.parse::<f64>() {
                    self.planet
                        .set_semi_major_axis(Distance::from_au(semi_major_axis));
                    self.semi_major_axis_string = semi_major_axis_string;
                }
            }
            PlanetDialogEvent::EccentricityChanged(eccentricity_string) => {
                if let Ok(eccentricity) = eccentricity_string.parse::<f64>() {
                    self.planet.set_eccentricity(eccentricity);
                    self.eccentricity_string = eccentricity_string;
                }
            }
            PlanetDialogEvent::InclinationChanged(inclination_string) => {
                if let Ok(inclination) = inclination_string.parse::<f64>() {
                    self.planet
                        .set_inclination(Angle::from_degrees(inclination));
                    self.inclination_string = inclination_string;
                }
            }
            PlanetDialogEvent::LongitudeOfAscendingNodeChanged(
                longitude_of_ascending_node_string,
            ) => {
                if let Ok(longitude_of_ascending_node) =
                    longitude_of_ascending_node_string.parse::<f64>()
                {
                    self.planet
                        .set_longitude_of_ascending_node(Angle::from_degrees(
                            longitude_of_ascending_node,
                        ));
                    self.longitude_of_ascending_node_string = longitude_of_ascending_node_string;
                }
            }
            PlanetDialogEvent::ArgumentOfPeriapsisChanged(argument_of_periapsis_string) => {
                if let Ok(argument_of_periapsis) = argument_of_periapsis_string.parse::<f64>() {
                    self.planet
                        .set_argument_of_periapsis(Angle::from_degrees(argument_of_periapsis));
                    self.argument_of_periapsis_string = argument_of_periapsis_string;
                }
            }
            PlanetDialogEvent::SiderialRotationPeriodChanged(siderial_rotation_period_string) => {
                if let Ok(siderial_rotation_period) = siderial_rotation_period_string.parse::<f64>()
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
            PlanetDialogEvent::Randomize => {
                self.planet = generate_random_planet();
                self.fill_string_members();
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
        Row::new()
            .push(self.edit_column())
            .push(self.additional_info_column())
            .into()
    }
}
