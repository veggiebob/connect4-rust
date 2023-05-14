use std::cmp::min;
use crate::connect4board::Error::BadDrop;
use crate::util::matrix_to_string;
use std::fmt::{Display, Formatter};

pub const BOARD_COLS: usize = 7;
pub const BOARD_ROWS: usize = 6;

pub type Team = u8;
pub const EMPTY: Team = 0;
pub const TEAM_X: Team = 1;
pub const TEAM_O: Team = 2;

#[derive(Debug)]
pub enum Error {
    BadDrop,
}

pub type Connect4Result<T> = Result<T, Error>;

// note: top row (row 0) represents the top of the physical board
#[derive(Clone)]
pub struct Board {
    pub mat: [[u8; BOARD_COLS]; BOARD_ROWS],
    pub next_empty_spot: [isize; BOARD_COLS],
}

impl Board {
    pub fn init_empty() -> Board {
        Board {
            mat: [[0; BOARD_COLS]; BOARD_ROWS],
            next_empty_spot: [(BOARD_ROWS - 1) as isize; BOARD_COLS],
        }
    }

    pub fn is_full(&self) -> bool {
        !self.next_empty_spot.iter().any(|x| *x >= 0)
    }

    pub fn valid_choices(&self) -> Vec<usize> {
        (0..BOARD_COLS).into_iter().filter(|c| self.next_empty_spot[*c] >= 0).collect()
    }

    pub fn drop(&mut self, col: usize, team: Team) -> Connect4Result<()> {
        if self.next_empty_spot[col] < 0 {
            Connect4Result::Err(BadDrop)
        } else {
            let mut es = &mut self.next_empty_spot[col];
            self.mat[*es as usize][col] = team;
            *es -= 1;
            Connect4Result::Ok(())
        }
    }

    pub fn undrop(&mut self, col: usize) -> Connect4Result<()> {
        if self.next_empty_spot[col] >= (BOARD_ROWS - 1) as isize {
            Err(BadDrop)
        } else {
            self.next_empty_spot[col] += 1;
            self.mat[self.next_empty_spot[col] as usize][col] = EMPTY;
            Ok(())
        }
    }

    // returns ((row1, col1), (row2, col2))
    pub fn won(&self) -> Option<((usize, usize), (usize, usize), Team)> {
        let mut row1: usize = 0;
        let mut row2: usize = 0;
        let mut col1: usize = 0;
        let mut col2: usize = 0;
        // horizontal
        for (row, i) in self.mat.iter().zip(0..) {
            row1 = i;
            let mut t = row[0];
            let mut len = 0;
            col1 = 0;
            for (x, j) in row.iter().zip(0..) {
                if *x == t {
                    len += 1;
                    if t != EMPTY && len == 4 {
                        return Some((
                            (row1, col1), // row stays the same
                            (row1,    j),
                            t)
                        );
                    }
                } else {
                    t = *x;
                    col1 = j;
                    len = 1;
                }
            }
        }
        // vertical
        for (e, j) in self.next_empty_spot.iter().zip(0..) {
            if (BOARD_ROWS as isize - *e - 1) < 4 {
                continue;
            }
            col1 = j;
            col2 = j;
            row1 = (*e + 1) as usize;
            let mut t = self.mat[(*e + 1) as usize][j];
            let mut len: usize = 1;
            for row in (*e + 2) as usize..BOARD_ROWS {
                if self.mat[row][j] == t {
                    len += 1;
                    if len == 4 && t != EMPTY {
                        return Some((
                            (row1, col1), // column stays the same
                            (row , col1),
                            t
                        ));
                    }
                } else {
                    row1 = row;
                    len = 1;
                    t = self.mat[row][j];
                }
            }
        }

        // major diagonal
        // 0 _ 0 0 0
        // x y _ 3 4
        // 0 x y _ 0
        // 0 0 x y _
        // 0 0 1 x y
        // could definitely be more efficient
        // but this is technically still O(n^2)
        // and the more efficient way is also O(n^2)
        // not like it matters because it's connect4
        for i in 0..BOARD_ROWS - 3 {
            for j in 0..BOARD_COLS - 3 {
                // major
                let mut t = self.mat[i][j];
                if t != EMPTY {
                    let mut broke = false;
                    for p in 1..4 {
                        if self.mat[i + p][j + p] != t {
                            broke = true;
                            break;
                        }
                    }
                    if !broke {
                        return Some((
                            (i, j),
                            (i + 3, j + 3),
                            t
                        ));
                    }
                }

                // minor diagonal
                let j = BOARD_COLS - j - 1; // reverse j
                let t = self.mat[i][j];
                if t != EMPTY {
                    let mut broke = false;
                    for p in 1..4 {
                        if self.mat[i + p][j - p] != t {
                            broke = true;
                            break;
                        }
                    }
                    if !broke {
                        return Some((
                            (i, j),
                            (i + 3, j - 3),
                            t
                        ));
                    }
                }
            }
        }
        None
    }
}

pub fn team_to_char(team: Team) -> char {
    match team {
        EMPTY => '_',
        TEAM_O => 'O',
        TEAM_X => 'X',
        _ => '?'
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pretty = self.mat
            .to_vec()
            .into_iter()
            .map(|row| row.to_vec().into_iter().map(team_to_char).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        write!(
            f,
            "{}",
            matrix_to_string(
                &pretty,
                None,
                true
            )
        )
    }
}
