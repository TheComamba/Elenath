use crate::model::{distant_star::DistantStar, planet_data::PlanetData};

pub(super) struct TableColData<T> {
    pub(super) header: &'static str,
    pub(super) content_closure: Box<dyn Fn(&T) -> String>,
}

impl TableColData<PlanetData> {
    pub(super) fn default_planet_col_data() -> Vec<TableColData<PlanetData>> {
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

impl TableColData<DistantStar> {
    pub(super) fn default_star_col_data() -> Vec<TableColData<DistantStar>> {
        vec![TableColData {
            header: "Name",
            content_closure: Box::new(|body| body.get_stellar_properties().get_name().to_string()),
        }]
    }
}
