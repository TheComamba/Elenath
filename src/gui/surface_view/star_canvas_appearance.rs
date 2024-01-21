use astro_utils::stars::star_appearance::StarAppearance;
use iced::{Color, Point};

use super::viewport::Viewport;

pub(super) struct StarCanvasAppearance<'a> {
    pub(super) name: &'a str,
    pub(super) pos: Point,
    pub(super) radius: f32,
    pub(super) color: Color,
}

impl<'a> StarCanvasAppearance<'a> {
    pub(super) fn from_star_appearance(
        appearance: &'a StarAppearance,
        viewport: &Viewport,
    ) -> StarCanvasAppearance<'a> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::gui::surface_view::{
        star_canvas_appearance::StarCanvasAppearance, viewport::Viewport,
    };
    use astro_utils::{
        color::sRGBColor, coordinates::direction::Direction,
        stars::star_appearance::StarAppearance, units::illuminance::Illuminance,
    };
    use iced::Point;

    const SOME_ILLUMINANCE: Illuminance = Illuminance::from_lux(100.);
    const SOME_COLOR: sRGBColor = sRGBColor::from_sRGB(0., 1., 0.);

    fn points_equal(p1: Point, p2: Point) -> bool {
        (p1.x - p2.x).abs() < 1e-5 && (p1.y - p2.y).abs() < 1e-5
    }

    #[test]
    fn star_at_center() {
        let ordinates = vec![-1., 0., 1., 12.];
        for x in ordinates.clone().iter() {
            for y in ordinates.clone().iter() {
                for z in ordinates.clone().iter() {
                    let center_direction = Direction::new(*x, *y, *z);
                    if center_direction.is_err() {
                        continue;
                    }
                    let center_direction = center_direction.unwrap();
                    let right_direction = center_direction.some_orthogonal_vector();
                    let viewport = Viewport {
                        center_direction: center_direction.clone(),
                        right_direction,
                    };
                    let star_appearance = StarAppearance::new(
                        String::new(),
                        SOME_ILLUMINANCE,
                        SOME_COLOR,
                        center_direction,
                    );
                    let canvas_appearance =
                        StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport);
                    assert!(points_equal(canvas_appearance.pos, Point { x: 0., y: 0. }));
                }
            }
        }
    }
}
