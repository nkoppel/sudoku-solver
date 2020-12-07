use crate::puzzle::*;

const ROW: u128 = 0o777;
const COLUMN: u128 = 0o001_001_001_001_001_001_001_001_001;
const BOX: u128 = 0o7007007;

#[inline]
pub fn quick_columns(mut board: u128) -> u128 {
    let mut twice = 0;

    twice |= board & (board >> 27);
    board |= board >> 27;

    twice |= board & (board >> 54);
    board |= board >> 54;

    board &= 0o777_777_777;

    twice |= twice >> 9;
    twice |= twice >> 18;

    twice |= board & (board >> 9);
    board |= board >> 9;

    twice |= board & (board >> 18);
    board |= board >> 18;

    board & !twice & ROW
}

#[inline]
pub fn quick_rows(mut board: u128) -> u128 {
    let three_column = 0o007_007_007_007_007_007_007_007_007;
    let mut twice = 0;

    twice |= board & (board >> 3);
    board |= (board >> 3) & three_column;

    twice |= board & (board >> 6);
    board |= board >> 6;

    board &= three_column;

    twice |= twice >> 1;
    twice |= twice >> 2;

    twice |= board & (board >> 1);
    board |= board >> 1;

    twice |= board & (board >> 2);
    board |= board >> 2;

    board & !twice & COLUMN
}

#[inline]
pub fn quick_boxes(mut board: u128) -> u128 {
    let three_row = 0o000_000_777_000_000_777_000_000_777;
    let res = 0o000_000_111_000_000_111_000_000_111;
    let mut twice = 0;

    twice |= board & (board >> 9);
    board |= (board >> 9) & three_row;

    twice |= board & (board >> 18);
    board |= board >> 18;

    board &= three_row;

    twice |= (twice >> 1) & res;
    twice |= twice >> 2;

    twice |= board & (board >> 1);
    board |= (board >> 1) & res;

    twice |= board & (board >> 2);
    board |= board >> 2;

    board & !twice & res
}

fn gen_group_table() -> [u128; 27] {
    let mut out = [0; 27];

    for i in 0..9 {
        out[i     ] = BOX << ((i % 3 * 3) + (i / 3 * 27));
        out[i +  9] = ROW << (i * 9);
        out[i + 18] = COLUMN << i;
    }

    out
}

fn gen_square_table() -> [u128; 81] {
    let groups = gen_group_table();
    let mut out = [0; 81];
    let mut tmp = 0;

    for i in 0..81 {
        for g in &groups {
            if *g & (1 << i) != 0 {
                tmp |= g;
            }
        }

        tmp &= !(1 << i);
        out[i] = !tmp;
        tmp = 0;
    }

    out
}

pub struct LocIter(pub u128);

impl Iterator for LocIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let out = self.0.trailing_zeros();
            self.0 ^= 1 << out;
            Some(out as usize)
        }
    }
}

pub struct Solver {
    pub puzzle: Puzzle,
    squares: [u128; 81],
    stack: Vec<(Puzzle, u8, u8)>
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            puzzle: Puzzle::new(),
            squares: gen_square_table(),
            stack: Vec::new()
        }
    }

    pub fn slow_is_valid(&self) -> bool {
        if !self.puzzle.is_valid() {
            return false;
        }

        let solved = self.puzzle.get_solved();

        for b in &self.puzzle.boards {
            for l in LocIter(b & solved) {
                if !self.squares[l] & b != 0 {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn update(&mut self) {
        let mut solved = self.puzzle.get_new_solved();

        while solved != 0 {
            while solved != 0 {
                for i in 0..9 {
                    for j in LocIter(self.puzzle.boards[i] & solved) {
                        self.puzzle.boards[i] &= self.squares[j];

                        if !self.puzzle.is_valid() {
                            return;
                        }
                    }
                }

                solved = self.puzzle.get_new_solved();
            }

            let mut num_solved = [0; 9];
            let mut all_solved = 0;

            let old_solved = self.puzzle.get_solved();

            for i in 0..9 {
                let unsolved = self.puzzle.boards[i] & !old_solved;

                let rows    = quick_rows(unsolved);
                let columns = quick_columns(unsolved);
                let boxes   = quick_boxes(unsolved);

                for j in LocIter(rows) {
                    let tmp = unsolved & (ROW << j);

                    num_solved[i] |= tmp;
                    all_solved |= tmp;
                }

                for j in LocIter(columns) {
                    let tmp = unsolved & (COLUMN << j);

                    num_solved[i] |= tmp;
                    all_solved |= tmp;
                }

                for j in LocIter(boxes) {
                    let tmp = unsolved & (BOX << j);

                    num_solved[i] |= tmp;
                    all_solved |= tmp;
                }
            }

            for i in 0..9 {
                self.puzzle.boards[i] &= !all_solved | num_solved[i];
            }

            solved = self.puzzle.get_new_solved();
        }
    }

    pub fn guess(&mut self) -> (u8, u8) {
        let options = self.puzzle.get_square_options();

        if options[0] == NEW_BOARD {
            return (81, 0);
        }

        let mut i = 1;
        while options[i] == 0 {i += 1}

        let sq = options[i].trailing_zeros();
        let bit = 1 << sq;

        let mut j = 0;
        while self.puzzle.boards[j] & bit == 0 {j += 1}

        for i in 0..9 {
            self.puzzle.boards[i] &= !bit;
        }
        self.puzzle.boards[j] |= bit;

        (sq as u8, j as u8)
    }

    pub fn solve(&mut self) -> bool {
        loop {
            self.update();

            while !self.puzzle.is_valid() {
                match self.stack.pop() {
                    Some((p, s, n)) => {
                        self.puzzle = p;

                        self.puzzle.boards[n as usize] &= !(1 << s);
                    }
                    None => return false
                }
            }

            let p = self.puzzle.clone();
            let (sq, num) = self.guess();

            if sq == 81 {
                if self.slow_is_valid() {
                    self.stack.clear();
                    return true;
                }
            } else {
                self.stack.push((p, sq, num));
            }
        }
    }

    pub fn test(&mut self, puzzle: &str, solution: &str) {
        let puzzle   = Puzzle::from_str(puzzle);
        let solution = Puzzle::from_str(solution);

        self.puzzle = puzzle.clone();

        assert!(self.solve() && self.puzzle == solution);
    }
}
