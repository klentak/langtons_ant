use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
  pub const SIZE_Y: usize = 300;
  pub const SIZE_X: usize = 100;
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
        Direction::Up =>  { self.y = (self.y + SIZE_X - 1) % SIZE_X; },
        Direction::Left => { self.x = (self.x + SIZE_Y - 1) % SIZE_Y; },
        Direction::Down => { self.y = (self.y + 1) % SIZE_X; },
        Direction::Right => { self.x = (self.x + 1) % SIZE_Y; },
      };
    }
  }

  pub struct Board {
    pub array: [[&'static str; SIZE_Y]; SIZE_X],
  }

  pub struct Simulation {
    ants: Vec<Ant>,
    board: Board,
    itteration: i64,
  }

  impl Simulation {
    fn new(ants: Vec<Ant>, board: Board, itteration: i64) -> Simulation {
      Simulation { ants, board, itteration }
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

    fn print(&mut self) {
      if self.itteration % 100 == 0 {
        for row in &self.board.array {
          for pixel in row {
            print!("{}", pixel);
          }
          println!("");
        }

        // adjust for terminal
        thread::sleep(Duration::new(0, 90000000));
        let _ = Command::new("clear").status();
      }
      self.itteration = self.itteration + 1;
    }
  }

  let mut simulation = Simulation::new(
    vec![
      Ant::new(
        50,
        50,
        Direction::Up,
        "\u{001b}[31mO\u{001b}[0m",
      ),
      Ant::new(
        100,
        50,
        Direction::Up,
        "\u{001b}[37mO\u{001b}[0m",
      )
    ],
    Board {
      array: [[DEFAULT_SIGN; SIZE_Y]; SIZE_X],
    },
    1
  );
  
  simulation.run(120000);
}
