extern crate hyper;
extern crate serde;
use std::env;

mod youdao;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _ = args.remove(0);
    let input = args.join(" ");
    youdao::translate(input)
}
