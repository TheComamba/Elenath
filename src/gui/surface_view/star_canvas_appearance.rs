use astro_utils::{
    planets::surface_normal::direction_relative_to_surface_normal,
    stars::star_appearance::StarAppearance, units::illuminance::Illuminance,
};
use iced::{Color, Vector};

use super::viewport::Viewport;

pub(super) struct StarCanvasAppearance<'a> {
    pub(super) name: &'a str,
    pub(super) center_offset: Vector,
    pub(super) radius: f32,
    pub(super) color: Color,
}

impl<'a> StarCanvasAppearance<'a> {
    // dimmest apparent magnitude: 6.5
    // as lux: 10.powf((-14.18 - 6.5) / 2.5);
    // A magnitude 6.5 star should appear with size between 0.1 and 1
    // So the factor is (0.1 to 1.) / sqrt(10.powf((-14.18 - 6.5) / 2.5))
    // which equals 1367.7 to 13677
    const BRIGHTNESS_FACTOR: f32 = 5000.;

    pub(super) fn from_star_appearance(
        appearance: &'a StarAppearance,
        viewport: &Viewport,
    ) -> Option<StarCanvasAppearance<'a>> {
        Some(Self {
            name: appearance.get_name(),
            center_offset: offset(appearance, viewport)?,
            radius: brightness_radius(&appearance.get_illuminance()),
            color: canvas_color(appearance),
        })
    }
}

fn offset(appearance: &StarAppearance, viewport: &Viewport) -> Option<Vector> {
    let direction = direction_relative_to_surface_normal(
        &appearance.get_direction_in_ecliptic(),
        &viewport.center_direction,
        &viewport.top_direction,
    );
    if direction.z() > 0.0 {
        let x = -direction.y() * viewport.px_per_unit_height; // rotation_reference corresponds to the x axis while iced y corresponds to top.
        let y = -direction.x() * viewport.px_per_unit_height; // y axis is inverted
        Some(iced::Vector::new(x as f32, y as f32))
    } else {
        None
    }
}

pub(super) fn brightness_radius(brightness: &Illuminance) -> f32 {
    let lux = brightness.as_lux();
    let size = lux.sqrt() * StarCanvasAppearance::BRIGHTNESS_FACTOR;
    if size > 1e5 {
        1e5
    } else {
        size
    }
}

pub(super) fn canvas_color(body: &StarAppearance) -> Color {
    let (r, g, b) = body.get_color().maximized_sRGB_tuple();
    let color = Color::from_rgb(r, g, b);
    color
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
    use iced::Vector;

    const SOME_ILLUMINANCE: Illuminance = Illuminance::from_lux(100.);
    const SOME_COLOR: sRGBColor = sRGBColor::from_sRGB(0., 1., 0.);
    const SOME_FLOAT: f32 = 1.;

    fn vecs_equal(p1: Vector, p2: Vector) -> bool {
        (p1.x - p2.x).abs() < 1e-4 && (p1.y - p2.y).abs() < 1e-4
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
                        px_per_unit_height: SOME_FLOAT,
                    };
                    let star_appearance = StarAppearance::new(
                        String::new(),
                        SOME_ILLUMINANCE,
                        SOME_COLOR,
                        center_direction,
                    );
                    let canvas_appearance =
                        StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport)
                            .unwrap();
                    assert!(vecs_equal(
                        canvas_appearance.center_offset,
                        Vector { x: 0., y: 0. }
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

                                println!(
                                    "center: {}, top: {}, left: {}, bottom: {}, right: {}",
                                    center, top, left, bottom, right
                                );

                                let viewport = Viewport {
                                    center_direction: center.clone(),
                                    top_direction: top.clone(),
                                    px_per_unit_height: SOME_FLOAT,
                                };
                                let half_opening_angle = center.angle_to(&top);
                                if half_opening_angle.as_degrees().abs() > 89. {
                                    continue;
                                }
                                let expected_offset =
                                    half_opening_angle.sin() * viewport.px_per_unit_height;

                                println!("half opening angle: {}", half_opening_angle);
                                println!("expected offset: {}", expected_offset);

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
                                    StarCanvasAppearance::from_star_appearance(&top, &viewport)
                                        .unwrap();
                                let left =
                                    StarCanvasAppearance::from_star_appearance(&left, &viewport)
                                        .unwrap();
                                let bottom =
                                    StarCanvasAppearance::from_star_appearance(&bottom, &viewport)
                                        .unwrap();
                                let right =
                                    StarCanvasAppearance::from_star_appearance(&right, &viewport)
                                        .unwrap();

                                println!(
                                    "top: {:?}, left: {:?}, bottom: {:?}, right: {:?}",
                                    top.center_offset,
                                    left.center_offset,
                                    bottom.center_offset,
                                    right.center_offset
                                );

                                assert!(vecs_equal(
                                    top.center_offset,
                                    Vector {
                                        x: 0.,
                                        y: -expected_offset
                                    }
                                ));
                                assert!(vecs_equal(
                                    left.center_offset,
                                    Vector {
                                        x: -expected_offset,
                                        y: 0.
                                    }
                                ));
                                assert!(vecs_equal(
                                    bottom.center_offset,
                                    Vector {
                                        x: 0.,
                                        y: expected_offset
                                    }
                                ));
                                assert!(vecs_equal(
                                    right.center_offset,
                                    Vector {
                                        x: expected_offset,
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
            px_per_unit_height: SOME_FLOAT,
        };
        let canvas_appearance =
            StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport).unwrap();
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
            px_per_unit_height: SOME_FLOAT,
        };
        let canvas_appearance =
            StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport).unwrap();
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
            px_per_unit_height: SOME_FLOAT,
        };
        let canvas_appearance =
            StarCanvasAppearance::from_star_appearance(&star_appearance, &viewport).unwrap();
        assert!(canvas_appearance.radius > 500.);
    }
}
