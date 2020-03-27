use crate::js_bridge;
use js_sys::Math;

pub fn draw() {
    init("canvas1", 1000, 1000);
    clear("#fff");

    for _ in 0..500 {
        let c = get_random_circle();
        draw_circle(c);    
    }
    
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

fn random_int(min: i32, max: i32) -> i32 {
    let range = max - min;
    ((Math::random() * (range as f64)) as i32) + min
}

fn random_int_around_point(center: i32, radius: i32) -> i32 {
    random_int(center - radius, center + radius)
}

fn get_random_circle() -> Circle {
    let c = Circle::new(
        random_int_around_point(500, 300), 
        random_int_around_point(500, 250), 
        random_int(0, 60), 
        Color::new(0xff, 0x00, 0xff, 0.3));
    c
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