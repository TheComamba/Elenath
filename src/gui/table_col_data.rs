use super::Gui;
use crate::model::celestial_body_data::CelestialBodyData;

pub(super) struct TableColData {
    pub(super) header: &'static str,
    pub(super) content_closure: Box<dyn Fn(&CelestialBodyData) -> String>,
}

impl TableColData {
    pub(super) const fn new(
        header: &'static str,
        content_closure: Box<dyn Fn(&CelestialBodyData) -> String>,
    ) -> TableColData {
        TableColData {
            header,
            content_closure,
        }
    }
}

impl Gui {
    pub(super) fn init_table_col_data(&mut self) {
        self.table_col_data.push(TableColData::new(
            "Name",
            Box::new(|body| body.get_name().to_string()),
        ));
        self.table_col_data.push(TableColData::new(
            "Mass",
            Box::new(|body| format!("{}", body.get_mass())),
        ));
        self.table_col_data.push(TableColData::new(
            "Radius",
            Box::new(|body| format!("{}", body.get_radius())),
        ));
        self.table_col_data.push(TableColData::new(
            "Albedo",
            Box::new(|body| format!("{}", body.get_albedo())),
        ));
        self.table_col_data.push(TableColData::new(
            "Semi-major axis",
            Box::new(|body| format!("{}", body.get_orbital_parameters().get_semi_major_axis())),
        ));
        self.table_col_data.push(TableColData::new(
            "Eccentricity",
            Box::new(|body| format!("{}", body.get_orbital_parameters().get_eccentricity())),
        ));
        self.table_col_data.push(TableColData::new(
            "Inclination",
            Box::new(|body| format!("{}", body.get_orbital_parameters().get_inclination())),
        ));
        self.table_col_data.push(TableColData::new(
            "Longitude of ascending node",
            Box::new(|body| {
                format!(
                    "{}",
                    body.get_orbital_parameters()
                        .get_longitude_of_ascending_node()
                )
            }),
        ));
        self.table_col_data.push(TableColData::new(
            "Argument of periapsis",
            Box::new(|body| {
                format!(
                    "{}",
                    body.get_orbital_parameters().get_argument_of_periapsis()
                )
            }),
        ));
    }
}
