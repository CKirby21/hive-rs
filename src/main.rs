const TOTAL_PIECES: usize = 28;

fn print_board(board: [[i32; TOTAL_PIECES]; TOTAL_PIECES]) {
    for row in board {
        for element in row {
            print!("{} ", element);
        }
        println!(); // Print a newline after each row
    }
    println!();
}

fn main() {
    println!("Hello, world!");

    let mut board: [[i32; TOTAL_PIECES]; TOTAL_PIECES] = [[0; TOTAL_PIECES]; TOTAL_PIECES];
    print_board(board);

    board[TOTAL_PIECES / 2][TOTAL_PIECES / 2] = 1;

    print_board(board);

}
