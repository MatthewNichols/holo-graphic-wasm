use crate::js_bridge;
use js_sys::Math;
use std::cmp::Ordering;

pub fn draw() {
    let mut pallette = Pallette::new();

    pallette.init("canvas1", 1000, 1000);
    pallette.clear("#fff");

    pallette.draw_circles_within_circle(Coordinates(500, 500), 200, 500);
    
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
        if circle.radius > 0 {
            js_bridge::drawCircle(circle.center_x, circle.center_y, circle.radius, circle.color.red, circle.color.green, circle.color.blue, circle.color.alpha);
        }
    }
    
    fn get_random_circle(&self, center: Coordinates, max_distance_from_center: i32) -> Circle {
        let angle = random_angle();
        let distance_from_center = Math::random() * max_distance_from_center as f64;

        let x = (distance_from_center * angle.cos()) as i32 + center.0;
        let y = (distance_from_center * angle.sin()) as i32 + center.1;
        let closest = self.closest_circle(Coordinates(x, y));
        let c = Circle::new(x, y, self.pick_random_size(Coordinates(x, y), closest), self.pick_random_color());
        c
    }

    fn closest_circle(&self, center: Coordinates) -> Option<Circle> {
        if self.circles.len() == 0 {
            return None;
        }

        let mut sorted_by_distance = self.circles.clone();
        sorted_by_distance.sort_by(|a, b| {
            let distance_a = a.distance_to_another_center(center) as i32;
            let distance_b = b.distance_to_another_center(center) as i32;
            distance_a.cmp(&distance_b)
        });
        
        Some(sorted_by_distance[0])
    }

    fn pick_random_color(&self) -> Color {
        self.colors[random_index(2)]
    }
    
    fn pick_random_size(&self, circle_center: Coordinates, closest_neighbor: Option<Circle>) -> i32 {
        let get_rand = || self.sizes[random_index(self.sizes.len() as i32)].radius;
        match closest_neighbor {
            Some(closest) => {
                let max_size = closest.distance_from_edge_to_point(circle_center);
                for _candidates_idx in 0..self.sizes.len() {
                    let candidate = get_rand();
                    if candidate < max_size {
                        return candidate;
                    }
                }
                //fall back
                return 0;
            },
            None => get_rand()
        }
        
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

    /// Returns the distance between the center of this circle and the specified Coordinates
    pub fn distance_to_another_center(&self, other: Coordinates) -> f64 {
        let x_diff = self.center_x - other.0;
        let y_diff = self.center_y - other.1;
        ((x_diff.pow(2) + y_diff.pow(2)) as f64).sqrt() 
    }

    /// Returns the distance between the edge of this circle and the specified Coordinates
    pub fn distance_from_edge_to_point(&self, other: Coordinates) -> i32 {
        let distance_to_center = self.distance_to_another_center(other) as i32;
        distance_to_center - self.radius
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