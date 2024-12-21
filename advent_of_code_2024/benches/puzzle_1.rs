use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2024::bin::puzzle_1::{part_a, part_b};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("puzzle 1a easy", |b| b.iter(|| part_a("./data/puzzle1/easy.txt")));
    c.bench_function("puzzle 1b easy", |b| b.iter(|| part_b("./data/puzzle1/easy.txt")));
    c.bench_function("puzzle 1a input", |b| b.iter(|| part_a("./data/puzzle1/input.txt")));
    c.bench_function("puzzle 1b input", |b| b.iter(|| part_b("./data/puzzle1/input.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
