extern crate json;
extern crate proptest;

use json::{Node, Serializable};
use proptest::prelude::*;
use std::fs;

fn parse(json: &str) -> Node {
    json::parse(json).unwrap().root
}

fn convert<T: Serializable>(value: &T) -> Node {
    json::convert(value).root
}

fn generate<T: Serializable>(value: &T) -> String {
    json::generate(&json::convert(value))
}

#[test]
fn parse_empty_object() {
    assert_eq!(parse("{}"), Node::Object((0, 0)));
}
#[test]
fn parse_empty_array() {
    assert_eq!(parse("[]"), Node::Array((0, 0)));
}
#[test]
fn parse_empty_string() {
    assert_eq!(parse(r#""""#), Node::String((0, 0)));
}
#[test]
fn parse_zero() {
    assert_eq!(parse("0"), Node::Number(0.0));
}
#[test]
fn parse_negative_with_0_exponent() {
    assert_eq!(parse("-1e0"), Node::Number(-1.0));
}
#[test]
fn parse_twitter() {
    const TWITTER: &str = r#"resources/twitter.json"#;
    let json = fs::read_to_string(TWITTER).unwrap();
    parse(&json);
}

proptest! {
    #[test]
    fn parse_integers(value: i32) {
        prop_assert_eq!(parse(&value.to_string()), Node::Number(value as f64));
    }

    #[test]
    fn parse_integers_with_exponents(value: i32, exponent in -20..20) {
        let json = format!("{0}e{1}", value, exponent);
        let value = (value as f64) * (10 as f64).powi(exponent);
        prop_assert_eq!(parse(&json), Node::Number(value));
    }

    #[test]
    fn parse_fractional(value: i32, fraction: u32) {
        let json = format!("{0}.{1}", value, fraction);
        let mut fraction = fraction as f64;
        while fraction >= 1.0 {
            fraction /= 10.0;
        }
        let value = value as f64 + fraction * value.signum() as f64;
        prop_assert_eq!(parse(&json), Node::Number(value));
    }

    #[test]
    fn parse_fractional_with_exponent(value: i32, fraction: u32, exponent in -20..20) {
        let json = format!("{0}.{1}E{2}", value, fraction, exponent);
        let mut fraction = fraction as f64;
        while fraction >= 1.0 {
            fraction /= 10.0;
        }
        let exponent = (10 as f64).powi(exponent);
        let value = (value as f64 + fraction * value.signum() as f64) * exponent;
        prop_assert_eq!(parse(&json), Node::Number(value));
    }

    #[test]
    fn parse_integer_vector(values: Vec<i32>) {
        prop_assert_eq!(parse(&generate(&values)), convert(&values));
    }
}
