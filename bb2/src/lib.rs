extern crate rand;

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    direction: Direction,
    distance: i32,
}

impl Move {
    pub fn new(direction: Direction, distance: i32) -> Move {
        Move {
            direction,
            distance,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::UP,
            1 => Direction::DOWN,
            2 => Direction::RIGHT,
            _ => Direction::LEFT,
        }
    }
}

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        Move {
            direction: rng.gen(),
            distance: rng.gen_range(1, 101),
        }
    }
}
