use astro_utils::stars::{star_appearance::StarAppearance, star_data::StarData};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Star {
    data: Option<StarData>,
    appearance: StarAppearance,
}

impl Star {
    pub(crate) fn new(data: StarData, appearance: StarAppearance) -> Self {
        Star {
            data: Some(data),
            appearance,
        }
    }

    pub(crate) fn from_data(data: StarData) -> Self {
        Star {
            data: Some(data),
            appearance: data.to_star_appearance(),
        }
    }

    pub(crate) fn from_appearance(appearance: StarAppearance) -> Self {
        Star {
            data: None,
            appearance,
        }
    }

    pub(crate) fn get_data(&self) -> Option<&StarData> {
        self.data.as_ref()
    }

    pub(crate) fn get_appearance(&self) -> &StarAppearance {
        &self.appearance
    }
}
