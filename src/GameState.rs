
mod EventParser;
use EventParser::TetrisEvent


enum PlayState{
  Play, Pause, Quit
}

enum Tetrimo{
  Line, S, RS, L, RL, T
}


struct Tetris {
  playing: PlayState,
  piece: Tetrimo,
  swap: Option<Tetrimo>,
  x: u8,
  y: u8,
  fall_speed: u8,
  board: Vec<bool>,
  width: usize,
  height: usize
}

impl Tetris {
  fn new(width :usize, height :usize) -> Tetris {

    // Construct board
    let mut grid_raw = vec![0; width * height];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let board: &mut [&mut [_]] = grid_base.as_mut_slice();

    Tetris{playing:Play, piece:Line, swap: None, x:0, y:0, fall_speed:0, board, height,
           width}
  }

  fn is_occupied(&self, x: usize, y: usize) -> bool {

    return (0 <= x < )& & !board[x][y]
  }

  fn control(&mut self, c: TetrisEvent) {
    
    let (x,y) = match c {
      TetrisEvent::Left  => { (self.x - 1, self.y) },
      TetrisEvent::Right => { (self.x + 1, self.y) },
      TetrisEvent::Up    => { (self.x, self.y + 1) },
      TetrisEvent::Down  => { (self.x, self.y - 1) },
    }
    if self.is_occupied(x, y) {
      self.x = x;
      self.y = y;
    }
  }
}
