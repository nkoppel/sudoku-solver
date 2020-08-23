mod puzzle;
mod solve;

use puzzle::*;
use solve::*;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn test_from_file(file: &str) {
    if let Ok(lines) = read_lines(file) {
        let mut linelist = Vec::new();

        for line in lines {
            linelist.push(line.unwrap());
        }

        let mut solver = Solver::new();
        let mut i = 0;
        let len = linelist.len();

        for line in linelist {
            solver.test(&line[0..81], &line[82.. ]);

            if i % 100000 == 0 {
                println!("{}/{}", i, len);
            }
            i += 1;
        }
    }
}

fn solve_from_file(file: &str) {
    if let Ok(lines) = read_lines(file) {
        let mut linelist = Vec::new();

        for line in lines {
            linelist.push(line.unwrap());
        }

        let mut solver = Solver::new();

        for line in &linelist {
            solver.puzzle =  Puzzle::from_str(line);

            solver.solve();
            // println!("{}", solver.puzzle.to_string());
        }
    }
}

fn main() {
    // let mut puzzle = Puzzle::from_str("010700098900001004000983200520400801000268000080000400000300000040002607001050000");

    // puzzle.solve(&groups, &squares);
    // println!("{}", puzzle.slow_is_valid(&squares));
    
    // test_from_file("/home/nathan/downloads/sudoku.csv");
    solve_from_file("/home/nathan/downloads/hardest_sudokus.txt");

    // test("000801200800000419100020060508300000070000520200000073020085007001000004760900080", "697841235852763419134529768518372946376498521249156873423685197981237654765914382", &groups, &squares);
}
