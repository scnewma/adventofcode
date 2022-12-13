use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! gen_benches {
    ( $( $day:ident ),* ) => {
        fn criterion_benchmark(c: &mut Criterion) {
            $(
                {
                    use adventofcode22::$day;
                    const INPUT: &str = include_str!(concat!("../inputs/", stringify!($day), ".input.txt"));
                    c.bench_function(concat!(stringify!($day), "::part01"), |b| b
                        .iter(|| $day::part01(INPUT)));
                    c.bench_function(concat!(stringify!($day), "::part02"), |b| b
                        .iter(|| $day::part02(INPUT)));
                }
            )*
        }
    };
}

gen_benches![day01, day02, day03, day04, day05, day06, day07, day08, day09, day12, day13];

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
