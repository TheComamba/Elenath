#[derive(Debug, Clone, Copy)]
pub(crate) enum BodyType {
    Planet,
    Star,
}

pub(crate) trait PartOfCelestialSystem {
    fn get_index(&self) -> Option<usize>;

    fn get_body_type(&self) -> BodyType;
}
