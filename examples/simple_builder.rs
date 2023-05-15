extern crate ressa_r;
use ressa_r::*;
fn main() {
    let js = include_str!("simple.js");
    let mut p = Parser::builder()
        .module(false)
        .tolerant(false)
        .js(js)
        .build()
        .unwrap();
    let script = p.parse().unwrap();
    println!("{:#?}", script);
}
