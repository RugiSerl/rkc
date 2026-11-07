use std::{fmt::Display, sync::mpsc::Iter};

use rand::{Rng, seq::IndexedRandom};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy)]
enum Color {
    White,
    Yellow,
    Green,
    Blue,
    Orange,
    Red,
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Move {
    U,UR,U2,
    D,DR,D2,
    L,LR,L2,
    R,RR,R2,
    F,FR,F2,
    B,BR,B2,
}

impl Move {
    fn generate_scramble<R>(size: usize, rng: &mut R) -> Vec<Self>
    where
        R: Rng + ?Sized
    {
        let possible_moves = Move::iter().collect::<Vec<Move>>();
        possible_moves.choose_iter(&mut rand::rng()).expect("No move specified").take(size).map(|m| *m).collect()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Color::White    => "\x1b[47m",
            Color::Yellow   => "\x1b[43m",
            Color::Green    => "\x1b[42m",
            Color::Blue     => "\x1b[44m",
            Color::Orange   => "\x1b[48;5;208m",
            Color::Red      => "\x1b[41m",
        };
        write!(f, "{char}  {char}")
    }
}
#[derive(Clone, Copy)]
struct Face {
    colors: [Color; 9]
}

impl Face {
    fn r(&self) -> Face {
        Self {
            colors: [
                self.colors[6], self.colors[3], self.colors[0],
                self.colors[7], self.colors[4], self.colors[1],
                self.colors[8], self.colors[5], self.colors[2],
            ]
        }
    }
    fn rr(&self) -> Face {
        self.r().r()
    }
    fn rrr(&self) -> Face {
        self.r().r().r()
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}\x1b[0m\n{}{}{}\x1b[0m\n{}{}{}\x1b[0m",
            self.colors[0], self.colors[1], self.colors[2],
            self.colors[3], self.colors[4], self.colors[5],
            self.colors[6], self.colors[7], self.colors[8])
    }
}

#[derive(Clone, Copy)]
struct RubiksCube {
    up: Face,
    down: Face,
    left: Face,
    right: Face,
    front: Face,
    back: Face,
}

impl RubiksCube {
    fn new() -> Self {
        Self {
            up: Face { colors: [Color::Yellow; 9] },
            down: Face { colors: [Color::White; 9] },
            left: Face { colors: [Color::Red; 9] },
            right: Face { colors: [Color::Orange; 9] },
            front: Face { colors: [Color::Green; 9] },
            back: Face { colors: [Color::Blue; 9] },
        }
    }

    fn up(&self) -> Self {
        Self {
            up: self.up.r(), // simply rotated
            down: self.down, // doesn't change
            left: Face {
                colors: [
                    self.front.colors[0], self.front.colors[1], self.front.colors[2],
                    self.left.colors[3], self.left.colors[4], self.left.colors[5],
                    self.left.colors[6], self.left.colors[7], self.left.colors[8],
                ]
            },
            front: Face {
                colors: [
                    self.right.colors[0], self.right.colors[1], self.right.colors[2],
                    self.front.colors[3], self.front.colors[4], self.front.colors[5],
                    self.front.colors[6], self.front.colors[7], self.front.colors[8],
                ]
            },
            right: Face {
                colors: [
                    self.back.colors[0], self.back.colors[1], self.back.colors[2],
                    self.right.colors[3], self.right.colors[4], self.right.colors[5],
                    self.right.colors[6], self.right.colors[7], self.right.colors[8],
                ]
            },
            back: Face {
                colors: [
                    self.left.colors[0], self.left.colors[1], self.left.colors[2],
                    self.back.colors[3], self.back.colors[4], self.back.colors[5],
                    self.back.colors[6], self.back.colors[7], self.back.colors[8],
                ]
            }
        }
    }

    fn front(&self) -> Self {
        self.rotate_x().up().rotate_x_r()
    }

    fn back(&self) -> Self {
        self.rotate_x_r().up().rotate_x()
    }

    fn down(&self) -> Self {
        self.rotate_x().rotate_x().up().rotate_x().rotate_x()
    }

    fn left(&self) -> Self {
        self.rotate_y_r().front().rotate_y()
    }

    fn right(&self) -> Self {
        self.rotate_y().front().rotate_y_r()
    }

    fn rotate_x(&self) -> Self {
        Self {
            down: self.back.rr(),
            front: self.down,
            up: self.front,
            back: self.up.rr(),
            right: self.right.r(),
            left: self.left.rrr(),
        }
    }

    fn rotate_x_r(&self) -> Self {
        self.rotate_x().rotate_x().rotate_x()
    }

    fn rotate_y(&self) -> Self {
        Self {
            left: self.front,
            front: self.right,
            right: self.back,
            back: self.left,
            up: self.up.r(),
            down: self.down.rrr(),
        }
    }

    fn rotate_y_r(&self) -> Self {
        self.rotate_y().rotate_y().rotate_y()
    }

    fn execute_move(&self, m: Move) -> Self {
        match m {
            Move::B => self.back(),     Move::BR => self.back().back().back(),      Move::B2 => self.back().back(),
            Move::F => self.front(),    Move::FR => self.front().front().front(),   Move::F2 => self.front().front(),
            Move::U => self.up(),       Move::UR => self.up().up().up(),            Move::U2 => self.up().up(),
            Move::D => self.down(),     Move::DR => self.down().down().down(),      Move::D2 => self.down().down(),
            Move::L => self.left(),     Move::LR => self.left().left().left(),      Move::L2 => self.left().left(),
            Move::R => self.right(),    Move::RR => self.right().right().right(),   Move::R2 => self.right().right(),
        }
    }

    fn execute_moves(&self, moves: impl IntoIterator<Item = Move>) -> Self {
        moves.into_iter().fold(*self, |r, m| r.execute_move(m))
    }

    fn scramble(&self) -> Self {

        Self::new()
    }
}

impl Display for RubiksCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "
      {}{}{}\x1b[0m
      {}{}{}\x1b[0m
      {}{}{}\x1b[0m
{}{}{}{}{}{}{}{}{}{}{}{}\x1b[0m
{}{}{}{}{}{}{}{}{}{}{}{}\x1b[0m
{}{}{}{}{}{}{}{}{}{}{}{}\x1b[0m
      {}{}{}\x1b[0m
      {}{}{}\x1b[0m
      {}{}{}\x1b[0m",
                                                                            self.up.colors[0],    self.up.colors[1],    self.up.colors[2],
                                                                            self.up.colors[3],    self.up.colors[4],    self.up.colors[5],
                                                                            self.up.colors[6],    self.up.colors[7],    self.up.colors[8],
            self.left.colors[0], self.left.colors[1], self.left.colors[2],  self.front.colors[0], self.front.colors[1], self.front.colors[2],  self.right.colors[0], self.right.colors[1], self.right.colors[2],  self.back.colors[0], self.back.colors[1], self.back.colors[2],
            self.left.colors[3], self.left.colors[4], self.left.colors[5],  self.front.colors[3], self.front.colors[4], self.front.colors[5],  self.right.colors[3], self.right.colors[4], self.right.colors[5],  self.back.colors[3], self.back.colors[4], self.back.colors[5],
            self.left.colors[6], self.left.colors[7], self.left.colors[8],  self.front.colors[6], self.front.colors[7], self.front.colors[8],  self.right.colors[6], self.right.colors[7], self.right.colors[8],  self.back.colors[6], self.back.colors[7], self.back.colors[8],
                                                                            self.down.colors[0],  self.down.colors[1],  self.down.colors[2],
                                                                            self.down.colors[3],  self.down.colors[4],  self.down.colors[5],
                                                                            self.down.colors[6],  self.down.colors[7],  self.down.colors[8],

        )
    }
}

fn random_cube() {
    let moves = Move::generate_scramble(16, &mut rand::rng());
    let cube = RubiksCube::new().execute_moves(moves.clone());
    print!("Here's the random moves : ");
    for m in moves {
        print!("{:?} ", m);
    }
    println!("{}", cube);
}

fn main() {
    random_cube();
}
