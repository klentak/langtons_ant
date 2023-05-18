fn main() {
  pub const SIZE_X: usize = 512;
  pub const SIZE_Y: usize = 100;
  pub const DEFAULT_SIGN: &str = "0";
  
  pub enum Direction {
    Up,
    Right ,
    Down,
    Left
  }

  pub struct Ant {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub sign: String
  }
  
  impl Ant {
    fn turn_right(&mut self) {
      self.direction = match self.direction {
        Direction::Up =>  Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
      };
    }
    
    fn turn_left(&mut self) {
      self.direction = match self.direction {
        Direction::Up =>  Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
      };
    }
    
    fn walk(&mut self) {
      match self.direction {
        Direction::Up =>  { self.y += 1 },
        Direction::Left => { self.x -= 1 },
        Direction::Down => { self.y -= 1 },
        Direction::Right => { self.x += 1 },
      };
    }
  }

  pub struct Board<'a> {
    pub array: [[&'a str; SIZE_X]; SIZE_Y]
  }

  struct Simulation<'a> {
    ants: [Ant; 1],
    board: Board<'a>
  }

  impl Simulation<'_> {
    fn run(& mut self, rounds: usize) {
      for _ in 0..rounds {
        self.print();
        &self.round();
      }
    }
    
    fn round(& mut self) {
      for mut ant in &self.ants {
        &self.make_move(&mut ant);
      }
    }
    
    fn make_move(&mut self, ant: &mut Ant) {
      if self.board.array[ant.y as usize][ant.x as usize] == DEFAULT_SIGN {
        &ant.turn_right();
        &ant.walk();
        self.board.array[ant.y as usize][ant.x as usize] = ant.sign.as_str();
      } else {
        &ant.turn_left();
        &ant.walk();
        self.board.array[ant.y as usize][ant.x as usize] = DEFAULT_SIGN;
      }
    }
    
    fn print(&self) {
      println!("{}", &self.board.array.len());
      for row in &self.board.array {
        for pixel in row {
          print!("{}", pixel)
        }
        println!("");
      }
    }
  }
  
  let mut simulation = Simulation {
    ants: [Ant {
      x: 0,
      y: 0,
      direction: Direction::Up,
      sign: String::from("\u{001b}[31mO\u{001b}[0m")
    }],
    board: Board {
      array: [[DEFAULT_SIGN; SIZE_X]; SIZE_Y],
    },
  };
  
  simulation.run(1);
}
