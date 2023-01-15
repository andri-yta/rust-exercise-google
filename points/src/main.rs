// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        let calc:f64 = (self.x.pow(2) + self.y.pow(2)).into();
        calc.sqrt()
    }

    pub fn dist(&self, other: Point) -> f64 {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let calc:f64 = (x.pow(2) + y.pow(2)).into();
        calc.sqrt()
    }

}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, new_point: Point) {
        self.points.push(new_point);
    }

    pub fn left_most_point(&self) -> Option<&Point> {
        if self.points.len() > 0 {
            let mut left = &self.points[0];
            for p in &self.points[1..] { // self.points.iter() {
                if p.x < left.x {
                    left = p;
                }
            }
            Some(&left)
        } else {
            None
        }
    }

    pub fn perimeter(&self) -> f64 {
        if self.points.len() > 0 {
            let mut result = 0.0;
            let mut current_point = self.points[0];
            for p in &self.points[1..] {
                result += current_point.dist(*p);
                current_point = *p;
            }
            result += current_point.dist(self.points[0]);
            result
        } else {
            0.0
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&Point> + '_ {
        self.points.iter()
    }

}


pub struct Circle {
    c: Point,
    r: i32,
}

impl Circle {
    pub fn new(c: Point, r: i32) -> Self {
        Self { c, r }
    }

    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * f64::from(self.r)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Self::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}

impl Shape {
    pub fn circumference(&self) -> f64  {
        match self {
            Shape::Polygon(poly) => poly.perimeter(),
            Shape::Circle(circle) => circle.circumference(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(&p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}