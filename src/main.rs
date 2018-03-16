

extern crate sdl2;
// use sdl2::pixels::Color;
use sdl2::event::Event;
// use sdl2::render::Canvas;
use std::time::Duration;

mod game_state;
mod game_render;
use game_state::{Game, PlayState, parse_event};


fn main() {
  println!("Hello, world!");

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
                              .position_centered()
                              .build()
                              .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();

  let mut snake = Game::new(10,10);

  'game_loop: loop {

    for event in event_pump.poll_iter() {
      // IO handler
      // println!("{:?}",event );
      if let Event::Quit{..} = event {
        break 'game_loop
      }

      let control = parse_event(event);
      if let Some(c) = control {
        snake.control_update(c);
      }
    }

    snake.time_update();

    if snake.play_state == PlayState::Quit {
      break 'game_loop
    }


    game_render::draw(&snake, &mut canvas);

    canvas.present();
    std::thread::sleep(Duration::from_millis((1000 / snake.speed) as u64));
  }
  println!("Thanks for playing.");
}
