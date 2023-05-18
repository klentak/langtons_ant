use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
  pub const SIZE_X: usize = 512;
  pub const SIZE_Y: usize = 100;
  pub const DEFAULT_SIGN: &'static str = "0";

  #[derive(Clone)]
  pub enum Direction {
    Up,
    Right,
    Down,
    Left
  }

  #[derive(Clone)]
  pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub sign: &'static str
  }

  impl Ant {
    fn new(x: usize, y: usize, direction: Direction, sign: &'static str) -> Ant {
      Ant { x, y, direction, sign }
    }

    fn turn_right(&mut self) {
      self.direction = match self.direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
      };
    }

    fn turn_left(&mut self) {
      self.direction = match self.direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
      };
    }

    fn walk(&mut self) {
      match self.direction {
        Direction::Up =>  { self.y -= 1 },
        Direction::Left => { self.x -= 1 },
        Direction::Down => { self.y += 1 },
        Direction::Right => { self.x += 1 },
      };
    }
  }

  pub struct Board {
    pub array: [[&'static str; SIZE_X]; SIZE_Y],
  }

  pub struct Simulation {
    ants: Vec<Ant>,
    board: Board,
  }

  impl Simulation {
    fn new(ants: Vec<Ant>, board: Board) -> Simulation {
      Simulation { ants, board }
    }

    fn run(&mut self, rounds: usize) {
      for _ in 0..rounds {
        self.print();
        self.round();
      }
    }

    fn round(&mut self) {
      let ants_copy = self.ants.clone();
      self.ants.clear();

      for ant in ants_copy {
        let moved_ant = self.make_move(&ant);
        self.ants.push(moved_ant);
      }
    }

    fn make_move(&mut self, ant: &Ant) -> Ant {
      let mut new_ant = ant.clone();

      if self.board.array[ant.y][ant.x] == DEFAULT_SIGN {
        new_ant.turn_right();
        self.board.array[new_ant.y][new_ant.x] = new_ant.sign;
        new_ant.walk();
      } else {
        new_ant.turn_left();
        self.board.array[new_ant.y][new_ant.x] = DEFAULT_SIGN;
        new_ant.walk();
      }

      new_ant
    }

    fn print(&self) {
      for row in &self.board.array {
        for pixel in row {
          print!("{}", pixel);
        }
        println!("");
      }

      thread::sleep(Duration::new(0, 100_000_00));
      let _ = Command::new("clear").status();
    }
  }

  let mut simulation = Simulation::new(
    vec![Ant::new(
      50,
      50,
      Direction::Up,
      "\u{001b}[31mO\u{001b}[0m",
    )],
    Board {
      array: [[DEFAULT_SIGN; SIZE_X]; SIZE_Y],
    },
  );

  
  simulation.run(1000);
}
