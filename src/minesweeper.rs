use std::collections::HashSet;
use std::fmt::{Display, Write};

use crate::random::random_range;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = (col, row);

                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        f.write_str("💣 ")?;
                    } else if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟦 ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighboring_mines(pos);

                    if mine_count > 0 {
                        write!(f, " {} ", mine_count)?;
                    } else {
                        f.write_str("⬜ ")?;
                    }
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }

                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }

    fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item=Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1))
                .map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.open_fields.contains(&position) {
            let mine_count = self.neighboring_mines(position);

            let flag_count =
                self.iter_neighbors(position)
                    .filter(|neighbor|
                        self.flagged_fields
                            .contains(neighbor)
                    )
                    .count();

            if mine_count == flag_count as u8 {
                for neighbor in self.iter_neighbors(position) {
                    if !self.flagged_fields.contains(&neighbor) && !self.open_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }

            return None;
        }

        if self.lost || self.flagged_fields.contains(&position) { return None; }

        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighboring_mines(position);

            if mine_count == 0 {
                for neighbor in self.iter_neighbors(position) {
                    if !self.open_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }
            Some(OpenResult::NoMine(mine_count))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        minesweeper::{Minesweeper, Position},
        random::random_range,
    };

    #[test]
    fn test_new() {
        let width = random_range(1, 10);
        let height = random_range(1, 10);
        let mine_count = random_range(1, width * height);
        let minesweeper = Minesweeper::new(width, height, mine_count);

        assert_eq!(minesweeper.width, width);
        assert_eq!(minesweeper.height, height);
        assert_eq!(minesweeper.mines.len(), mine_count);
    }

    #[test]
    fn check_mine_number() {
        let width = random_range(1, 20);
        let height = random_range(1, 20);
        let mine_count: usize = random_range(0, width / 2);

        let ms = Minesweeper::new(width, height, mine_count);

        assert_eq!(ms.mines.len(), mine_count);
    }

    #[test]
    fn check_open() {
        let width = random_range(1, 20);
        let height = random_range(1, 20);
        let mine_count: usize = random_range(0, width / 2);

        let opened_position: Position = (random_range(0, width), random_range(0, height));

        let mut ms = Minesweeper::new(width, height, mine_count);

        ms.open(opened_position);

        if ms.mines.contains(&opened_position) {
            assert!(ms.lost);
        } else {
            assert!(ms.open_fields.contains(&opened_position));
        }
    }

    #[test]
    fn check_flag_exists() {
        let width = random_range(1, 20);
        let height = random_range(1, 20);
        let mine_count: usize = random_range(0, width);

        let flag_pos: Position = (random_range(0, width), random_range(0, height));

        let mut ms = Minesweeper::new(width, height, mine_count);

        ms.toggle_flag(flag_pos);

        assert_eq!(ms.flagged_fields.is_empty(), false);
        assert!(ms.flagged_fields.contains(&flag_pos));
    }
}