use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        canvas::{self, Path},
        text::Shaping,
    },
    Color, Pixels, Point, Rectangle, Vector,
};

pub(super) fn draw_background(bounds: Rectangle, frame: &mut canvas::Frame) {
    let background = Path::rectangle(Point::ORIGIN, bounds.size());
    frame.fill(&background, Color::BLACK);
}

pub(super) fn draw_name(name: &str, color: Color, body_center: Point, frame: &mut canvas::Frame) {
    const ORDINATE_OFFSET: f32 = 10.;
    if name.is_empty() || name.starts_with("Gaia") {
        return;
    }
    let name_widget = canvas::Text {
        color,
        content: name.to_string(),
        position: body_center + Vector::new(ORDINATE_OFFSET, ORDINATE_OFFSET),
        shaping: Shaping::Advanced,
        ..Default::default()
    };
    frame.fill_text(name_widget);
}

/*
 * Iced's bound.contains is currently broken:
 * https://github.com/TheComamba/IcedPlayground/blob/main/canvas_coordinates/src/main.rs
 */
pub(super) fn contains_workaround(bounds: &Rectangle, point: Point) -> bool {
    point.x >= 0. && point.x <= bounds.width && point.y >= 0. && point.y <= bounds.height
}

pub(crate) fn display_info_text(frame: &mut canvas::Frame, text: &str) {
    let name_widget = canvas::Text {
        size: Pixels(30.0),
        color: Color::WHITE,
        content: text.to_string(),
        position: frame.center(),
        vertical_alignment: Vertical::Center,
        horizontal_alignment: Horizontal::Center,
        ..Default::default()
    };
    frame.fill_text(name_widget)
}
