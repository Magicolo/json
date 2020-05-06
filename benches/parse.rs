extern crate criterion;
extern crate json;

use criterion::{criterion_group, criterion_main, Criterion};
use json::*;
use std::fs;

pub fn parse_twitter(criterion: &mut Criterion) {
    const TWITTER: &str = r"resources/twitter.json";
    let json = fs::read_to_string(TWITTER).unwrap();
    let mut group = criterion.benchmark_group("twitter");
    group.bench_function("parse", |bencher| bencher.iter(|| parse(&json)));
    group.finish();
}

criterion_group!(benches, parse_twitter);
criterion_main!(benches);
