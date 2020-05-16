use std::ops::AddAssign;

const SIZE: usize = 8;

type Universe = [[u8; SIZE]; SIZE];

#[derive(Debug)]
pub struct GameOfLife {
    state: Universe,
    last: Universe,
    ticks: u32,
}

impl GameOfLife {
    pub fn new() -> Self {
        return GameOfLife { state: [[0; 8]; 8], last: [[0; 8]; 8], ticks: 0 };
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x >= SIZE || y >= SIZE { return; }
        self.state[x][y] = 1;
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        if x >= SIZE || y >= SIZE { return false; }
        self.state[x][y] == 1
    }
    pub fn is_dead(&self) -> bool {
        self.state.iter().all(|line| line.iter().all(|cell| cell == &0u8))
    }

    pub fn tick(&mut self) {
        self.last = self.state;

        for x in 0..self.last.len() {
            for y in 0..self.last[x].len() {
                self.state[x][y] = Self::will_be_alive(self.last, x, y)
            }
        }
        self.ticks += 1;
    }

    fn will_be_alive(state: Universe, x: usize, y: usize) -> u8 {
        let mut alive_neighbors = 0;
        for i in SIZE - 1..=SIZE + 1 {
            for j in SIZE - 1..=SIZE + 1 {
                if i == SIZE && j == SIZE { // don' count self
                    continue;
                }
                // turns into a wrapping universe
                let x_i = (x + i) % SIZE;
                let y_j = (y + j) % SIZE;
                alive_neighbors += state[x_i][y_j];
            }
        };
        return match (state[x][y], alive_neighbors) {
            (0, 3) => 1,
            (1, 2) | (1, 3) => 1,
            _ => 0,
        };
    }

    pub fn pretty_state(&self) -> String {
        let mut s = "".to_string();
        for line in &self.state {
            s.add_assign(&format!("{:?}\n", line))
        }
        s
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_tick_stable() {
        let mut game = GameOfLife::new();
        // stable square
        game.set_alive(1, 1);
        game.set_alive(1, 2);
        game.set_alive(2, 1);
        game.set_alive(2, 2);
        println!("{}----------", game.pretty_state());
        game.tick();
        assert_eq!(game.state, game.last)
    }

    #[test]
    fn test_tick_flip() {
        let mut game = GameOfLife::new();
        // a line flips between horizontal and vertical
        game.set_alive(1, 1);
        game.set_alive(1, 2);
        game.set_alive(1, 3);
        println!("{}----------", game.pretty_state());
        game.tick();
        assert_ne!(game.state, game.last);
        println!("{}----------", game.pretty_state());
        game.tick();
        assert_ne!(game.state, game.last);
        assert!(game.is_alive(1, 1) && game.is_alive(1, 2) && game.is_alive(1, 3));
    }

    #[test]
    fn test_tick_cross() {
        let mut game = GameOfLife::new();
        // a line flips between horizontal and vertical
        game.set_alive(1, 1);
        game.set_alive(1, 2);
        game.set_alive(1, 3);
        game.set_alive(2, 2);
        game.set_alive(0, 2);
        while !game.is_dead() {
            game.tick();
            println!("{}----------", game.pretty_state());
        }
    }

    #[test]
    fn test_tick_glider() {
        let mut game = GameOfLife::new();
        // a line flips between horizontal and vertical
        game.set_alive(0, 1);
        game.set_alive(1, 2);
        game.set_alive(2, 0);
        game.set_alive(2, 1);
        game.set_alive(2, 2);
        println!("{}----------", game.pretty_state());
        while !game.is_dead() && game.ticks < 4 {
            game.tick();
            println!("{}----------", game.pretty_state());
        }
        assert!(
            game.is_alive(1, 2) && game.is_alive(2, 3) &&
                game.is_alive(3, 1) && game.is_alive(3, 2) && game.is_alive(3, 3),
            "glider moved down in (1,1) direction"
        )
    }

    #[test]
    fn test_tick_dead_in_2_ticks() {
        let mut game = GameOfLife::new();
        assert!(game.is_dead());
        // "v" -> 2 high "I" -> gone
        game.set_alive(1, 1);
        game.set_alive(1, 3);
        game.set_alive(2, 2);
        assert!(!game.is_dead());
        println!("{}----------", game.pretty_state());
        game.tick();
        println!("{}----------", game.pretty_state());
        assert!(!game.is_dead());
        game.tick();
        println!("{}----------", game.pretty_state());
        assert!(game.is_dead())
    }
}
