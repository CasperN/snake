use std::collections::LinkedList;

extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

extern crate rand;
use self::rand::{ThreadRng, Rng};


#[derive(Debug, Clone, PartialEq)]
pub enum Movement{
  Up, Down, Left, Right
}

#[derive(Debug, PartialEq, Clone)]
pub enum PlayState{
  Play, Pause, Quit, End
}

#[derive(Debug, Clone)]
pub enum UserInput{
  Movement(Movement), Menu(PlayState)
}

use self::Movement::{Up, Down, Left, Right};
use self::PlayState::{Play, Pause, Quit, End};

/* --------------------------------------------------------------------------------------------- */

pub fn parse_event(event: Event) -> Option<UserInput> {
  match event {
    Event::KeyDown{keycode: Some(key), ..} => {
      match key {
        Keycode::W => Some(UserInput::Movement(Up)), // Up and Down swapped for rendering
        Keycode::S => Some(UserInput::Movement(Down)),
        Keycode::A => Some(UserInput::Movement(Left)),
        Keycode::D => Some(UserInput::Movement(Right)),
        Keycode::P => Some(UserInput::Menu(Pause)),
        Keycode::Escape => Some(UserInput::Menu(Quit)),
        _ => None
      }
    },
    _ => None
  }
}

pub struct Game {
  pub play_state: PlayState,
  xy: (u32,u32),
  direction: Movement,
  pub food_xy: (u32, u32),
  pub speed: u32,
  pub width: u32,
  pub height: u32,
  pub snake: LinkedList<(u32, u32)>,
  pub score: u32,
  rng: ThreadRng
}

impl Game {

  pub fn new(width :u32, height :u32) -> Game {
    // Construct board
    let mut rng = rand::thread_rng();

    let food_xy = (rng.gen::<u32>() % width, rng.gen::<u32>() % height);
    let xy      = (rng.gen::<u32>() % width, rng.gen::<u32>() % height);
    let mut snake = LinkedList::new();

    snake.push_front(xy);

    let mut g = Game{play_state:Play, xy, direction:Right,
                     food_xy, speed:1, score:0, width, height, snake, rng};
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

  fn move_snake(&mut self){

    // Canvas is flipped so can't go up from y == 0
    let (mut x, mut y) = self.xy;
    if (self.direction == Left && x == 0) || (self.direction == Right && x == self.width - 1) ||
       (self.direction == Up && y == 0) || (self.direction == Down && y == self.height -1) {
        println!("You hit the wall");
        self.play_state = Quit;
        return;
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
      self.play_state = Quit;
      return;
    }

    self.snake.push_front(self.xy);
    if self.xy != self.food_xy {
      self.snake.pop_back();
    } else {
      self.place_food();
      self.score += 1;
    }

  }

  pub fn control_update(&mut self, ui: UserInput){
    println!("{:?}", ui);
    match ui {
      UserInput::Movement(direction) => { self.direction = direction },
      UserInput::Menu(menu) => { self.play_state = menu; }
    };
  }

  pub fn time_update(&mut self){
    self.move_snake();
  }

}
