use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2024::puzzles::{puzzle1, puzzle2, puzzle3, puzzle4, puzzle5, puzzle6, puzzle7, puzzle8, puzzle9, puzzle10, puzzle11, puzzle12, puzzle13, puzzle14, puzzle15, puzzle16, puzzle17, puzzle18, puzzle19, puzzle20, puzzle21, puzzle22, puzzle23, puzzle24, puzzle25};

fn puzzle_benchmark(c: &mut Criterion) {
    c.bench_function("puzzle 1a input", |b| b.iter(|| puzzle1::part_a("./data/puzzle1/input.txt")));
    c.bench_function("puzzle 1b input", |b| b.iter(|| puzzle1::part_b("./data/puzzle1/input.txt")));
    c.bench_function("puzzle 2a input", |b| b.iter(|| puzzle2::part_a("./data/puzzle2/input.txt")));
    c.bench_function("puzzle 2b input", |b| b.iter(|| puzzle2::part_b("./data/puzzle2/input.txt")));
    c.bench_function("puzzle 3a input", |b| b.iter(|| puzzle3::part_a("./data/puzzle3/input.txt")));
    c.bench_function("puzzle 3b input", |b| b.iter(|| puzzle3::part_b("./data/puzzle3/input.txt")));
    c.bench_function("puzzle 4a input", |b| b.iter(|| puzzle4::part_a("./data/puzzle4/input.txt")));
    c.bench_function("puzzle 4b input", |b| b.iter(|| puzzle4::part_b("./data/puzzle4/input.txt")));
    c.bench_function("puzzle 5a input", |b| b.iter(|| puzzle5::part_a("./data/puzzle5/input.txt")));
    c.bench_function("puzzle 5b input", |b| b.iter(|| puzzle5::part_b("./data/puzzle5/input.txt")));
    c.bench_function("puzzle 6a input", |b| b.iter(|| puzzle6::part_a("./data/puzzle6/input.txt")));
    c.bench_function("puzzle 6b input", |b| b.iter(|| puzzle6::part_b("./data/puzzle6/input.txt")));
    c.bench_function("puzzle 7a input", |b| b.iter(|| puzzle7::part_a("./data/puzzle7/input.txt")));
    c.bench_function("puzzle 7b input", |b| b.iter(|| puzzle7::part_b("./data/puzzle7/input.txt")));
    c.bench_function("puzzle 8a input", |b| b.iter(|| puzzle8::part_a("./data/puzzle8/input.txt")));
    c.bench_function("puzzle 8b input", |b| b.iter(|| puzzle8::part_b("./data/puzzle8/input.txt")));
    c.bench_function("puzzle 9a input", |b| b.iter(|| puzzle9::part_a("./data/puzzle9/input.txt")));
    c.bench_function("puzzle 9b input", |b| b.iter(|| puzzle9::part_b("./data/puzzle9/input.txt")));
    c.bench_function("puzzle 10a input", |b| b.iter(|| puzzle10::part_a("./data/puzzle10/input.txt")));
    c.bench_function("puzzle 10b input", |b| b.iter(|| puzzle10::part_b("./data/puzzle10/input.txt")));
    c.bench_function("puzzle 11a input", |b| b.iter(|| puzzle11::part_a("./data/puzzle11/input.txt")));
    c.bench_function("puzzle 11b input", |b| b.iter(|| puzzle11::part_b("./data/puzzle11/input.txt")));
    c.bench_function("puzzle 12a input", |b| b.iter(|| puzzle12::part_a("./data/puzzle12/input.txt")));
    c.bench_function("puzzle 12b input", |b| b.iter(|| puzzle12::part_b("./data/puzzle12/input.txt")));
    c.bench_function("puzzle 13a input", |b| b.iter(|| puzzle13::part_a("./data/puzzle13/input.txt")));
    c.bench_function("puzzle 13b input", |b| b.iter(|| puzzle13::part_b("./data/puzzle13/input.txt")));
    c.bench_function("puzzle 14a input", |b| b.iter(|| puzzle14::part_a("./data/puzzle14/input.txt")));
    // c.bench_function("puzzle 14b input", |b| b.iter(|| puzzle14::part_b("./data/puzzle14/input.txt")));
    c.bench_function("puzzle 15a input", |b| b.iter(|| puzzle15::part_a("./data/puzzle15/input.txt")));
    c.bench_function("puzzle 15b input", |b| b.iter(|| puzzle15::part_b("./data/puzzle15/input.txt")));
    c.bench_function("puzzle 16a input", |b| b.iter(|| puzzle16::part_a("./data/puzzle16/input.txt")));
    c.bench_function("puzzle 16b input", |b| b.iter(|| puzzle16::part_b("./data/puzzle16/input.txt")));
    c.bench_function("puzzle 17a input", |b| b.iter(|| puzzle17::part_a("./data/puzzle17/input.txt")));
    c.bench_function("puzzle 17b input", |b| b.iter(|| puzzle17::part_b("./data/puzzle17/input.txt")));
    c.bench_function("puzzle 18a input", |b| b.iter(|| puzzle18::part_a("./data/puzzle18/input.txt")));
    c.bench_function("puzzle 18b input", |b| b.iter(|| puzzle18::part_b("./data/puzzle18/input.txt")));
    c.bench_function("puzzle 19a input", |b| b.iter(|| puzzle19::part_a("./data/puzzle19/input.txt")));
    c.bench_function("puzzle 19b input", |b| b.iter(|| puzzle19::part_b("./data/puzzle19/input.txt")));
    c.bench_function("puzzle 20a input", |b| b.iter(|| puzzle20::part_a("./data/puzzle20/input.txt")));
    c.bench_function("puzzle 20b input", |b| b.iter(|| puzzle20::part_b("./data/puzzle20/input.txt")));
    c.bench_function("puzzle 21a input", |b| b.iter(|| puzzle21::part_a("./data/puzzle21/input.txt")));
    c.bench_function("puzzle 21b input", |b| b.iter(|| puzzle21::part_b("./data/puzzle21/input.txt")));
    c.bench_function("puzzle 22a input", |b| b.iter(|| puzzle22::part_a("./data/puzzle22/input.txt")));
    c.bench_function("puzzle 22b input", |b| b.iter(|| puzzle22::part_b("./data/puzzle22/input.txt")));
    c.bench_function("puzzle 23a input", |b| b.iter(|| puzzle23::part_a("./data/puzzle23/input.txt")));
    c.bench_function("puzzle 23b input", |b| b.iter(|| puzzle23::part_b("./data/puzzle23/input.txt")));
    c.bench_function("puzzle 24a input", |b| b.iter(|| puzzle24::part_a("./data/puzzle24/input.txt")));
    c.bench_function("puzzle 24b input", |b| b.iter(|| puzzle24::part_b("./data/puzzle24/input.txt")));
    c.bench_function("puzzle 25a input", |b| b.iter(|| puzzle25::part_a("./data/puzzle25/input.txt")));
    c.bench_function("puzzle 25b input", |b| b.iter(|| puzzle25::part_b("./data/puzzle25/input.txt")));
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.01).sample_size(10);
    targets = puzzle_benchmark
}
criterion_main!(benches);
