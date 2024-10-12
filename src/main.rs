use rand::thread_rng;
use rand::Rng;
use rand::distributions::Uniform;

struct Board {
    content: [[u8; 5]; 5]
}

impl Board {
    fn show(&self) {
        for row in self.content.iter() {
            for column in row.iter() {
                print!("{column} ");
            }
            println!();
        }
    }

    fn generate() -> Self {
        let mut gen_board = Board {
            content: [[0; 5], [0; 5], [0; 5], [0; 5], [0; 5]]
        };

        let mut rnd_gen = thread_rng();
        let range = Uniform::new_inclusive(0, 4);
        let ships_size = Uniform::new_inclusive(1, 2);
        let ships: u8 = 2;
        
        let mut count: u8 = 0;
        while count < ships {
            let ship_location: [u8; 2] = [rnd_gen.sample(range), rnd_gen.sample(range)];
            let ship_size: u8 = rnd_gen.sample(ships_size);

            if ((gen_board.content.len() - 1) as u8 - ship_location[1]) >= ship_size {
                for ship in ship_location[1]..(ship_location[1] + ship_size) {
                    gen_board.content[ship_location[0] as usize][ship as usize] = 10 * (count + 1);
                }
                count += 1;
            }
        }

        gen_board
    }
}

fn main() {
    let board = Board::generate();
    board.show();
}
