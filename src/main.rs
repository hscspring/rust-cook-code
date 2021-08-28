mod enum_struct_trait_base;
mod enum_struct_trait;
mod multi_args;
mod scrape_url;

use enum_struct_trait::{ Shape, ShapeEnum };
use multi_args::Shape as MShape;

fn main() {
    let se = Shape::new(ShapeEnum::Rectangle(2.0, 3.0));
    println!("{:?}", se.shape);

    let me = MShape::new(1.0);
    println!("{:?}", me);
}
