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
    let groups = gen_group_table();
    let squares = gen_square_table();

    b.iter(|| {
        let mut p = puzzle.clone();
        test::black_box(&mut p).solve(&groups, &squares)
    });
}

#[bench]
fn bench_slow_is_valid(b: &mut Bencher) {
    let puzzle = Puzzle::from_str(TEST_PUZZLE);
    let squares = gen_square_table();

    b.iter(|| test::black_box(&puzzle).slow_is_valid(&squares));
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let puzzle = Puzzle::from_str(TEST_PUZZLE);
    let groups = gen_group_table();
    let squares = gen_square_table();

    b.iter(|| {
        let mut p = puzzle.clone();
        test::black_box(&mut p).update(&groups, &squares)
    });
}

#[bench]
fn bench_guess(b: &mut Bencher) {
    let puzzle = Puzzle::from_str(TEST_PUZZLE);

    b.iter(|| {
        let mut p = puzzle.clone();
        test::black_box(&mut p).guess()
    });
}

#[bench]
fn bench_get_square_options(b: &mut Bencher) {
    let mut puzzle = Puzzle::from_str(TEST_PUZZLE);
    let groups = gen_group_table();
    let squares = gen_square_table();

    puzzle.update(&groups, &squares);

    b.iter(|| test::black_box(&puzzle).get_square_options());
}
