
use game_state::Game;
use sdl2;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;


pub fn draw_snake(game: & Game, canvas:&mut Canvas<Window>){
  // Background color
  let background_col = Color::RGB(0, 0, 0);
  let board_col = Color::RGB(255, 255, 255);
  let snake_col = Color::RGB(100, 200, 100);
  let head_col = Color::RGB(50, 100, 50);
  let food_col = Color::RGB(250,50, 50);

  canvas.set_draw_color(background_col);
  canvas.clear();

  let board = Rect::new(100, 100, 500, 500);
  let sh = board.height() / game.height;
  let sw = board.width() / game.width;

  canvas.set_draw_color(board_col);
  canvas.fill_rect(board);

  {
    let mut square = | x, y, col | // #Currying
      draw_rect(x as i32 * sw as i32 + 100, y as i32 * sh as i32 + 100, sw, sh, col, canvas);

    let mut col = head_col;
    for &(x,y) in game.snake.iter() {
      square(x,y, col);
      col = snake_col;
    }

    let (fx, fy) = game.food_xy;
    square(fx, fy, food_col);
  }

  canvas.present();
}


pub fn draw_menu(canvas: &mut Canvas<Window>){

  let pause_col = Color::RGB(100,100,100);
  let font_col = Color::RGB(255, 0, 0);
  canvas.set_draw_color(pause_col);
  canvas.clear();

  // let ttf_context = sdl2::ttf::init().unwrap();
  // font.set_style(sdl2::ttf::STYLE_BOLD);
  // let surface = font.render("Hello Rust!");//.blended(font_col).unwrap();

  canvas.present();
}

pub fn draw_end(canvas: &mut Canvas<Window>){
  let end_col = Color::RGB(50,50,50);
  let font_col = Color::RGB(255, 0, 0);
  canvas.set_draw_color(end_col);
  canvas.clear();
  canvas.present();
}


fn draw_rect(x:i32, y:i32, h:u32, w:u32, color:Color, canvas:&mut Canvas<Window>){

  let r = Rect::new(x, y, h, w);
  canvas.set_draw_color(color);
  canvas.fill_rect(r);
}
