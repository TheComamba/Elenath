use astro_utils::{
    astro_display::AstroDisplay, units::luminous_intensity::luminous_intensity_to_illuminance,
};

use crate::model::{planet::Planet, star::Star};

pub(super) struct TableColData<T> {
    pub(super) header: &'static str,
    pub(super) content_closure: Box<dyn Fn(&T) -> String>,
}

impl TableColData<Planet> {
    pub(super) fn default_planet_col_data() -> Vec<TableColData<Planet>> {
        vec![
            TableColData {
                header: "Planet Name",
                content_closure: Box::new(|body| body.get_data().get_name().to_string()),
            },
            TableColData {
                header: "Mass",
                content_closure: Box::new(|body| body.get_data().get_mass().astro_display()),
            },
            TableColData {
                header: "Radius",
                content_closure: Box::new(|body| body.get_data().get_radius().astro_display()),
            },
            TableColData {
                header: "Density",
                content_closure: Box::new(|body| {
                    body.get_derived_data().get_density().astro_display()
                }),
            },
            TableColData {
                header: "Surface Gravity",
                content_closure: Box::new(|body| {
                    body.get_derived_data()
                        .get_surface_gravity()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Escape Velocity",
                content_closure: Box::new(|body| {
                    body.get_derived_data()
                        .get_escape_velocity()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Color",
                content_closure: Box::new(|body| body.get_data().get_color().astro_display()),
            },
            TableColData {
                header: "Geometric Albedo",
                content_closure: Box::new(|body| {
                    format!("{:.2}", body.get_data().get_geometric_albedo())
                }),
            },
            TableColData {
                header: "Black Body Temp.",
                content_closure: Box::new(|body| {
                    body.get_derived_data()
                        .get_black_body_temperature()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Semi-major Axis",
                content_closure: Box::new(|body| {
                    body.get_data()
                        .get_orbital_parameters()
                        .get_semi_major_axis()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Eccentricity",
                content_closure: Box::new(|body| {
                    format!(
                        "{:.2}",
                        body.get_data().get_orbital_parameters().get_eccentricity()
                    )
                }),
            },
            TableColData {
                header: "Inclination",
                content_closure: Box::new(|body| {
                    body.get_data()
                        .get_orbital_parameters()
                        .get_inclination()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Ascending Node",
                content_closure: Box::new(|body| {
                    body.get_data()
                        .get_orbital_parameters()
                        .get_longitude_of_ascending_node()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Arg. of Periapsis",
                content_closure: Box::new(|body| {
                    body.get_data()
                        .get_orbital_parameters()
                        .get_argument_of_periapsis()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Orbital Period",
                content_closure: Box::new(|body| {
                    body.get_derived_data().get_orbital_period().astro_display()
                }),
            },
            TableColData {
                header: "Orbital Resonance",
                content_closure: Box::new(|body| {
                    body.get_derived_data()
                        .get_orbital_resonance()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Sideral Day",
                content_closure: Box::new(|body| {
                    body.get_data()
                        .get_sideral_rotation_period()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Synodic Day",
                content_closure: Box::new(|body| {
                    body.get_derived_data()
                        .get_mean_synodic_day()
                        .astro_display()
                }),
            },
            TableColData {
                header: "Rotation Axis",
                content_closure: Box::new(|body| {
                    body.get_data().get_rotation_axis().astro_display()
                }),
            },
            TableColData {
                header: "Axial Tilt",
                content_closure: Box::new(|body| {
                    body.get_derived_data().get_axial_tilt().astro_display()
                }),
            },
        ]
    }
}

impl TableColData<Star> {
    pub(super) fn default_star_col_data() -> Vec<TableColData<Star>> {
        vec![
            TableColData {
                header: "Star Name",
                content_closure: Box::new(|body| body.get_data().unwrap().get_name().to_string()),
            },
            TableColData {
                header: "Mass",
                content_closure: Box::new(|body| {
                    if let Some(mass) = body.get_data().unwrap().get_mass() {
                        mass.astro_display()
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Radius",
                content_closure: Box::new(|body| {
                    if let Some(radius) = body.get_data().unwrap().get_radius() {
                        radius.astro_display()
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Luminous Intensity",
                content_closure: Box::new(|body| {
                    if let Some(luminous_intensity) =
                        body.get_data().unwrap().get_luminous_intensity()
                    {
                        luminous_intensity.astro_display()
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Temperature",
                content_closure: Box::new(|body| {
                    if let Some(temperature) = body.get_data().unwrap().get_temperature() {
                        temperature.astro_display()
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Age",
                content_closure: Box::new(|body| {
                    if let Some(age) = body.get_data().unwrap().get_age() {
                        age.astro_display()
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Distance",
                content_closure: Box::new(|body| {
                    if let Some(distance) = body.get_data().unwrap().get_distance() {
                        distance.astro_display()
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Vis. Mag.",
                content_closure: Box::new(|body| {
                    if let (Some(luminous_intensity), Some(distance)) = (
                        body.get_data().unwrap().get_luminous_intensity(),
                        body.get_data().unwrap().get_distance(),
                    ) {
                        let illuminance =
                            luminous_intensity_to_illuminance(luminous_intensity, distance);
                        illuminance.astro_display()
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Direction in Ecliptic",
                content_closure: Box::new(|body| {
                    body.get_data()
                        .unwrap()
                        .get_direction_in_ecliptic()
                        .astro_display()
                }),
            },
        ]
    }
}
