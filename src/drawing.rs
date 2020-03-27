use crate::js_bridge;

pub fn draw() {
    init("canvas1", 1000, 1000);
    clear("#fff");
    let c = Circle { center_x: 300, center_y: 250, radius: 60, color: "#ff00ff" };
    draw_circle(c);
}

pub fn init(canvas_id: &str, height: i32, width: i32) {
    js_bridge::init(canvas_id, height, width);
}

pub fn clear(color_code: &str) {
    js_bridge::clear(color_code);
}

pub fn draw_circle(circle: Circle) {
    js_bridge::drawCircle(circle.center_x, circle.center_y, circle.radius, circle.color);
}

pub struct Circle<'a> {
    center_x: i32, 
    center_y: i32, 
    radius: i32, 
    color: &'a str,
}