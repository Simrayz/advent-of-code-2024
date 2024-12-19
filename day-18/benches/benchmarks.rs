use day_18::{part1, part2, MemorySpace};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn parse_memory_space() {
    MemorySpace::new(70, divan::black_box(include_str!("../input-puzzle.txt",)));
}

#[divan::bench]
fn part1() {
    part1::process(
        divan::black_box(include_str!("../input-puzzle.txt",)),
        70,
        1024,
    )
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(
        divan::black_box(include_str!("../input-puzzle.txt",)),
        70,
        1024,
    )
    .unwrap();
}
