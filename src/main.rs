mod puzzle;
mod solve;

use puzzle::*;
use solve::*;

use std::fs::File;
use std::io::{self, Lines, Read, BufRead, BufReader, stdin};

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

        solver.solve();
        println!("{}", solver.puzzle);
    }
}

fn main() {
    let file: Box<dyn Read> =
        match std::env::args().collect::<Vec<_>>().get(1) {
            Some(file) => Box::new(File::open(file).expect("File not found")),
            None => Box::new(stdin())
        };

    let reader = BufReader::new(file).lines();

    solve_from_lines(reader);
}
