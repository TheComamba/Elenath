use super::{planet::Planet, star::Star};
use astro_utils::{
    coordinates::cartesian::CartesianCoordinates,
    planets::planet_data::PlanetData,
    stars::{
        gaia_data::star_is_already_known, star_appearance::StarAppearance, star_data::StarData,
    },
};
use serde::{Deserialize, Serialize};
use simple_si_units::base::Time;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CelestialSystem {
    system_type: SystemType,
    central_body: StarData,
    planets: Vec<PlanetData>,
    distant_stars: Vec<Star>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SystemType {
    Real,
    Generated,
}

impl CelestialSystem {
    pub(crate) fn new(system_type: SystemType, mut central_body: StarData) -> Self {
        central_body.set_distance(None);
        CelestialSystem {
            system_type,
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
        self.sort_planets_by_semimajor_axis();
    }

    pub(crate) fn overwrite_planet_data(&mut self, index: usize, planet: PlanetData) {
        self.planets[index] = planet;
        self.sort_planets_by_semimajor_axis();
    }

    fn sort_planets_by_semimajor_axis(&mut self) {
        self.planets.sort_by(|a, b| {
            a.get_orbital_parameters()
                .get_semi_major_axis()
                .partial_cmp(&b.get_orbital_parameters().get_semi_major_axis())
                .unwrap()
        });
    }

    pub(crate) fn add_star_from_data(&mut self, star_data: StarData) {
        let index = self.distant_stars.len();
        self.distant_stars
            .push(Star::from_data(star_data, Some(index)));
        self.sort_stars_by_brightness();
    }

    pub(crate) fn add_star_appearances_without_duplicates(
        &mut self,
        star_appearances: Vec<StarAppearance>,
    ) {
        for star_appearance in star_appearances {
            let known_stars = self.get_distant_star_appearances();
            if !star_is_already_known(&star_appearance, &known_stars[..]) {
                let index = self.distant_stars.len();
                self.distant_stars
                    .push(Star::from_appearance(star_appearance, Some(index)));
            }
        }
        self.sort_stars_by_brightness();
    }

    pub(crate) fn overwrite_star_data(&mut self, index: Option<usize>, star_data: StarData) {
        match index {
            Some(index) => self.distant_stars[index] = Star::from_data(star_data, Some(index)),
            None => self.central_body = star_data,
        }
        self.sort_stars_by_brightness();
    }

    fn sort_stars_by_brightness(&mut self) {
        self.distant_stars.sort_by(|a, b| {
            b.get_appearance()
                .get_illuminance()
                .partial_cmp(&a.get_appearance().get_illuminance())
                .unwrap()
        });
        for (i, star) in self.distant_stars.iter_mut().enumerate() {
            star.set_index(i);
        }
    }

    pub(crate) fn get_central_body_data(&self) -> &StarData {
        &self.central_body
    }

    pub(crate) fn get_central_body_appearance(
        &self,
        observer_pos: &CartesianCoordinates,
    ) -> StarAppearance {
        let mut body = self.central_body.clone();
        let relative_position = -observer_pos;
        let distance = relative_position.length();
        let pos = relative_position.to_ecliptic();
        body.set_distance(Some(distance));
        body.set_pos(pos);
        body.to_star_appearance()
    }

    pub(crate) fn get_planets_data(&self) -> Vec<&PlanetData> {
        let mut bodies = Vec::new();
        for planet in &self.planets {
            bodies.push(planet);
        }
        bodies
    }

    pub(crate) fn get_planet_data(&self, index: usize) -> Option<&PlanetData> {
        self.planets.get(index)
    }

    pub(crate) fn get_planets_at_time(&self, time: Time<f64>) -> Vec<Planet> {
        let mut bodies: Vec<Planet> = Vec::new();
        for (i, planet_data) in self.planets.iter().enumerate() {
            let previous = if i > 0 {
                Some(bodies[i - 1].get_derived_data())
            } else {
                None
            };
            let planet = Planet::new(
                planet_data.clone(),
                &self.central_body,
                previous,
                time,
                Some(i),
            );
            bodies.push(planet);
        }
        bodies
    }

    pub(crate) fn get_stars(&self) -> Vec<Star> {
        let mut bodies = Vec::new();
        bodies.push(Star::from_data(self.central_body.clone(), None));
        for star in &self.distant_stars {
            bodies.push(star.clone());
        }
        bodies
    }

    pub(crate) fn get_distant_star_appearances(&self) -> Vec<&StarAppearance> {
        let mut stars = Vec::new();
        for star in &self.distant_stars {
            stars.push(star.get_appearance());
        }
        stars
    }

    pub(crate) fn get_star_data(&self, index: Option<usize>) -> Option<&StarData> {
        match index {
            Some(index) => self.distant_stars.get(index).and_then(|s| s.get_data()),
            None => Some(&self.central_body),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::part_of_celestial_system::PartOfCelestialSystem;

    use super::*;
    use astro_utils::{
        real_data::{
            planets::*,
            stars::{all::get_many_stars, SUN},
        },
        units::luminous_intensity::absolute_magnitude_to_luminous_intensity,
    };
    use simple_si_units::base::Distance;

    #[test]
    fn planets_are_sorted_by_semimajor_axis() {
        let mut system = CelestialSystem::new(SystemType::Real, SUN.to_star_data());
        system.add_planet_data(VENUS.to_planet_data());
        system.add_planet_data(MERCURY.to_planet_data());
        system.add_planet_data(MARS.to_planet_data());
        system.add_planet_data(EARTH.to_planet_data());
        let planets = system.get_planets_data();
        assert_eq!(planets[0].get_name(), "Mercury");
        assert_eq!(planets[1].get_name(), "Venus");
        assert_eq!(planets[2].get_name(), "Earth");
        assert_eq!(planets[3].get_name(), "Mars");
    }

    #[test]
    fn edited_planets_are_sorted_by_semimajor_axis() {
        let mut system = CelestialSystem::new(SystemType::Real, SUN.to_star_data());
        system.add_planet_data(MERCURY.to_planet_data());
        system.add_planet_data(EARTH.to_planet_data());
        system.overwrite_planet_data(0, JUPITER.to_planet_data());
        let planets = system.get_planets_data();
        assert_eq!(planets[0].get_name(), "Earth");
        assert_eq!(planets[1].get_name(), "Jupiter");
    }

    #[test]
    fn central_body_has_distance_none() {
        for star in get_many_stars().iter() {
            let system = CelestialSystem::new(SystemType::Real, star.to_star_data());
            assert!(system.get_central_body_data().get_distance().is_none());
        }
    }

    #[test]
    fn stars_are_sorted_by_brightness() {
        let mut system = CelestialSystem::new(SystemType::Real, SUN.to_star_data());
        for star in get_many_stars().iter().rev() {
            system.add_star_from_data(star.to_star_data());
        }
        let stars = system.get_stars();
        for i in 1..stars.len() - 1 {
            assert!(
                stars[i].get_appearance().get_illuminance()
                    >= stars[i + 1].get_appearance().get_illuminance()
            );
        }
    }

    #[test]
    fn edited_stars_are_sorted_by_brightness() {
        let mut system = CelestialSystem::new(SystemType::Real, SUN.to_star_data());
        for star in get_many_stars().iter() {
            system.add_star_from_data(star.to_star_data());
        }
        let mut bright_star = SUN.to_star_data();
        bright_star.set_distance(Some(Distance::from_lyr(1.)));
        bright_star.set_luminous_intensity(Some(absolute_magnitude_to_luminous_intensity(-10.)));
        system.overwrite_star_data(Some(17), bright_star);
        let stars = system.get_stars();
        for i in 1..stars.len() - 1 {
            assert!(
                stars[i].get_appearance().get_illuminance()
                    >= stars[i + 1].get_appearance().get_illuminance()
            );
        }
    }

    #[test]
    fn star_index_is_correct_after_sorting() {
        let mut system = CelestialSystem::new(SystemType::Real, SUN.to_star_data());
        for star in get_many_stars().iter().rev() {
            system.add_star_from_data(star.to_star_data());
        }
        for (i, star) in system.get_stars().iter().enumerate() {
            if i == 0 {
                assert_eq!(star.get_index(), None);
            } else {
                assert_eq!(star.get_index(), Some(i - 1));
            }
        }
    }
}
