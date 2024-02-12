use super::part_of_celestial_system::{BodyType, PartOfCelestialSystem};
use astro_utils::{
    coordinates::cartesian::CartesianCoordinates,
    planets::{derived_data::DerivedPlanetData, planet_data::PlanetData},
};
use simple_si_units::base::{Mass, Time};

pub(crate) struct Planet {
    data: PlanetData,
    derived_data: DerivedPlanetData,
    pos: CartesianCoordinates,
    index: Option<usize>,
}

impl Planet {
    pub(crate) fn new(
        data: PlanetData,
        central_body_mass: Mass<f64>,
        previous: Option<&DerivedPlanetData>,
        time: Time<f64>,
        index: Option<usize>,
    ) -> Self {
        let derived_data = DerivedPlanetData::new(&data, central_body_mass, previous);
        let pos = data.get_orbital_parameters().calculate_position(
            data.get_mass(),
            central_body_mass,
            time,
        );
        Self {
            data,
            derived_data,
            pos,
            index,
        }
    }

    pub(crate) fn get_data(&self) -> &PlanetData {
        &self.data
    }

    pub(crate) fn get_derived_data(&self) -> &DerivedPlanetData {
        &self.derived_data
    }

    pub(crate) fn get_position(&self) -> &CartesianCoordinates {
        &self.pos
    }
}

impl PartOfCelestialSystem for Planet {
    fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn get_body_type(&self) -> BodyType {
        BodyType::Planet
    }
}
