use astro_utils::stars::{star_appearance::StarAppearance, star_data::StarData};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Star {
    data: Option<StarData>,
    appearance: StarAppearance,
    index: usize,
}

impl Star {
    pub(crate) fn from_data(data: StarData, index: usize) -> Self {
        let appearance = data.to_star_appearance();
        Star {
            data: Some(data),
            appearance,
            index,
        }
    }

    pub(crate) fn from_appearance(appearance: StarAppearance, index: usize) -> Self {
        Star {
            data: None,
            appearance,
            index,
        }
    }

    pub(crate) fn get_data(&self) -> Option<&StarData> {
        self.data.as_ref()
    }

    pub(crate) fn get_appearance(&self) -> &StarAppearance {
        &self.appearance
    }
}
