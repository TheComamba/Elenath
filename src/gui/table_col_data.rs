use crate::model::celestial_body_data::CelestialBodyData;

pub(super) struct TableColData {
    pub(super) header: &'static str,
    pub(super) content_closure: Box<dyn Fn(&CelestialBodyData) -> String>,
}

impl TableColData {
    const fn new(
        header: &'static str,
        content_closure: Box<dyn Fn(&CelestialBodyData) -> String>,
    ) -> TableColData {
        TableColData {
            header,
            content_closure,
        }
    }
    pub(super) fn default_table_col_data() -> Vec<TableColData> {
        vec![
            TableColData::new("Name", Box::new(|body| body.get_name().to_string())),
            TableColData::new("Mass", Box::new(|body| format!("{}", body.get_mass()))),
            TableColData::new("Radius", Box::new(|body| format!("{}", body.get_radius()))),
            TableColData::new("Albedo", Box::new(|body| format!("{}", body.get_albedo()))),
            TableColData::new(
                "Semi-major axis",
                Box::new(|body| format!("{}", body.get_orbital_parameters().get_semi_major_axis())),
            ),
            TableColData::new(
                "Eccentricity",
                Box::new(|body| format!("{}", body.get_orbital_parameters().get_eccentricity())),
            ),
            TableColData::new(
                "Inclination",
                Box::new(|body| format!("{}", body.get_orbital_parameters().get_inclination())),
            ),
            TableColData::new(
                "Longitude of ascending node",
                Box::new(|body| {
                    format!(
                        "{}",
                        body.get_orbital_parameters()
                            .get_longitude_of_ascending_node()
                    )
                }),
            ),
            TableColData::new(
                "Argument of periapsis",
                Box::new(|body| {
                    format!(
                        "{}",
                        body.get_orbital_parameters().get_argument_of_periapsis()
                    )
                }),
            ),
            TableColData::new(
                "Sideral rotation period",
                Box::new(|body| format!("{}", body.get_sideral_rotation_period())),
            ),
            TableColData::new(
                "Rotation axis",
                Box::new(|body| format!("{}", body.get_rotation_axis())),
            ),
        ]
    }
}
