use std::collections::LinkedList;

extern crate rand;
use self::rand::{ThreadRng, Rng};

use self::Movement::{Up, Down, Left, Right};


#[derive(Debug, Clone, PartialEq)]
pub enum Movement{
  Up, Down, Left, Right
}

pub enum SnakeState {
  Walled, Cannibaled, Alive, Fed
}


pub struct SnakeGame {
  xy: (u32,u32),
  pub direction: Movement,
  pub food_xy: (u32, u32),
  pub width: u32,
  pub height: u32,
  pub snake: LinkedList<(u32, u32)>,
  rng: ThreadRng
}

impl SnakeGame {

  pub fn new(width :u32, height :u32) -> SnakeGame {
    // Construct board
    let mut rng = rand::thread_rng();

    let food_xy = (rng.gen::<u32>() % width, rng.gen::<u32>() % height);
    let xy      = (rng.gen::<u32>() % width, rng.gen::<u32>() % height);
    let mut snake = LinkedList::new();

    snake.push_front(xy);

    let mut g = SnakeGame{xy, direction:Right, food_xy, width, height, snake, rng};
    g.place_food();
    g
  }


  fn place_food(&mut self){
    let (mut x,mut y) = self.food_xy;
    while self.snake.contains(&(x, y)) {
      x = self.rng.gen::<u32>() % self.width;
      y = self.rng.gen::<u32>() % self.height;
    }
    self.food_xy = (x,y);
  }

  pub fn move_snake(&mut self) -> SnakeState {

    // Canvas is flipped so can't go up from y == 0
    let (mut x, mut y) = self.xy;
    if (self.direction == Left && x == 0) || (self.direction == Right && x == self.width - 1) ||
       (self.direction == Up && y == 0) || (self.direction == Down && y == self.height -1) {
        println!("You hit the wall");
        return SnakeState::Walled;
    }

    match self.direction {
      Left => x -= 1,
      Right => x += 1,
      Up => y -= 1,
      Down => y += 1
    };
    self.xy = (x,y);

    if self.snake.contains(&self.xy) {
      println!("Oh no you ate yourself.");
      return SnakeState::Cannibaled;
    }
    self.snake.push_front(self.xy);

    if self.xy != self.food_xy {
      self.snake.pop_back();
      SnakeState::Alive
    } else {
      self.place_food();
      SnakeState::Fed
    }
  }
}
