extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use game_state::{SnakeGame, Movement, SnakeState, draw_snake};
use game_state::Movement::{Up, Down, Left, Right};

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
    match self.play_state {

      Play => {
        match ui {
          UserInput::Movement(direction) => {
            if !self.moved_this_turn {
              self.game.direction = direction
            }
            self.moved_this_turn = true;
          },

          UserInput::Menu(menu) => {
            match menu {
              Pause => {
                  self.play_state = if self.play_state == Play { Pause } else { Play };
              },
              Quit => {
                self.play_state = Quit;
              }
              _ => {unreachable!();}
            }
          }
        };
      },

      _ => {info!("Not sure yet");}

    }
  }

  pub fn time_update(&mut self){
    debug!("time step");

    match self.play_state {

      Play => {
        let snake_state = self.game.move_snake();
        match snake_state {
          SnakeState::Fed => {self.score += 1;},
          SnakeState::Alive => {},
          SnakeState::Cannibaled | SnakeState::Walled => {
            info!("Game over!");
            self.play_state = End;
          }
        }
        self.moved_this_turn = false;
      },
      _ => {}
    };
  }

  pub fn draw(&self, canvas:&mut Canvas<sdl2::video::Window>){
    match self.play_state {
      Play => draw_snake(&self.game, canvas),
      Pause => {},
      Quit => {},
      End => {}
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
