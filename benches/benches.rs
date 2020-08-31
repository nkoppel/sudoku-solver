#![feature(test)]

use sudoku2::*;

extern crate test;
use test::Bencher;

const TEST_PUZZLE: &str = "000801200800000419100020060508300000070000520200000073020085007001000004760900080";

#[bench]
fn bench_to_squarewise(b: &mut Bencher) {
    let puzzle = Puzzle::from_str(TEST_PUZZLE);

    b.iter(|| test::black_box(&puzzle).to_squarewise());
}

#[bench]
fn bench_from_string(b: &mut Bencher) {
    b.iter(|| Puzzle::from_str(test::black_box(TEST_PUZZLE)));
}

#[bench]
fn bench_solve(b: &mut Bencher) {
    let puzzle = Puzzle::from_str(TEST_PUZZLE);
    let mut solver = Solver::new();

    b.iter(|| {
        solver.puzzle = puzzle.clone();
        test::black_box(&mut solver).solve()
    });
}

#[bench]
fn bench_slow_is_valid(b: &mut Bencher) {
    let puzzle = Puzzle::from_str(TEST_PUZZLE);
    let mut solver = Solver::new();

    b.iter(|| test::black_box(&solver).slow_is_valid());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let puzzle = Puzzle::from_str(TEST_PUZZLE);
    let mut solver = Solver::new();

    b.iter(|| {
        solver.puzzle = puzzle.clone();
        test::black_box(&mut solver).update()
    });
}

#[bench]
fn bench_guess(b: &mut Bencher) {
    let puzzle = Puzzle::from_str(TEST_PUZZLE);
    let mut solver = Solver::new();

    b.iter(|| {
        solver.puzzle = puzzle.clone();
        test::black_box(&mut solver).guess()
    });
}
