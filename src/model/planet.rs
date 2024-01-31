use super::part_of_celestial_system::PartOfCelestialSystem;
use astro_utils::{
    coordinates::cartesian::CartesianCoordinates, planets::planet_data::PlanetData,
    stars::star_data::StarData, units::time::Time,
};

pub(crate) struct Planet {
    data: PlanetData,
    pos: CartesianCoordinates,
    index: Option<usize>,
}

impl Planet {
    pub(crate) fn new(
        data: PlanetData,
        central_body: &StarData,
        time: Time,
        index: Option<usize>,
    ) -> Self {
        let mass = data.get_mass();
        let pos = data
            .get_orbital_parameters()
            .calculate_position(mass, central_body, time);
        Self { data, pos, index }
    }

    pub(crate) fn get_data(&self) -> &PlanetData {
        &self.data
    }

    pub(crate) fn get_position(&self) -> &CartesianCoordinates {
        &self.pos
    }
}

impl PartOfCelestialSystem for Planet {
    fn get_index(&self) -> Option<usize> {
        self.index
    }
}
