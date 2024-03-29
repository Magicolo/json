extern crate json;
extern crate proptest;

use json::*;
use proptest::prelude::*;
use std::fs;

fn parse(json: &str) -> Node {
    let mut tree = json::parse(json).unwrap();
    tree.process();
    tree.root
}

fn convert<T: Convert>(value: &T) -> Node {
    let mut tree = json::convert(value);
    tree.process();
    tree.root
}

fn generate<T: Convert>(value: &T) -> String {
    json::generate(&json::convert(value))
}

#[test]
#[should_panic]
fn parse_empty() {
    parse("");
}

#[test]
fn parse_empty_object() {
    assert_eq!(parse("{}"), Node::Object(Vec::new()));
}

#[test]
#[should_panic]
fn parse_unbalanced_object() {
    parse("{");
}

#[test]
fn parse_empty_array() {
    assert_eq!(parse("[]"), Node::Array(Vec::new()));
}

#[test]
#[should_panic]
fn parse_unbalanced_array() {
    parse("[");
}

#[test]
fn parse_empty_string() {
    assert_eq!(parse(r#""""#), Node::String("".into()));
}

#[test]
#[should_panic]
fn parse_unbalanced_string() {
    parse(r#"""#);
}

#[test]
#[should_panic]
fn parse_quote_string() {
    assert_eq!(parse(r#"""""#), Node::String(r#"""#.into()));
}

#[test]
fn parse_space_strings() {
    assert_eq!(parse(r#""\t""#), Node::String("\t".into()));
    assert_eq!(parse(r#""\n""#), Node::String("\n".into()));
    assert_eq!(parse(r#""\r""#), Node::String("\r".into()));
    assert_eq!(parse(r#"" ""#), Node::String(" ".into()));
}

#[test]
fn parse_slash_strings() {
    assert_eq!(parse(r#""\""#), Node::String("\\".into()));
    assert_eq!(parse(r#""/""#), Node::String("/".into()));
    assert_eq!(parse(r#""\/""#), Node::String("/".into()));
}

#[test]
fn parse_zero() {
    assert_eq!(parse("0"), Node::Number(0.0));
}

#[test]
#[should_panic]
fn parse_missing_number_after_minus() {
    parse("-");
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

    #[test]
    fn parse_alpha_numerical_strings(value in "[a-zA-Z0-9]+") {
        prop_assert_eq!(parse(&format!(r#""{0}""#, value)), Node::String(value));
    }

    #[test]
    fn parse_escaped_strings(value in r"(\[nrtbf/\])+") {
        let escaped = value
            .replace(r"\n", "\n")
            .replace(r"\r", "\r")
            .replace(r"\t", "\t")
            .replace(r"\b", "\u{0008}")
            .replace(r"\f", "\u{000C}")
            .replace(r"\\", r"\")
            .replace(r#"\""#, r#"""#)
            .replace(r"\/", "/");
        prop_assert_eq!(parse(&format!(r#""{0}""#, escaped)), Node::String(value));
    }

    // #[test]
    // fn parse_unicode_strings(value in "[0-9a-fA-F]{4}") {
    //     let value = format!(r"\u{}", value);
    //     println!("{}", &value);
    // }
}
