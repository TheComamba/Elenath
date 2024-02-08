use crate::model::{celestial_system::CelestialSystem, planet::Planet};

use super::viewport::Viewport;
use astro_utils::{
    coordinates::cartesian::CartesianCoordinates,
    coordinates::transformations::relative_direction::direction_relative_to_normal,
    stars::star_appearance::StarAppearance,
};
use iced::{Color, Vector};
use simple_si_units::electromagnetic::Illuminance;

pub(super) struct StarCanvasAppearance {
    pub(super) name: String,
    pub(super) center_offset: Vector,
    pub(super) radius: f32,
    pub(super) color: Color,
}

impl StarCanvasAppearance {
    pub(super) const MIN_RADIUS: f32 = 1.;
    const MAX_RADIUS: f32 = 1e5;
    const ILLUMINANCE_AT_MIN_RADIUS: Illuminance<f64> = Illuminance { lux: 1e-6 };

    pub(super) fn from_star_appearance(
        appearance: &StarAppearance,
        viewport: &Viewport,
    ) -> Option<StarCanvasAppearance> {
        let (color, radius) = Self::color_and_radius(appearance);
        Some(Self {
            name: appearance.get_name().to_string(),
            center_offset: offset(appearance, viewport)?,
            radius,
            color,
        })
    }

    pub(super) fn from_central_body(
        celestial_system: &CelestialSystem,
        viewport: &Viewport,
        observer_position: &CartesianCoordinates,
    ) -> Option<StarCanvasAppearance> {
        let central_body = celestial_system.get_central_body();
        let central_body_pos = -observer_position;
        let central_body_dir = central_body_pos.to_direction();
        let central_body_dir = match central_body_dir {
            Ok(dir) => dir,
            Err(_) => {
                return None;
            }
        };
        let mut central_body_appearance = central_body.get_appearance().clone();
        central_body_appearance.set_direction_in_ecliptic(central_body_dir);

        StarCanvasAppearance::from_star_appearance(&central_body_appearance, viewport)
    }

    pub(super) fn from_planet(
        celestial_system: &CelestialSystem,
        planet: &Planet,
        viewport: &Viewport,
        observer_position: &CartesianCoordinates,
    ) -> Option<StarCanvasAppearance> {
        let planet_appearance = planet.get_data().to_star_appearance(
            celestial_system.get_central_body_data(),
            planet.get_position(),
            observer_position,
        );
        let planet_appearance = match planet_appearance {
            Ok(appearance) => appearance,
            Err(_) => {
                return None;
            }
        };

        StarCanvasAppearance::from_star_appearance(&planet_appearance, viewport)
    }

    fn color_and_radius(body: &StarAppearance) -> (Color, f32) {
        let (r, g, b) = body.get_color().maximized_sRGB_tuple();
        let illuminance = body.get_illuminance();
        if illuminance < &Self::ILLUMINANCE_AT_MIN_RADIUS {
            let radius = Self::MIN_RADIUS;
            let alpha = illuminance / &Self::ILLUMINANCE_AT_MIN_RADIUS;
            let color = Color::from_rgba(r as f32, g as f32, b as f32, alpha as f32);
            (color, radius)
        } else {
            let radius =
                (illuminance / &Self::ILLUMINANCE_AT_MIN_RADIUS).sqrt() as f32 * Self::MIN_RADIUS;
            let color = Color::from_rgb(r as f32, g as f32, b as f32);
            if radius > Self::MAX_RADIUS {
                (color, Self::MAX_RADIUS)
            } else {
                (color, radius)
            }
        }
    }
}

fn offset(appearance: &StarAppearance, viewport: &Viewport) -> Option<Vector> {
    let direction = direction_relative_to_normal(
        appearance.get_direction_in_ecliptic(),
        &viewport.center_direction,
        &viewport.top_direction,
    );
    if direction.z() > 0.0 {
        let x = direction.y() as f32 * viewport.px_per_unit_height; // rotation_reference corresponds to the x axis while iced y corresponds to top.
        let y = -direction.x() as f32 * viewport.px_per_unit_height; // y axis is inverted
        Some(iced::Vector::new(x as f32, y as f32))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        gui::surface_view::{star_canvas_appearance::StarCanvasAppearance, viewport::Viewport},
        model::celestial_system::{CelestialSystem, SystemType},
    };
    use astro_utils::{
        color::sRGBColor,
        coordinates::direction::Direction,
        planets::{orbit_parameters::OrbitParameters, planet_data::PlanetData},
        real_data::stars::SUN_DATA,
        stars::star_appearance::StarAppearance,
        units::{
            angle::ANGLE_ZERO, distance::EARTH_RADIUS,
            illuminance::apparent_magnitude_to_illuminance, mass::EARTH_MASS, time::TIME_ZERO,
        },
    };
    use iced::Vector;
    use simple_si_units::{base::Distance, electromagnetic::Illuminance, geometry::Angle};

    const SOME_ILLUMINANCE: Illuminance<f64> = Illuminance { lux: 100. };
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
        let ordinates = vec![0., 1., 1., 12.];
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
                                let left = top.rotated(Angle::from_degrees(-90.), &center);
                                let bottom = left.rotated(Angle::from_degrees(-90.), &center);
                                let right = bottom.rotated(Angle::from_degrees(-90.), &center);

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
                                if half_opening_angle.to_degrees().abs() > 89. {
                                    continue;
                                }
                                let expected_offset = half_opening_angle.rad.sin() as f32
                                    * viewport.px_per_unit_height;

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
    fn view_direction_z() {
        let viewport = Viewport {
            center_direction: Direction::Z,
            top_direction: Direction::Y,
            px_per_unit_height: SOME_FLOAT,
        };
        for x in [-0.1, 0.1] {
            for y in [-0.1, 0.1] {
                let star_direction = Direction::new(x, y, 1.).unwrap();
                println!("star direction: {}", star_direction);
                let star = StarAppearance::new(
                    "".to_string(),
                    SOME_ILLUMINANCE,
                    SOME_COLOR,
                    star_direction,
                );
                let appearance = StarCanvasAppearance::from_star_appearance(&star, &viewport);
                let center_offset = appearance.unwrap().center_offset;
                println!("center offset: {:?}", center_offset);
                if x > 0. {
                    assert!(center_offset.x < 0.);
                } else {
                    assert!(center_offset.x > 0.);
                }
                if y > 0. {
                    assert!(center_offset.y < 0.);
                } else {
                    assert!(center_offset.y > 0.);
                }
            }
        }
    }

    #[test]
    fn view_direction_x() {
        let viewport = Viewport {
            center_direction: Direction::X,
            top_direction: Direction::Z,
            px_per_unit_height: SOME_FLOAT,
        };
        for y in [-0.1, 0.1] {
            for z in [-0.1, 0.1] {
                let star_direction = Direction::new(1., y, z).unwrap();
                println!("star direction: {}", star_direction);
                let star = StarAppearance::new(
                    "".to_string(),
                    SOME_ILLUMINANCE,
                    SOME_COLOR,
                    star_direction,
                );
                let appearance = StarCanvasAppearance::from_star_appearance(&star, &viewport);
                let center_offset = appearance.unwrap().center_offset;
                println!("center offset: {:?}", center_offset);
                if y > 0. {
                    assert!(center_offset.x < 0.);
                } else {
                    assert!(center_offset.x > 0.);
                }
                if z > 0. {
                    assert!(center_offset.y < 0.);
                } else {
                    assert!(center_offset.y > 0.);
                }
            }
        }
    }

    #[test]
    fn apparent_magnitude_6p5_star_is_barely_visible() {
        let star_appearance = StarAppearance::new(
            String::new(),
            apparent_magnitude_to_illuminance(6.5),
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
        println!("radius: {}", canvas_appearance.radius);
        assert!(canvas_appearance.radius > 0.);
        assert!(canvas_appearance.color.a > 0.);
        assert!(canvas_appearance.color.a < 0.1);
    }

    #[test]
    fn apparent_magnitude_0_star_is_bright() {
        let star_appearance = StarAppearance::new(
            String::new(),
            apparent_magnitude_to_illuminance(0.),
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
        println!("radius: {}", canvas_appearance.radius);
        assert!(canvas_appearance.radius > 1.);
        assert!(canvas_appearance.radius < 10.);
    }

    #[test]
    fn venus_is_not_too_big() {
        let star_appearance = StarAppearance::new(
            String::new(),
            apparent_magnitude_to_illuminance(-4.92),
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
        println!("radius: {}", canvas_appearance.radius);
        assert!(canvas_appearance.radius > 1.);
        assert!(canvas_appearance.radius < 16.);
    }

    #[test]
    fn the_sun_is_very_bright() {
        let star_appearance = StarAppearance::new(
            String::new(),
            apparent_magnitude_to_illuminance(-26.72),
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
        println!("radius: {}", canvas_appearance.radius);
        assert!(canvas_appearance.radius > 500.);
    }

    #[test]
    fn aligned_planet_sun_and_observer() {
        const CENTER: Vector = Vector { x: 0., y: 0. };

        let mut celestial_system =
            CelestialSystem::new(SystemType::Generated, SUN_DATA.to_star_data());
        let orbit = OrbitParameters::new(
            Distance::from_au(1.),
            0.,
            ANGLE_ZERO,
            ANGLE_ZERO,
            ANGLE_ZERO,
        );
        let planet_data = PlanetData::new(
            "Inner".to_string(),
            EARTH_MASS,
            EARTH_RADIUS,
            1.,
            sRGBColor::from_sRGB(1., 1., 1.),
            TIME_ZERO,
            orbit,
            Direction::Z,
        );
        celestial_system.add_planet_data(planet_data);
        let planets = celestial_system.get_planets_at_time(TIME_ZERO);
        let planet = planets.first().unwrap();
        let planet_position = planet.get_position();

        let away_from_sun = planet_position.to_direction().unwrap();
        let to_sun = -&away_from_sun;
        let viewport_away_from_sun = Viewport {
            center_direction: away_from_sun,
            top_direction: Direction::Z,
            px_per_unit_height: SOME_FLOAT,
        };
        let viewport_to_sun = Viewport {
            center_direction: to_sun,
            top_direction: Direction::Z,
            px_per_unit_height: SOME_FLOAT,
        };

        let inner_observer = planet_position * 0.5;

        let sun_appearance = StarCanvasAppearance::from_central_body(
            &celestial_system,
            &viewport_to_sun,
            &inner_observer,
        );
        assert!(sun_appearance.is_some());
        let sun_appearance = sun_appearance.unwrap();
        assert!(vecs_equal(sun_appearance.center_offset, CENTER));

        let planet_appearance = StarCanvasAppearance::from_planet(
            &celestial_system,
            &planet,
            &viewport_away_from_sun,
            &inner_observer,
        );
        assert!(planet_appearance.is_some());
        let planet_appearance = planet_appearance.unwrap();
        assert!(vecs_equal(planet_appearance.center_offset, CENTER));

        let outer_observer = planet_position * 1.5;
        let sun_appearance = StarCanvasAppearance::from_central_body(
            &celestial_system,
            &viewport_to_sun,
            &outer_observer,
        );
        assert!(sun_appearance.is_some());
        let sun_appearance = sun_appearance.unwrap();
        assert!(vecs_equal(sun_appearance.center_offset, CENTER));

        let planet_appearance = StarCanvasAppearance::from_planet(
            &celestial_system,
            &planet,
            &viewport_to_sun,
            &outer_observer,
        );
        assert!(planet_appearance.is_some());
        let planet_appearance = planet_appearance.unwrap();
        assert!(vecs_equal(planet_appearance.center_offset, CENTER));
    }
}
