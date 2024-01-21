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
    const SOME_HEIGHT: f32 = 100.;

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
                    let top_direction = center_direction.some_orthogonal_vector();
                    let viewport = Viewport {
                        center_direction: center_direction.clone(),
                        top_direction,
                        height: SOME_HEIGHT,
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
                            x: SOME_HEIGHT / 2.,
                            y: SOME_HEIGHT / 2.
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
                                let top_direction = Direction::new(*x2, *y2, *z2);
                                if center_direction.is_err() || top_direction.is_err() {
                                    continue;
                                }
                                let center = center_direction.unwrap();
                                let top = top_direction.unwrap();
                                if center.eq_within(&top, 1e-5) || center.eq_within(&(-&top), 1e-5)
                                {
                                    continue;
                                }
                                let left = top.rotated(Angle::from_degrees(90.), &center);
                                let bottom = left.rotated(Angle::from_degrees(90.), &center);
                                let right = bottom.rotated(Angle::from_degrees(90.), &center);

                                let viewport = Viewport {
                                    center_direction: center.clone(),
                                    top_direction: top.clone(),
                                    height: SOME_HEIGHT,
                                };

                                let top = StarAppearance::new(
                                    String::new(),
                                    SOME_ILLUMINANCE,
                                    SOME_COLOR,
                                    top,
                                );
                                let left = StarAppearance::new(
                                    String::new(),
                                    SOME_ILLUMINANCE,
                                    SOME_COLOR,
                                    left,
                                );
                                let bottom = StarAppearance::new(
                                    String::new(),
                                    SOME_ILLUMINANCE,
                                    SOME_COLOR,
                                    bottom,
                                );
                                let right = StarAppearance::new(
                                    String::new(),
                                    SOME_ILLUMINANCE,
                                    SOME_COLOR,
                                    right,
                                );

                                let top =
                                    StarCanvasAppearance::from_star_appearance(&top, &viewport);
                                let left =
                                    StarCanvasAppearance::from_star_appearance(&left, &viewport);
                                let bottom =
                                    StarCanvasAppearance::from_star_appearance(&bottom, &viewport);
                                let right =
                                    StarCanvasAppearance::from_star_appearance(&right, &viewport);

                                assert!(points_equal(
                                    top.pos,
                                    Point {
                                        x: SOME_HEIGHT / 2.,
                                        y: 0.
                                    }
                                ));
                                assert!(points_equal(
                                    left.pos,
                                    Point {
                                        x: 0.,
                                        y: SOME_HEIGHT / 2.
                                    }
                                ));
                                assert!(points_equal(
                                    bottom.pos,
                                    Point {
                                        x: SOME_HEIGHT / 2.,
                                        y: SOME_HEIGHT
                                    }
                                ));
                                assert!(points_equal(
                                    right.pos,
                                    Point {
                                        x: SOME_HEIGHT,
                                        y: SOME_HEIGHT / 2.
                                    }
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn apparent_magnitude_6p5_star_is_barely_visible() {
        let star_appearance = StarAppearance::new(
            String::new(),
            Illuminance::from_apparent_magnitude(6.5),
            SOME_COLOR,
            Direction::X,
        );
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Y,
            height: SOME_HEIGHT,
        };
        let canvas_appearance =
            StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport);
        assert!(canvas_appearance.radius > 0.);
        assert!(canvas_appearance.radius < 1.);
    }

    #[test]
    fn apparent_magnitude_0_star_is_bright() {
        let star_appearance = StarAppearance::new(
            String::new(),
            Illuminance::from_apparent_magnitude(0.),
            SOME_COLOR,
            Direction::X,
        );
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Y,
            height: SOME_HEIGHT,
        };
        let canvas_appearance =
            StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport);
        assert!(canvas_appearance.radius > 1.);
        assert!(canvas_appearance.radius < 10.);
    }

    #[test]
    fn the_sun_is_very_bright() {
        let star_appearance = StarAppearance::new(
            String::new(),
            Illuminance::from_apparent_magnitude(-26.72),
            SOME_COLOR,
            Direction::X,
        );
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Y,
            height: SOME_HEIGHT,
        };
        let canvas_appearance =
            StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport);
        assert!(canvas_appearance.radius > 500.);
    }
}
