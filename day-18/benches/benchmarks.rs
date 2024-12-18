use day_18::{part1, part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(
        divan::black_box(include_str!("../input-puzzle.txt",)),
        (70, 70),
        1024,
    )
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(
        divan::black_box(include_str!("../input-puzzle.txt",)),
        (70, 70),
        1024,
    )
    .unwrap();
}
