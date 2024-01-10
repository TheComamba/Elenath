use std::path::PathBuf;

use super::{celestial_body::CelestialBody, star::Star};
use astro_utils::{
    planets::planet::Planet,
    stars::{gaia_data::star_is_already_known, star::StarData},
    units::time::Time,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CelestialSystem {
    central_body: Star,
    planets: Vec<Planet>,
    distant_stars: Vec<Star>,
}

impl CelestialSystem {
    pub(crate) fn new(central_body: StarData) -> Self {
        let central_body = central_body;
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

    pub(crate) fn get_central_body(&self) -> &StarData {
        &self.central_body
    }

    pub(crate) fn get_current_data(&self, time: Time) -> Vec<CelestialBody> {
        let mut bodies = Vec::new();
        let central_body = CelestialBody::central_body(self.central_body.clone());
        bodies.push(central_body.clone());
        for planet in &self.planets {
            bodies.push(CelestialBody::from_planet(planet, &self.central_body, time));
        }
        for star in &self.distant_stars {
            bodies.push(CelestialBody::from_distant_star(star));
        }
        bodies
    }

    pub(crate) fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }

    pub(crate) fn add_distant_star(&mut self, star: StarData) {
        self.distant_stars.push(star);
    }

    pub(crate) fn add_distant_stars(&mut self, stars: Vec<StarData>) {
        self.distant_stars.extend(stars);
    }

    pub(crate) fn add_distant_stars_without_duplicates(&mut self, stars: Vec<Star>) {
        for star in stars {
            if !star_is_already_known(&star, &self.distant_stars.iter().collect::<Vec<_>>()) {
                self.distant_stars.push(star);
            }
        }
    }

    pub(crate) fn get_planets_data(&self) -> Vec<&Planet> {
        let mut bodies = Vec::new();
        for planet in &self.planets {
            bodies.push(planet);
        }
        bodies
    }

    pub(crate) fn get_star_data(&self) -> Vec<&Star> {
        let mut bodies = Vec::new();
        bodies.push(&self.central_body);
        for star in &self.distant_stars {
            bodies.push(star);
        }
        bodies
    }
}
