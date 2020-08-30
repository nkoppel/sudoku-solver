use crate::puzzle::*;

const ROW: u128 = 0x1ff;
const COLUMN: u128 = 0x1008040201008040201;
const BOX: u128 = 0b111000000111000000111;

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
    groups: [u128; 27],
    stack: Vec<(Puzzle, u8, u8)>
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            puzzle: Puzzle::new(),
            squares: gen_square_table(),
            groups: gen_group_table(),
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

            for i in 0..9 {
                if (self.puzzle.boards[i] & self.puzzle.get_solved()).count_ones() < 9 {
                    for g in &self.groups {
                        let tmp = self.puzzle.boards[i] & g;

                        if tmp.is_power_of_two() {
                            num_solved[i] |= tmp;
                            all_solved |= tmp;
                        }
                    }
                }
            }

            for i in 0..9 {
                self.puzzle.boards[i] &= !all_solved;
                self.puzzle.boards[i] |= num_solved[i];
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
