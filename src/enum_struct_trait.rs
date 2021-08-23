#[derive(Debug)]
pub enum ShapeEnum {
    Rectangle(f64, f64),
    Circle(f64),
    Triangle(f64, f64, f64),
}

#[derive(Debug)]
pub struct Shape {
    pub shape: ShapeEnum,
}

impl Shape {
    pub fn new(shape: ShapeEnum) -> Shape {
        Shape { shape: shape }
    }
}

pub trait Calculate {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
}

impl Calculate for Shape {
    fn perimeter(&self) -> f64 {
        match self.shape {
            ShapeEnum::Rectangle(a, b) => 2.0 * (a + b),
            ShapeEnum::Circle(r) => 2.0 * 3.14 * r,
            ShapeEnum::Triangle(a, b, c) => a + b + c,
        }
    }
    fn area(&self) -> f64 {
        match self.shape {
            ShapeEnum::Rectangle(a, b) => a * b,
            ShapeEnum::Circle(r) => 3.14 * r * r,
            ShapeEnum::Triangle(a, b, c) => {
                let p = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).sqrt()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perimeter() {
        let se = Shape::new(ShapeEnum::Rectangle(2.0, 3.0));
        assert_eq!(se.perimeter(), 10.0);
    }

    #[test]
    fn area() {
        let se = Shape::new(ShapeEnum::Triangle(3.0, 4.0, 5.0));
        assert_eq!(se.area(), 6.0);
    }
}
