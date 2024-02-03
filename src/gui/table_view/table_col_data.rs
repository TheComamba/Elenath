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
                content_closure: Box::new(|body| format!("{}", body.get_data().get_mass())),
            },
            TableColData {
                header: "Radius",
                content_closure: Box::new(|body| format!("{}", body.get_data().get_radius())),
            },
            TableColData {
                header: "Color",
                content_closure: Box::new(|body| format!("{}", body.get_data().get_color())),
            },
            TableColData {
                header: "Geometric Albedo",
                content_closure: Box::new(|body| {
                    format!("{}", body.get_data().get_geometric_albedo())
                }),
            },
            TableColData {
                header: "Semi-major Axis",
                content_closure: Box::new(|body| {
                    format!(
                        "{}",
                        body.get_data()
                            .get_orbital_parameters()
                            .get_semi_major_axis()
                    )
                }),
            },
            TableColData {
                header: "Eccentricity",
                content_closure: Box::new(|body| {
                    format!(
                        "{}",
                        body.get_data().get_orbital_parameters().get_eccentricity()
                    )
                }),
            },
            TableColData {
                header: "Inclination",
                content_closure: Box::new(|body| {
                    format!(
                        "{}",
                        body.get_data().get_orbital_parameters().get_inclination()
                    )
                }),
            },
            TableColData {
                header: "Ascending Node",
                content_closure: Box::new(|body| {
                    format!(
                        "{}",
                        body.get_data()
                            .get_orbital_parameters()
                            .get_longitude_of_ascending_node()
                    )
                }),
            },
            TableColData {
                header: "Arg. of Periapsis",
                content_closure: Box::new(|body| {
                    format!(
                        "{}",
                        body.get_data()
                            .get_orbital_parameters()
                            .get_argument_of_periapsis()
                    )
                }),
            },
            TableColData {
                header: "Sideral Day",
                content_closure: Box::new(|body| {
                    format!("{}", body.get_data().get_sideral_rotation_period())
                }),
            },
            TableColData {
                header: "Rotation Axis",
                content_closure: Box::new(|body| {
                    format!("{}", body.get_data().get_rotation_axis())
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
                        format!("{}", mass)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Radius",
                content_closure: Box::new(|body| {
                    if let Some(radius) = body.get_data().unwrap().get_radius() {
                        format!("{}", radius)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Luminosity",
                content_closure: Box::new(|body| {
                    if let Some(luminosity) = body.get_data().unwrap().get_luminosity() {
                        format!("{}", luminosity)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Temperature",
                content_closure: Box::new(|body| {
                    if let Some(temperature) = body.get_data().unwrap().get_temperature() {
                        format!("{}", temperature)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Direction in Ecliptic",
                content_closure: Box::new(|body| {
                    format!("{}", body.get_data().unwrap().get_direction_in_ecliptic())
                }),
            },
            TableColData {
                header: "Distance",
                content_closure: Box::new(|body| {
                    if let Some(distance) = body.get_data().unwrap().get_distance() {
                        format!("{}", distance)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Apparent Brightness",
                content_closure: Box::new(|body| {
                    if let (Some(abs_mag), Some(distance)) = (
                        body.get_data().unwrap().get_luminosity(),
                        body.get_data().unwrap().get_distance(),
                    ) {
                        let apparent_magnitude = abs_mag.to_illuminance(&distance);
                        format!("{}", apparent_magnitude)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
        ]
    }
}
