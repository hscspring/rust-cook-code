#[derive(Debug)]
pub enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    pub fn new<T>(args: T) -> Shape
        where T: Into<Shape>
    {
        args.into()
    }
}

impl From<f64> for Shape {
    fn from(a: f64) -> Shape {
        Shape::Circle(a)
    }
}

impl From<(f64, f64)> for Shape {
    fn from((a, b): (f64, f64)) -> Shape {
        Shape::Rectangle(a, b)
    }
}

impl From<(f64, f64, f64)> for Shape {
    fn from((a, b, c): (f64, f64, f64)) -> Shape {
        Shape::Triangle(a, b, c)
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn one_arg() {
        let c1 = Shape::new(1.0);
        println!("{:?}", c1);

        let c2 = Shape::Circle(1.0);
        println!("{:?}", c2);
    }

    #[test]
    fn two_args() {
        let r1 = Shape::new((1.0, 2.0));
        println!("{:?}", r1);

        let r2 = Shape::Rectangle(1.0, 2.0);
        println!("{:?}", r2);
    }

    #[test]
    fn three_args() {
        let t1 = Shape::new((1.0, 2.0, 3.0));
        println!("{:?}", t1);

        let t2 = Shape::Triangle(1.0, 2.0, 3.0);
        println!("{:?}", t2);
    }


}