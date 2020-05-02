extern crate criterion;
extern crate json;

use criterion::{criterion_group, criterion_main, Criterion};
use json::parse;
use std::fs;

pub fn parse_twitter(criterion: &mut Criterion) {
    const TWITTER: &str = r#"resources/twitter.json"#;
    let json = fs::read_to_string(TWITTER).unwrap();
    criterion.bench_function("parse_twitter", |bencher| bencher.iter(|| parse(&json)));
}

criterion_group!(benches, parse_twitter);
criterion_main!(benches);
