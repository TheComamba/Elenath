use super::celestial_system::{CelestialSystem, SystemType};
use crate::error::ElenathError;
use astro_utils::{
    real_data::{planets::*, stars::all::get_many_stars, stars::SUN},
    stars::{
        gaia_data::fetch_brightest_stars,
        random_stars::{generate_random_star, generate_random_stars},
    },
};
use simple_si_units::base::Distance;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneratedCentralBody {
    Sun,
    RandomStar,
}

pub(crate) fn solar_system(load_gaia_data: bool) -> Result<CelestialSystem, ElenathError> {
    let mut system = CelestialSystem::new(SystemType::Real, SUN.to_star_data());
    system.add_planet_data(MERCURY.to_planet_data());
    system.add_planet_data(VENUS.to_planet_data());
    system.add_planet_data(EARTH.to_planet_data());
    system.add_planet_data(MARS.to_planet_data());
    system.add_planet_data(CERES.to_planet_data());
    system.add_planet_data(JUPITER.to_planet_data());
    system.add_planet_data(SATURN.to_planet_data());
    system.add_planet_data(URANUS.to_planet_data());
    system.add_planet_data(NEPTUNE.to_planet_data());
    system.add_planet_data(PLUTO.to_planet_data());

    let stars = get_many_stars().iter().map(|s| s.to_star_data()).collect();
    system.add_stars_from_data(stars);

    if load_gaia_data {
        let stars = fetch_brightest_stars()?;
        system.add_star_appearances_without_duplicates(stars);
    }

    Ok(system)
}

pub(crate) fn generated_system(
    central_body: &GeneratedCentralBody,
    max_distance: Distance<f64>,
) -> Result<CelestialSystem, ElenathError> {
    let central_body_data = match central_body {
        GeneratedCentralBody::Sun => SUN.to_star_data(),
        GeneratedCentralBody::RandomStar => generate_random_star(None)?,
    };

    let mut system = CelestialSystem::new(SystemType::Generated, central_body_data);

    let distant_stars = generate_random_stars(max_distance)?;
    system.add_stars_from_data(distant_stars);

    Ok(system)
}
