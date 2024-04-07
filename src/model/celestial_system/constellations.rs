use astro_utils::stars::{
    constellation::{collect_constellations, Constellation},
    data::StarData,
};

use super::CelestialSystem;

impl CelestialSystem {
    pub(super) fn update_constellations(&mut self) {
        let stars: Vec<StarData> = self
            .get_stars()
            .iter()
            .filter_map(|s| s.get_data())
            .cloned()
            .collect();
        self.constellations = collect_constellations(&stars[..], self.time_since_epoch);
    }

    pub(crate) fn get_constellations(&self) -> &Vec<Constellation> {
        &self.constellations
    }
}
