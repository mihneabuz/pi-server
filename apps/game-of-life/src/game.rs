use std::cmp::min;

use bitvec::{bitvec, vec::BitVec};

#[derive(Clone, Debug)]
pub struct GameOfLife {
    height: usize,
    width: usize,
    curr: BitVec,
    next: BitVec,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let len = height * width;
        Self {
            height,
            width,
            curr: bitvec![0; len],
            next: bitvec![0; len],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) -> Option<bool> {
        if !self.in_bounds(x, y) {
            return None;
        }

        let index = y * self.width + x;
        let old = *self.curr.get(index)?;
        self.curr.set(index, alive);

        Some(old)
    }

    pub fn step_iter(&mut self) -> Updates {
        Updates {
            game: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Update {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
}

pub struct Updates<'a> {
    game: &'a mut GameOfLife,
    x: usize,
    y: usize,
}

impl Iterator for Updates<'_> {
    type Item = Update;

    fn next(&mut self) -> Option<Self::Item> {
        while self.x < self.game.width() {
            while self.y < self.game.height() {
                let xr = self.x.saturating_sub(1)..=min(self.x + 1, self.game.width - 1);
                let yr = self.y.saturating_sub(1)..=min(self.y + 1, self.game.height - 1);

                let alive = self.game.curr[self.y * self.game.width + self.x];

                let mut neighs = 0;
                for nx in xr {
                    for ny in yr.clone() {
                        if self.x == nx && self.y == ny {
                            continue;
                        }

                        neighs += self.game.curr[ny * self.game.width + nx] as u8;
                    }
                }

                let lives = match (alive, neighs) {
                    (true, 0..=1) => false,
                    (true, 2..=3) => true,
                    (true, 4..) => false,
                    (false, 3) => true,
                    _ => alive,
                };

                self.game.next.set(self.y * self.game.width + self.x, lives);

                let update = (alive != lives).then_some(Update {
                    x: self.x,
                    y: self.y,
                    alive: lives,
                });

                self.y += 1;

                if update.is_some() {
                    return update;
                }
            }

            self.x += 1;
            self.y = 0;
        }

        std::mem::swap(&mut self.game.curr, &mut self.game.next);

        None
    }
}
