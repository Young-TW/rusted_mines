use rusted_mines::board::Board;

fn main() {
    let mut game_board = Board::new(10, 10, 10);
    loop {
        game_board.init();
        game_board.play();
    }
}
