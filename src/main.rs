
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate sdl2;
use sdl2::event::Event;
use std::time::Duration;

mod controller;
mod game_state;
use controller::{parse_event, PlayState, Controller};


fn main() {
  pretty_env_logger::init();

  info!("Initializing sdl and canvas");
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
                              .position_centered()
                              .build()
                              .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();

  let mut game_controller = Controller::new();

  info!("Beginning game loop");
  'game_loop: loop {

    for event in event_pump.poll_iter() {

      if let Event::Quit{..} = event {
        break 'game_loop
      }

      let control = parse_event(event);

      if let Some(c) = control {
        game_controller.control_update(c);
      }
    }

    game_controller.time_update();

    if game_controller.play_state == PlayState::Quit {
      break 'game_loop
    }

    game_controller.draw(&mut canvas);

    canvas.present();
    std::thread::sleep(Duration::from_millis((1000 / game_controller.speed) as u64));
  }

  println!("Thanks for playing.");
}
