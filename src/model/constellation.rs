use astro_utils::stars::star_data::StarData;

pub(crate) fn collect_constellation_names(all_stars: &[StarData]) -> Vec<String> {
    let mut constellation_names: Vec<String> = Vec::new();
    for star in all_stars {
        if let Some(constellation) = star.get_constellation() {
            if !constellation_names.contains(constellation) {
                constellation_names.push(constellation.clone());
            }
        }
    }
    constellation_names
}

pub(crate) fn collect_stars_in_constellation<'a>(
    constellation_name: &str,
    all_stars: &'a [StarData],
) -> Vec<&'a StarData> {
    let mut stars_in_constellation: Vec<&StarData> = Vec::new();
    for star in all_stars {
        if let Some(constellation) = star.get_constellation() {
            if constellation == constellation_name {
                stars_in_constellation.push(star);
            }
        }
    }
    stars_in_constellation
}
