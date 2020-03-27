use crate::js_bridge;

pub fn draw() {
    init("canvas1", 1000, 1000);
    clear("#fff");
    let c = Circle::new(300, 250, 60, Color::new(0xff, 0x00, 0xff, 0.3));
    draw_circle(c);
}

pub fn init(canvas_id: &str, height: i32, width: i32) {
    js_bridge::init(canvas_id, height, width);
}

pub fn clear(color_code: &str) {
    js_bridge::clear(color_code);
}

pub fn draw_circle(circle: Circle) {
    js_bridge::drawCircle(circle.center_x, circle.center_y, circle.radius, circle.color.red, circle.color.green, circle.color.blue, circle.color.alpha);
}

pub struct Circle {
    center_x: i32, 
    center_y: i32, 
    radius: i32, 
    color: Color,
}

impl Circle {
    pub fn new(center_x: i32, center_y: i32, radius: i32, color: Color) -> Circle {
        Circle { center_x, center_y, radius, color }
    }
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
        Color { red, green, blue, alpha }
    }
}