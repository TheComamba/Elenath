use super::{planet::Planet, star::Star};
use astro_utils::{
    planets::planet_data::PlanetData,
    stars::{
        gaia_data::star_is_already_known, star_appearance::StarAppearance, star_data::StarData,
    },
    units::time::Time,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub(crate) struct CelestialSystem {
    central_body: Star,
    planets: Vec<PlanetData>,
    distant_stars: Vec<Star>,
}

impl CelestialSystem {
    pub(crate) fn new(central_body_data: StarData) -> Self {
        let central_body = Star::from_data(central_body_data);
        CelestialSystem {
            central_body,
            planets: vec![],
            distant_stars: vec![],
        }
    }

    pub(crate) fn write_to_file(&self, path: PathBuf) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub(crate) fn read_from_file(path: PathBuf) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let celestial_system = serde_json::from_reader(reader)?;
        Ok(celestial_system)
    }

    pub(crate) fn add_planet_data(&mut self, planet: PlanetData) {
        self.planets.push(planet);
    }

    pub(crate) fn add_star_from_data(&mut self, star_data: StarData) {
        self.distant_stars.push(Star::from_data(star_data));
    }

    pub(crate) fn add_star_from_appearance(&mut self, star_appearance: StarAppearance) {
        self.distant_stars
            .push(Star::from_appearance(star_appearance));
    }

    pub(crate) fn add_star_appearances_without_duplicates(
        &mut self,
        star_appearances: Vec<StarAppearance>,
    ) {
        for star_appearance in star_appearances {
            if !star_is_already_known(
                &star_appearance,
                &self
                    .distant_stars
                    .iter()
                    .map(|s| s.get_appearance())
                    .collect::<Vec<_>>(),
            ) {
                self.distant_stars
                    .push(Star::from_appearance(star_appearance));
            }
        }
    }

    pub(crate) fn get_central_body(&self) -> &Star {
        &self.central_body
    }

    pub(crate) fn get_planet_data(&self) -> Vec<&PlanetData> {
        let mut bodies = Vec::new();
        for planet in &self.planets {
            bodies.push(planet);
        }
        bodies
    }

    pub(crate) fn get_planets_at_time(&self, time: Time) -> Vec<&Planet> {
        let mut bodies = Vec::new();
        for planet_data in &self.planets {
            let planet = Planet::new(planet_data, self.central_body.get_data(), time);
            bodies.push(planet);
        }
        bodies
    }

    pub(crate) fn get_stars(&self) -> Vec<&Star> {
        let mut bodies = Vec::new();
        bodies.push(&self.central_body);
        for star in &self.distant_stars {
            bodies.push(star);
        }
        bodies
    }
}
