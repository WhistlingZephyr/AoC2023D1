use d1::part2;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn part2bench(c: &mut Criterion) {
    let input = include_str!("../input.txt").trim();
    let mut group = c.benchmark_group("day01.part2");
    group.bench_function("solution", |b| {
        b.iter(|| part2::solve(black_box(input)))
    });
    group.finish();
}

criterion_group!(benches, part2bench);
criterion_main!(benches);