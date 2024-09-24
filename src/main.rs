struct Board([i8; 5], [i8; 5], [i8; 5], [i8; 5], [i8; 5]);

impl Board {
    fn show(&self) {
        for column in self.0.iter() {
            print!("{column} ");
        }
        println!();
        for column in self.1.iter() {
            print!("{column} ");
        }
        println!();
        for column in self.2.iter() {
            print!("{column} ");
        }
        println!();
        for column in self.3.iter() {
            print!("{column} ");
        }
        println!();
        for column in self.4.iter() {
            print!("{column} ");
        }
        println!();
    }
}

fn main() {
    let board = Board([1, 1, 1, 1, 1], 
                             [2, 2, 2, 2, 2], 
                             [3, 3, 3, 3, 3], 
                             [4, 4, 4, 4, 4], 
                             [5, 5, 5, 5, 5]);

    board.show();
    println!();
    board.show();
}
