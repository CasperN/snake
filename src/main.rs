


extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

mod EventParser;



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

  let mut rect_col = Color::RGB(0, 0, 0);


  'game_loop: loop {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for event in event_pump.poll_iter() {
      // IO handler
      // println!("{:?}",event );
      if let Event::Quit{..} = event {
        break 'game_loop
      }
      let control = EventParser::parse_event(event);
      if let Some(c) = control {
        println!("{:?}", c);
      }
    }

    canvas.set_draw_color(rect_col);
    canvas.fill_rect(Rect::new(200, 200, 400, 500));


    canvas.present();
    std::thread::sleep(Duration::from_millis(1));
  }
  println!("Thanks for playing.");
}
