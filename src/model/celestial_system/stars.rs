use std::cmp::Ordering;

use astro_utils::{
    coordinates::cartesian::CartesianCoordinates,
    real_data::stars::{all::get_many_stars, SUN},
    stars::{
        appearance::StarAppearance,
        data::StarData,
        gaia::{
            gaia_source::{fetch_brightest_stars, star_is_already_known},
            gaia_universe_simulation::fetch_brightest_stars_simulated_data,
        },
        random::random_stars::{generate_random_star, generate_random_stars},
    },
};
use simple_si_units::base::Distance;

use crate::{
    error::ElenathError,
    model::star::{Star, StarDataType},
};

use super::CelestialSystem;

impl CelestialSystem {
    pub(crate) fn add_stars_from_data(&mut self, star_data: Vec<StarData>) {
        let index = self.distant_stars.len();
        for data in star_data {
            self.distant_stars
                .push(Star::from_data(data, Some(index), self.time_since_epoch));
        }
        self.process_stars();
    }

    pub(crate) fn add_star_appearances_without_duplicates(
        &mut self,
        star_appearances: Vec<StarAppearance>,
    ) {
        for star_appearance in star_appearances {
            let known_stars: Vec<StarAppearance> = self
                .get_distant_star_appearances()
                .into_iter()
                .cloned()
                .collect();
            if !star_is_already_known(&star_appearance, &known_stars[..]) {
                let index = self.distant_stars.len();
                self.distant_stars
                    .push(Star::from_appearance(star_appearance, Some(index)));
            }
        }
        self.process_stars();
    }

    pub(crate) fn overwrite_star_data(&mut self, index: Option<usize>, star_data: StarData) {
        match index {
            Some(index) => {
                self.distant_stars[index] =
                    Star::from_data(star_data, Some(index), self.time_since_epoch)
            }
            None => self.central_body = star_data,
        }
        self.process_stars();
    }

    fn process_stars(&mut self) {
        self.sort_stars_by_brightness();
        self.update_constellations();
    }

    fn sort_stars_by_brightness(&mut self) {
        fn illum(b: &Star) -> &simple_si_units::electromagnetic::Illuminance<f64> {
            b.get_appearance().get_illuminance()
        }

        self.distant_stars
            .sort_by(|a, b| illum(b).partial_cmp(illum(a)).unwrap_or(Ordering::Equal));
        for (i, star) in self.distant_stars.iter_mut().enumerate() {
            star.set_index(i);
        }
    }

    pub(crate) fn randomize_stars(
        &mut self,
        keep_central_body: bool,
        max_distance: Distance<f64>,
    ) -> Result<(), ElenathError> {
        if !keep_central_body {
            self.central_body = generate_random_star(None)?
        };
        let stars = generate_random_stars(max_distance)?;
        self.add_stars_from_data(stars);
        Ok(())
    }

    pub(crate) fn load_real_stars(&mut self, data_type: StarDataType) -> Result<(), ElenathError> {
        self.central_body = SUN.to_star_data();
        self.distant_stars.clear();
        match data_type {
            StarDataType::Hardcoded => {
                let stars = get_many_stars().iter().map(|s| s.to_star_data()).collect();
                self.add_stars_from_data(stars);
            }
            StarDataType::GaiaMeasurement => {
                let hardcoded_stars = get_many_stars().iter().map(|s| s.to_star_data()).collect();
                self.add_stars_from_data(hardcoded_stars);

                let gaia_stars = fetch_brightest_stars()?;
                self.add_star_appearances_without_duplicates(gaia_stars);
            }
            StarDataType::GaiaSimulation => {
                let stars = fetch_brightest_stars_simulated_data()?;
                self.add_stars_from_data(stars);
            }
        }
        Ok(())
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
        body.set_pos_at_epoch(relative_position);
        body.to_star_appearance(self.time_since_epoch)
    }

    pub(crate) fn get_stars(&self) -> Vec<Star> {
        let mut bodies = Vec::new();
        bodies.push(Star::from_data(
            self.central_body.clone(),
            None,
            self.time_since_epoch,
        ));
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
    use astro_utils::{
        real_data::stars::{all::get_many_stars, SUN},
        units::luminous_intensity::absolute_magnitude_to_luminous_intensity,
    };
    use simple_si_units::base::Distance;

    use crate::model::celestial_system::{part::PartOfCelestialSystem, CelestialSystem};

    #[test]
    fn central_body_has_distance_zero() {
        for star in get_many_stars().iter() {
            let system = CelestialSystem::new(star.to_star_data());
            assert!(system.get_central_body_data().get_distance_at_epoch() < Distance::from_m(1.));
        }
    }

    #[test]
    fn stars_are_sorted_by_brightness() {
        let mut system = CelestialSystem::new(SUN.to_star_data());
        let reverse_stars = get_many_stars()
            .iter()
            .rev()
            .map(|s| s.to_star_data())
            .collect();
        system.add_stars_from_data(reverse_stars);
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
        let mut system = CelestialSystem::new(SUN.to_star_data());
        let stars = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        system.add_stars_from_data(stars);
        let mut bright_star = SUN.to_star_data();
        bright_star.set_distance_at_epoch(Distance::from_lyr(1.));
        bright_star.set_luminous_intensity_at_epoch(absolute_magnitude_to_luminous_intensity(-10.));
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
        let mut system = CelestialSystem::new(SUN.to_star_data());
        let reversed_stars = get_many_stars()
            .iter()
            .rev()
            .map(|s| s.to_star_data())
            .collect();
        system.add_stars_from_data(reversed_stars);
        for (i, star) in system.get_stars().iter().enumerate() {
            if i == 0 {
                assert_eq!(star.get_index(), None);
            } else {
                assert_eq!(star.get_index(), Some(i - 1));
            }
        }
    }
}
