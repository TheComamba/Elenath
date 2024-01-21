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
        color::sRGBColor,
        coordinates::direction::Direction,
        stars::star_appearance::StarAppearance,
        units::{angle::Angle, illuminance::Illuminance},
    };
    use iced::Point;

    const SOME_ILLUMINANCE: Illuminance = Illuminance::from_lux(100.);
    const SOME_COLOR: sRGBColor = sRGBColor::from_sRGB(0., 1., 0.);
    const SOME_WIDTH: f32 = 100.;

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
                        width: SOME_WIDTH,
                    };
                    let star_appearance = StarAppearance::new(
                        String::new(),
                        SOME_ILLUMINANCE,
                        SOME_COLOR,
                        center_direction,
                    );
                    let canvas_appearance =
                        StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport);
                    assert!(points_equal(
                        canvas_appearance.pos,
                        Point {
                            x: SOME_WIDTH / 2.,
                            y: SOME_WIDTH / 2.
                        }
                    ));
                }
            }
        }
    }

    #[test]
    fn stars_at_boundaries() {
        let ordinates = vec![-1., 0., 1., 12.];
        for x1 in ordinates.clone().iter() {
            for y1 in ordinates.clone().iter() {
                for z1 in ordinates.clone().iter() {
                    for x2 in ordinates.clone().iter() {
                        for y2 in ordinates.clone().iter() {
                            for z2 in ordinates.clone().iter() {
                                let center_direction = Direction::new(*x1, *y1, *z1);
                                let right_direction = Direction::new(*x2, *y2, *z2);
                                if center_direction.is_err() || right_direction.is_err() {
                                    continue;
                                }
                                let center = center_direction.unwrap();
                                let right = right_direction.unwrap();
                                if center.eq_within(&right, 1e-5)
                                    || center.eq_within(&(-&right), 1e-5)
                                {
                                    continue;
                                }
                                let bottom = right.rotated(Angle::from_degrees(90.), &center);
                                let left = bottom.rotated(Angle::from_degrees(90.), &center);
                                let top = left.rotated(Angle::from_degrees(90.), &center);

                                let viewport = Viewport {
                                    center_direction: center.clone(),
                                    right_direction: right.clone(),
                                    width: SOME_WIDTH,
                                };

                                let right = StarAppearance::new(
                                    String::new(),
                                    SOME_ILLUMINANCE,
                                    SOME_COLOR,
                                    right,
                                );
                                let bottom = StarAppearance::new(
                                    String::new(),
                                    SOME_ILLUMINANCE,
                                    SOME_COLOR,
                                    bottom,
                                );
                                let left = StarAppearance::new(
                                    String::new(),
                                    SOME_ILLUMINANCE,
                                    SOME_COLOR,
                                    left,
                                );
                                let top = StarAppearance::new(
                                    String::new(),
                                    SOME_ILLUMINANCE,
                                    SOME_COLOR,
                                    top,
                                );

                                let right =
                                    StarCanvasAppearance::from_star_appearance(&right, &viewport);
                                let bottom =
                                    StarCanvasAppearance::from_star_appearance(&bottom, &viewport);
                                let left =
                                    StarCanvasAppearance::from_star_appearance(&left, &viewport);
                                let top =
                                    StarCanvasAppearance::from_star_appearance(&top, &viewport);

                                assert!(points_equal(
                                    right.pos,
                                    Point {
                                        x: SOME_WIDTH,
                                        y: SOME_WIDTH / 2.
                                    }
                                ));
                                assert!(points_equal(
                                    bottom.pos,
                                    Point {
                                        x: SOME_WIDTH / 2.,
                                        y: SOME_WIDTH
                                    }
                                ));
                                assert!(points_equal(
                                    left.pos,
                                    Point {
                                        x: 0.,
                                        y: SOME_WIDTH / 2.
                                    }
                                ));
                                assert!(points_equal(
                                    top.pos,
                                    Point {
                                        x: SOME_WIDTH / 2.,
                                        y: 0.
                                    }
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
}
