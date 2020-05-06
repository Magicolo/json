extern crate json;

use json::*;
use std::fs;

fn main() {
    const TWITTER: &str = r"resources\twitter.json";
    let json = fs::read_to_string(TWITTER).unwrap();
    loop {
        parse(&json).unwrap();
    }
}
