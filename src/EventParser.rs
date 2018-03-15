

extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

#[derive(Debug, Clone)]
pub enum TetrisEvent {
  Up,
  Down,
  Left,
  Right,
  FastFall,
  Pause,
  Quit
}


pub fn parse_event(event: Event) -> Option<TetrisEvent> {
  match event {
    Event::KeyDown{keycode: Some(key), ..} => {
      match key {
        Keycode::W => Some(TetrisEvent::Up),
        Keycode::S => Some(TetrisEvent::Down),
        Keycode::A => Some(TetrisEvent::Left),
        Keycode::D => Some(TetrisEvent::Right),
        Keycode::Space => Some(TetrisEvent::FastFall),
        Keycode::P => Some(TetrisEvent::Pause),
        Keycode::Escape => Some(TetrisEvent::Quit),
        _ => None
      }
    },
    _ => None
  }
}
