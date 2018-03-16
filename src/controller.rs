extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use game_state::{SnakeGame, Movement, SnakeState};
use game_state::Movement::{Up, Down, Left, Right};

use game_render::draw_snake;

use self::PlayState::{Play, Pause, Quit, End};


#[derive(Debug, PartialEq, Clone)]
pub enum PlayState{
  Play, Pause, Quit, End
}


#[derive(Debug, Clone)]
pub enum UserInput{
  Movement(Movement), Menu(PlayState)
}


pub struct Controller {
  pub play_state: PlayState,
  pub speed: u32,
  pub score: u32,
  moved_this_turn: bool,
  game: SnakeGame,
}

impl Controller {
  pub fn new() -> Controller {
    Controller {play_state: Play, speed:2, score:0, game:SnakeGame::new(10,10),
                moved_this_turn:false}
  }



  pub fn control_update(&mut self, ui: UserInput){

    debug!("Control event: {:?}", ui);
    match ui {

      UserInput::Movement(direction) => {
        if !self.moved_this_turn {
          self.game.direction = direction
        }
        self.moved_this_turn = true;
      },

      UserInput::Menu(menu) => {
        self.play_state = menu;
      }
    };
  }

  pub fn time_update(&mut self){
    debug!("time step")

    match self.play_state {

      Play => {
        let snake_state = self.game.move_snake();
        match snake_state {
          SnakeState::Fed => {self.score += 1;},
          SnakeState::Alive => {},
          SnakeState::Cannibaled | SnakeState::Walled => {self.play_state = Quit;}
        }
        self.moved_this_turn = false;
      },

      _ => {}
    };
  }

  pub fn draw(&self, canvas:&mut Canvas<sdl2::video::Window>){
    match self.play_state {
      _ => draw_snake(&self.game, canvas) // TODO Draw menu when in that state
    };
  }
}


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
