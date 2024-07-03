use std::f64::consts::PI;

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            center: Point { x: x, y: y },
            radius: r,
        }
    }

    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
    pub fn intersect(&self, circle: &Circle) -> bool {
        if self.center.distance(&circle.center) > circle.radius + self.radius {
			return false;
		}
		true
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, point: &Point) -> f64 {
        ((self.x - point.x).powi(2) + (self.y - point.y).powi(2)).sqrt()
    }
}
