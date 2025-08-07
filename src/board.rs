use rand::thread_rng;
use rand::Rng;
use rand::distributions::Uniform;

pub struct Board {
    content: [[u8; 5]; 5]
}

impl Board {
    /// Generate a empty board.
    /// 
    /// The generated board hasn't ships.
    pub fn new() -> Self {
        let empty_board = Board {
            content: [[0; 5], [0; 5], [0; 5], [0; 5], [0; 5]]
        };
        return empty_board;
    }

    /// Show all board in the std output.
    pub fn show(&self) {
        for row in self.content.iter() {
            for column in row.iter() {
                print!("{column} ");
            }
            println!();
        }
    }

    /// Initializes the board with 2 ships randomly generated.
    pub fn init_rnd(&mut self) {
        let max_ships: u8 = 2;
        let mut count: u8 = 0;
        while count < max_ships {
            let position = Board::generate_ship(5, 1);
            self.content[position[0] as usize][position[1] as usize] = 1;
            count += 1;
        }
    }

    /// Generate the coordinates for the ship.
    /// 
    /// Actually, generates a ship with size 1.
    /// 
    /// + `board_size`: The length of side for board.
    /// + `size`: The size of ship.
    fn generate_ship(board_side: u8, size: u8) -> [u8; 2] {
        let mut rnd_gen = thread_rng();
        let range = Uniform::new(0, board_side);

        let x = rnd_gen.sample(range);
        let y = rnd_gen.sample(range);

        [x, y]
    }
}