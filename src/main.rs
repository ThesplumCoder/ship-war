mod board;

use board::Board;

fn main() {
    let mut board = Board::new();
    board.init_rnd();
    board.show();
}
