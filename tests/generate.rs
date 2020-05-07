extern crate json;
extern crate proptest;

use json::*;
use proptest::prelude::*;

fn generate(node: Node) -> String {
    json::generate(&Tree::new(node))
}

#[test]
fn generate_empty_string() {
    assert_eq!(generate(Node::String("".into())), r#""""#);
}

#[test]
fn generate_quote_string() {
    assert_eq!(generate(Node::String("\"".into())), r#""\"""#);
}

#[test]
fn generate_space_strings() {
    assert_eq!(generate(Node::String("\n".into())), r#""\n""#);
    assert_eq!(generate(Node::String("\r".into())), r#""\r""#);
    assert_eq!(generate(Node::String("\t".into())), r#""\t""#);
}

#[test]
fn generate_slash_strings() {
    assert_eq!(generate(Node::String("\\".into())), r#""\\""#);
    assert_eq!(generate(Node::String("/".into())), r#""/""#);
}

#[test]
fn generate_special_string() {
    assert_eq!(
        generate(Node::String("\"\n\\\r\t".into())),
        r#""\"\n\\\r\t""#
    );
}

#[test]
fn generate_unicode_string() {
    assert_eq!(generate(Node::String('\u{80}'.to_string())), r#""\u0080""#);
}

#[test]
fn generate_empty_array() {
    assert_eq!(generate(Node::Array(Vec::new())), "[]");
}

#[test]
fn generate_empty_object() {
    assert_eq!(generate(Node::Object(Vec::new())), "{}");
}

#[test]
fn generate_bool() {
    assert_eq!(generate(Node::Boolean(true)), "true");
    assert_eq!(generate(Node::Boolean(false)), "false");
}

proptest! {
    #[test]
    fn generate_integers(value: i32) {
        prop_assert_eq!(generate(Node::Number(value as f64)), value.to_string());
    }

    #[test]
    fn generate_fractionnal(value: f64) {
        prop_assert_eq!(generate(Node::Number(value)), value.to_string());
    }

    #[test]
    fn generate_alpha_numerical_strings(value in "[a-zA-Z0-9]+") {
        let right = format!(r#""{}""#, value);
        prop_assert_eq!(generate(Node::String(value)), right);
    }

    #[test]
    fn generate_unicode_strings(value in 128..1000) {
        let left = format!("{}", std::char::from_u32(value as u32).unwrap());
        let mut json = format!(r#""\u{:X}""#, value);
        while json.len() < 8 {
            json.insert(3, '0');
        }
        prop_assert_eq!(generate(Node::String(left)), json);
    }
}
