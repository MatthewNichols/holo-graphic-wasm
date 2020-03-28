use crate::js_bridge;
use js_sys::Math;

pub fn draw() {
    let mut pallette = Pallette::new();

    pallette.init("canvas1", 1000, 1000);
    pallette.clear("#fff");

    pallette.draw_circles_within_circle(Coordinates(500, 500), 200, 200);
    
}

struct Pallette {
    colors: Vec<Color>,
    sizes: Vec<CircleSize>,
    circles: Vec<Circle>,
}

impl Pallette {
    pub fn new() -> Pallette {
        Pallette {
            colors: vec![
                Color::new(20, 151, 162, 1.0),
                Color::new(74, 59, 142, 1.0),
                Color::new(21, 119, 78, 1.0),
            ],
            sizes: vec![
                CircleSize { radius: 17, weight: 10 },
                CircleSize { radius: 12, weight: 10 },
                CircleSize { radius: 8, weight: 12 },
                CircleSize { radius: 5, weight: 12 },
            ],
            circles: vec![]
        }
    }

    pub fn init(&self, canvas_id: &str, height: i32, width: i32) {
        js_bridge::init(canvas_id, height, width);
    }
    
    pub fn clear(&self, color_code: &str) {
        js_bridge::clear(color_code);
    }
    
    pub fn draw_circles_within_circle(&mut self, center: Coordinates, max_distance_from_center: i32, number_of_attempts: i32) {
        for _ in 0..number_of_attempts {
            let c = self.get_random_circle(center, max_distance_from_center);
            self.draw_circle(c);
            self.circles.push(c);    
        }
    }

    pub fn draw_circle(&self, circle: Circle) {
        js_bridge::drawCircle(circle.center_x, circle.center_y, circle.radius, circle.color.red, circle.color.green, circle.color.blue, circle.color.alpha);
    }
    
    fn get_random_circle(&self, center: Coordinates, max_distance_from_center: i32) -> Circle {
        let angle = random_angle();
        let distance_from_center = Math::random() * max_distance_from_center as f64;

        let x = (distance_from_center * angle.cos()) as i32 + center.0;
        let y = (distance_from_center * angle.sin()) as i32 + center.1;
        let c = Circle::new(x, y, self.pick_random_size(), self.pick_random_color());
        c
    }

    fn pick_random_color(&self) -> Color {
        self.colors[random_index(2)]
    }
    
    fn pick_random_size(&self) -> i32 {
        self.sizes[random_index(self.sizes.len() as i32)].radius
    }
}

fn random_angle() -> f64 {
    Math::random() * 2.0 * std::f64::consts::PI 
}

fn random_index(max: i32) -> usize {
    (Math::random() * (max as f64)) as usize
}

#[derive(Clone, Copy)]
pub struct Coordinates(i32, i32);

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct CircleSize {
    radius: i32,
    weight: i32
}