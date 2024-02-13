use super::{viewport::Viewport, widget::SurfaceViewState};
use crate::{
    gui::surface_view::star_appearance::StarCanvasAppearance,
    model::{celestial_system::CelestialSystem, constellation::*, star::Star},
};
use astro_utils::coordinates::cartesian::CartesianCoordinates;
use iced::{
    widget::canvas::{Frame, Path, Stroke},
    Rectangle, Vector,
};

impl SurfaceViewState {
    pub(super) fn draw_constellations(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        celestial_system: &CelestialSystem,
        viewport: &Viewport,
        observer_position: &CartesianCoordinates,
    ) {
        let stars = celestial_system.get_stars();

        for constellation_name in collect_constellation_names(&stars[..]) {
            let stars = collect_stars_in_constellation(&constellation_name, &stars[..]);
            self.draw_constellation(frame, bounds, stars, &viewport);
        }
    }

    fn draw_constellation(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        stars: Vec<&Star>,
        viewport: &Viewport,
    ) {
        const RADIUS: f32 = 200.;

        let canvas_appearances = stars
            .iter()
            .map(|s| StarCanvasAppearance::from_star_appearance(s.get_appearance(), viewport))
            .filter_map(|a| a);
        let mut center = Vector::ZERO;
        for appearance in canvas_appearances {
            center = center + appearance.center_offset;
            let position = frame.center() + appearance.center_offset;
            let circle = Path::circle(position, RADIUS);
            frame.stroke(&circle, Stroke::default());
        }
        todo!()
    }
}
