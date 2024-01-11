use astro_utils::{
    coordinates::cartesian::CartesianCoordinates, planets::planet_data::PlanetData,
    stars::star_data::StarData, units::time::Time,
};

pub(crate) struct Planet {
    data: PlanetData,
    pos: CartesianCoordinates,
}

impl Planet {
    pub(crate) fn new(data: PlanetData, central_body: &StarData, time: Time) -> Self {
        let mass = data.get_mass();
        let pos = data
            .get_orbital_parameters()
            .calculate_position(mass, central_body, time);
        Self { data, pos }
    }
}
