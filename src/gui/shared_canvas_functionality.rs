use crate::model::celestial_body::CelestialBody;
use iced::{
    widget::canvas::{self, Path},
    Color, Point, Rectangle,
};

pub(super) fn draw_background(bounds: iced::Rectangle, frame: &mut canvas::Frame) {
    let background = Path::rectangle(Point::ORIGIN, bounds.size());
    frame.fill(&background, Color::BLACK);
}

pub(super) fn draw_body_name(
    body: &CelestialBody,
    color: Color,
    body_center: Point,
    radius: f32,
    frame: &mut canvas::Frame,
) {
    let ordinate_offset = (0.5 as f32).sqrt() * radius;
    let mut name_widget = canvas::Text::default();
    name_widget.color = color;
    name_widget.content = body.get_name().to_string();
    name_widget.position = body_center + iced::Vector::new(ordinate_offset, ordinate_offset);
    frame.fill_text(name_widget);
}

pub(super) fn maximized_color(body: &CelestialBody) -> Color {
    let (r, g, b) = body.get_color().normalized_sRGB_tuple();
    let max_rgb = r.max(g).max(b);
    let r = r / max_rgb;
    let g = g / max_rgb;
    let b = b / max_rgb;
    Color::from_rgb(r, g, b)
}

/*
 * Iced's bound.contains is currently broken:
 * https://github.com/TheComamba/IcedPlayground/blob/main/canvas_coordinates/src/main.rs
 */
pub(super) fn contains_workaround(bounds: &Rectangle, point: Point) -> bool {
    return point.x >= 0. && point.x <= bounds.width && point.y >= 0. && point.y <= bounds.height;
}
