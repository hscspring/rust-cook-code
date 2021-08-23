#[derive(Debug)]
pub struct Circle {
    r: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    a: f64, 
    b: f64,
}

#[derive(Debug)]
pub struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

#[derive(Debug)]
pub enum Shape {
    Rectangle,
    Circle,
    Triangle,
}

pub trait Calculate {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
}

impl Calculate for Rectangle {
    fn perimeter(&self) -> f64 {
        (self.a + self.b) * 2.0
    }

    fn area(&self) -> f64 {
        self.a * self.b
    }
}

impl Calculate for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * 3.14 * self.r
    }

    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

impl Calculate for Triangle {
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }

    fn area(&self) -> f64 {
        let p = (self.a + self.b + self.c) / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle() {
        let c = Circle { r: 1.0 };
        assert_eq!(3.14, c.area());
        assert_eq!(6.28, c.perimeter());
    }

    #[test]
    fn rectangle() {
        let c = Rectangle { a: 2.0 , b: 3.0 };
        assert_eq!(6.0, c.area());
        assert_eq!(10.0, c.perimeter());
    }

    #[test]
    fn triangle() {
        let c = Triangle { a: 3.0, b: 4.0, c: 5.0 };
        assert_eq!(6.0, c.area());
        assert_eq!(12.0, c.perimeter());
    }
}
