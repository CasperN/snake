
use event_parser::TetrisEvent;

#[derive(Debug, PartialEq)]
pub enum PlayState{
  Play, Pause, Quit
}

enum Tetrimo{
  Line, S, RS, L, RL, T
}


pub struct Tetris {
  pub playing: PlayState,
  piece: Tetrimo,
  swap: Option<Tetrimo>,
  x: usize,
  y: usize,
  fall_speed: u8,
  board: Vec<bool>,
  width: usize,
  height: usize
}

impl Tetris {

  pub fn new(width :usize, height :usize) -> Tetris {
    // Construct board

    let board = vec![false; width * height];
    Tetris{ playing:PlayState::Play, piece:Tetrimo::Line, swap: None, x:0, y:0, fall_speed:0,
            board, height, width}
  }

  fn is_free(&self, x: usize, y: usize) -> bool {
    x < self.width && y < self.height && !self.board[y * self.width + x]
  }

  pub fn control(&mut self, c: TetrisEvent) {

    let (x,y) = match c {

      TetrisEvent::Left  => (self.x - 1, self.y),
      TetrisEvent::Right => (self.x + 1, self.y),
      TetrisEvent::Up    => (self.x, self.y + 1),
      TetrisEvent::Down  => (self.x, self.y - 1),
      TetrisEvent::FastFall => (self.x, 0),
      TetrisEvent::Quit  => {
        self.playing = PlayState::Quit;
        println!("{:?}", self.playing);
        return
      },
      TetrisEvent::Pause => {
        self.playing = PlayState::Pause;
        return
      }
    };

    if self.is_free(x, y) {
      self.x = x;
      self.y = y;
    }
  }
}
