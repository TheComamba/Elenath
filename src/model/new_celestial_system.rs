use super::celestial_system::{CelestialSystem, SystemType};
use crate::error::ElenathError;
use astro_utils::{
    data::{planets::*, stars::*},
    stars::{
        gaia_data::fetch_brightest_stars,
        random_stars::{generate_random_star, generate_random_stars},
    },
    units::length::Length,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneratedCentralBody {
    Sun,
    RandomStar,
}

pub(crate) fn solar_system(load_gaia_data: bool) -> Result<CelestialSystem, ElenathError> {
    let mut system = CelestialSystem::new(SystemType::Real, SUN_DATA.to_star_data());
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

    for data in BRIGHTEST_STARS.iter() {
        system.add_star_from_data(data.to_star_data());
    }

    if load_gaia_data {
        let stars = fetch_brightest_stars()?;
        system.add_star_appearances_without_duplicates(stars);
    }

    Ok(system)
}

pub(crate) fn generated_system(
    central_body: &GeneratedCentralBody,
    max_distance: Length,
) -> Result<CelestialSystem, ElenathError> {
    let central_body_data = match central_body {
        GeneratedCentralBody::Sun => SUN_DATA.to_star_data(),
        GeneratedCentralBody::RandomStar => generate_random_star(None)?,
    };

    let mut system = CelestialSystem::new(SystemType::Generated, central_body_data);

    let distant_stars = generate_random_stars(max_distance)?;
    for star in distant_stars {
        system.add_star_from_data(star);
    }

    Ok(system)
}
