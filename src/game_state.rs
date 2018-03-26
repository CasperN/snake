use std;
use std::collections::LinkedList;
use std::time::Duration;

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
  changed_direction: bool,
  rng: ThreadRng
}

impl Game {



  pub fn new(width :u32, height :u32, speed:u32, ) -> Game {
    // Construct board
    let mut rng = rand::thread_rng();
    let mut snake = LinkedList::new();
    let mut g = Game{play_state:Play, xy:(0,0), direction:Right, changed_direction:false,
                     food_xy:(0,0), speed, score:0, width, height, snake, rng};

    g.xy = g.random_square();
    g.set_dir();
    g.snake.push_front(g.xy);
    g.place_food();
    g
  }

  fn random_square(&mut self) -> (u32, u32) {
    (self.rng.gen::<u32>() % self.width, self.rng.gen::<u32>() % self.height)
  }

  fn set_dir(&mut self) {
    let (x,y) = self.xy;
    let dx = self.width as i32 - x as i32;
    let dy = self.height as i32 - y as i32;

    self.direction = if dx.abs() > dy.abs() {
      if dx > 0 { Right } else { Left }
    } else {
      if dy > 0 { Down } else { Up }
    };
  }


  fn place_food(&mut self){
    self.food_xy = self.random_square();
    while self.snake.contains(&self.food_xy){
      self.food_xy = self.random_square();
    }
  }


  fn move_snake(&mut self){

    // Canvas is flipped so can't go up from y == 0
    let (mut x, mut y) = self.xy;

    let walled = match self.direction {
      Left => x == 0,
      Right => x == self.width - 1,
      Up => y == 0,
      Down => y == self.height -1
    };
    
    if walled {
        println!("You hit the wall");
        self.play_state = End;
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
      self.play_state = End;
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

    if self.play_state == End {
      self.xy = self.random_square();
      self.snake = LinkedList::new();
      self.snake.push_front(self.xy);
      self.set_dir();
      self.place_food();
      self.score = 0;
      self.play_state = Play;
    }


    match ui {

      UserInput::Movement(direction) => {
        if self.play_state == Play && !self.changed_direction {
          let not_180 = match self.direction {
            Left => direction != Right,
            Right => direction != Left,
            Up => direction != Down,
            Down => direction != Up
          };
          if not_180 {
            self.direction = direction;
            self.changed_direction = true;
          }
        }
      },

      UserInput::Menu(menu) => {
        self.play_state = match menu {
          Pause => if self.play_state == Pause { Play } else { Pause },
          Quit => Quit,
          _ => {unreachable!()}
        }
      }
    };
  }


  pub fn time_update(&mut self){
    if self.play_state == Play {
      self.move_snake();
      self.changed_direction = false;
      std::thread::sleep(Duration::from_millis((1000 / self.speed) as u64));
    }
  }
}
