pub fn print_board(b: u128) {
    for y in (0..9).rev() {
        for x in (0..9).rev() {
            if b & (1 << (x + y * 9)) != 0 {
                print!("# ");
            } else {
                print!("_ ");
            }
        }

        println!();
    }
    println!();
}

pub const NEW_BOARD: u128 = 0x1_fffff_fffff_fffff_fffff;

#[derive(Clone)]
pub struct Puzzle{
    pub boards: [u128; 9],
    solved: u128
}

impl Puzzle {
    pub const fn new() -> Self {
        Puzzle {
            boards: [NEW_BOARD; 9],
            solved: 0
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut all_solved = 0;
        let mut out = Self::new();

        out.boards = [0; 9];

        for i in 0..81.min(s.len()) {
            if let Ok(n) = s[i..i+1].parse::<usize>() {
                if n > 0 {
                    out.boards[n - 1] |= 1 << i;
                    all_solved |= 1 << i;
                }
            }
        }

        let unsolved = !all_solved & NEW_BOARD;

        for b in out.boards.iter_mut() {
            *b |= unsolved;
        }

        out
    }

    pub fn to_squarewise(&self) -> [u16; 81] {
        let mut out = [0; 81];

        for i in 0..81 {
            for j in 0..9 {
                out[i] |= ((self.boards[j] & (1 << i) != 0) as u16) << j;
            }
        }

        out
    }

    pub fn to_string(&self) -> String {
        let squarewise = self.to_squarewise();
        let mut out = String::new();

        for x in squarewise.iter() {
            if x.is_power_of_two() {
                out.push((x.trailing_zeros() + 49) as u8 as char);
            } else {
                out.push('0');
            }
        }

        out
    }

    pub fn get_solved(&self) -> u128 {
        let mut once = 0;
        let mut twice = 0;

        for b in &self.boards {
            twice |= once & *b;
            once |= *b;
        }

        once & !twice
    }

    pub fn get_new_solved(&mut self) -> u128 {
        let out = self.get_solved() & !self.solved;
        self.solved |= out;
        out
    }

    pub fn is_valid(&self) -> bool {
        let mut tmp = 0;

        for b in &self.boards {
            tmp |= *b;
        }

        tmp == NEW_BOARD
    }

    pub fn get_square_options(&self) -> [u128; 9] {
        let mut out = [0; 9];

        for b in &self.boards {
            for i in (1..9).rev() {
                out[i] |= out[i - 1] & *b;
            }

            out[0] |= b;
        }

        for i in 0..8 {
            out[i] &= !out[i + 1];
        }

        out
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        self.boards == other.boards
    }
}

const UNKNOWN: u16 = 0x1ff;

use std::fmt;

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let squarewise = self.to_squarewise();

        for y in 0..9 {
            for x in 0..9 {
                let s = squarewise[x + y * 9];

                if s.is_power_of_two() {
                    write!(f, " {}", s.trailing_zeros() + 1)?;
                } else {
                    write!(f, " _")?;
                }

                if x % 3 == 2 && x < 8 {
                    write!(f, " |")?;
                }
            }

            writeln!(f)?;

            if y % 3 == 2 && y < 8 {
                writeln!(f, "-------+-------+-------")?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minor_sep_top = "............||...........||............";
        let minor_sep = ":...:...:...||...:...:...||...:...:...:";
        let major_sep = "============##===========##============";

        let squarewise = self.to_squarewise();

        writeln!(f, "{}", minor_sep_top)?;

        for y in 0..9 {
            for r in 0..3 {
                write!(f, ":")?;
                for x in 0..9 {
                    for b in r * 3..r * 3 + 3 {
                        let s = squarewise[x + y * 9];

                        if s == UNKNOWN {
                            write!(f, "_")?;
                        } else if s & (1 << b) != 0 {
                            write!(f, "{}", b + 1)?;
                        } else {
                            write!(f, " ")?;
                        }
                    }

                    if x % 3 == 2 && x < 8 {
                        write!(f, "||")?;
                    } else if x < 8 {
                        write!(f, ":")?;
                    }
                }
                writeln!(f, ":")?;
            }

            if y % 3 == 2 && y < 8 {
                writeln!(f, "{}", major_sep)?;
            } else if y < 8 {
                writeln!(f, "{}", minor_sep)?;
            }
        }
        writeln!(f, "{}", minor_sep)?;

        Ok(())
    }
}
