mod puzzle;
mod solve;

use puzzle::*;
use solve::*;

use std::fs::File;
use std::io::{Lines, Read, BufRead, BufReader, stdin};

fn test_from_lines<T>(lines: Lines<T>)
    where T: BufRead
{
    let mut solver = Solver::new();
    let mut i = 0;

    for line in lines {
        let line = line.unwrap();
        solver.test(&line[0..81], &line[82.. ]);

        println!("Puzzle #{} solved successfully", i);
        i += 1;
    }

}

fn solve_from_lines<T>(lines: Lines<T>)
    where T: BufRead
{
    let mut solver = Solver::new();

    for line in lines {
        let line = line.unwrap();
        solver.puzzle =  Puzzle::from_str(&line);

        if !solver.solve() {
            panic!("Unable to solve puzzle: {}", line);
        }
        // println!("{}", solver.puzzle.to_string());
        // println!("{}", solver.puzzle);
    }
}

fn main() {
    // let file: Box<dyn Read> =
        // match std::env::args().collect::<Vec<_>>().get(1) {
            // Some(file) => Box::new(File::open(file).expect("File not found")),
            // None => Box::new(stdin())
        // };

    // let reader = BufReader::new(file).lines();

    // solve_from_lines(reader);

    // let mut solver = Solver::new();
    // solver.puzzle = Puzzle::from_str("000000002001000700030050090000006040003040800040509000090060030002000100700003000");

    // println!("{:?}", solver.puzzle);

    // println!("{}", solver.solve());

    // println!("{:?}", solver.puzzle);
    
    let test = 0x186c10030b8300000160f;

    print_board(test);

    let (tmp1, tmp2) = quick_horiz_triads(test);

    print_board(tmp1);
    print_board(tmp2);
}
