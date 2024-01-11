use astro_utils::{planets::planet::Planet, stars::star_data::StarData};

pub(super) struct TableColData<T> {
    pub(super) header: &'static str,
    pub(super) content_closure: Box<dyn Fn(&T) -> String>,
}

impl TableColData<Planet> {
    pub(super) fn default_planet_col_data() -> Vec<TableColData<Planet>> {
        vec![
            TableColData {
                header: "Name",
                content_closure: Box::new(|body| body.get_name().to_string()),
            },
            TableColData {
                header: "Mass",
                content_closure: Box::new(|body| format!("{}", body.get_mass())),
            },
            TableColData {
                header: "Radius",
                content_closure: Box::new(|body| format!("{}", body.get_radius())),
            },
            TableColData {
                header: "Geometric Albedo",
                content_closure: Box::new(|body| format!("{}", body.get_geometric_albedo())),
            },
            TableColData {
                header: "Semi-major axis",
                content_closure: Box::new(|body| {
                    format!("{}", body.get_orbital_parameters().get_semi_major_axis())
                }),
            },
            TableColData {
                header: "Eccentricity",
                content_closure: Box::new(|body| {
                    format!("{}", body.get_orbital_parameters().get_eccentricity())
                }),
            },
            TableColData {
                header: "Inclination",
                content_closure: Box::new(|body| {
                    format!("{}", body.get_orbital_parameters().get_inclination())
                }),
            },
            TableColData {
                header: "Longitude of ascending node",
                content_closure: Box::new(|body| {
                    format!(
                        "{}",
                        body.get_orbital_parameters()
                            .get_longitude_of_ascending_node()
                    )
                }),
            },
            TableColData {
                header: "Argument of periapsis",
                content_closure: Box::new(|body| {
                    format!(
                        "{}",
                        body.get_orbital_parameters().get_argument_of_periapsis()
                    )
                }),
            },
            TableColData {
                header: "Sideral rotation period",
                content_closure: Box::new(|body| format!("{}", body.get_sideral_rotation_period())),
            },
            TableColData {
                header: "Rotation axis",
                content_closure: Box::new(|body| format!("{}", body.get_rotation_axis())),
            },
        ]
    }
}

impl TableColData<StarData> {
    pub(super) fn default_star_col_data() -> Vec<TableColData<StarData>> {
        vec![
            TableColData {
                header: "Name",
                content_closure: Box::new(|body| body.get_name().to_string()),
            },
            TableColData {
                header: "Mass",
                content_closure: Box::new(|body| {
                    if let Some(mass) = body.get_mass() {
                        format!("{}", mass)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Radius",
                content_closure: Box::new(|body| {
                    if let Some(radius) = body.get_radius() {
                        format!("{}", radius)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Absolute magnitude",
                content_closure: Box::new(|body| {
                    if let Some(luminosity) = body.get_absolute_magnitude() {
                        format!("{}", luminosity)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Temperature",
                content_closure: Box::new(|body| {
                    if let Some(temperature) = body.get_temperature() {
                        format!("{}", temperature)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Direction in Ecliptic",
                content_closure: Box::new(|body| format!("{}", body.get_direction_in_ecliptic())),
            },
            TableColData {
                header: "Distance",
                content_closure: Box::new(|body| {
                    if let Some(distance) = body.get_distance() {
                        format!("{}", distance)
                    } else {
                        String::from("N/A")
                    }
                }),
            },
            TableColData {
                header: "Apparent magnitude",
                content_closure: Box::new(|body| {
                    if let (Some(abs_mag), Some(distance)) =
                        (body.get_absolute_magnitude(), body.get_distance)
                    {
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
