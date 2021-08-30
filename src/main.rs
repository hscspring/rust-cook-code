mod enum_struct_trait_base;
mod enum_struct_trait;
mod multi_args;
mod scrape_url;
mod fib;

use std::{env};
use enum_struct_trait::{ Shape, ShapeEnum };
use multi_args::Shape as MShape;

fn main() {
    let se = Shape::new(ShapeEnum::Rectangle(2.0, 3.0));
    println!("{:?}", se.shape);

    let me = MShape::new(1.0);
    println!("{:?}", me);

    let args: Vec<String> = env::args().collect();
    let (url, output) = parse_config(&args);
    if let Err(e) = scrape_url::scrape(url, output) {
        println!("{}", e);
    }

    fib::get_fib1(10);
    fib::get_fib2(10);
    fib::get_fib3(10);

}

fn parse_config(args: &[String]) -> (&str, &str) {
    let url = &args[1];
    let output = &args[2];
    (url, output)
}