use std::fmt;

use console::Term;

// const TOTAL_PIECES: usize = 28;
const TOTAL_PIECES: usize = 22;

/////////////////////////////////////////////////////////////////////////

fn main() {
    // TODO make this a 3d array
    let mut board: [[Piece; TOTAL_PIECES]; TOTAL_PIECES] = [[create_piece(Bug::None, Player::None); TOTAL_PIECES]; TOTAL_PIECES];
    let mut board_selection =  (TOTAL_PIECES / 2, TOTAL_PIECES / 2);
    
    let mut player_one_hand = create_hand(Player::One);
    let mut player_one_hand_selection = 0;
    
    let mut player_two_hand = create_hand(Player::Two);
    let mut player_two_hand_selection = 0;

    let mut game = Game {
        board,
        board_selection,
        player_one_hand,
        player_one_hand_selection,
        player_two_hand,
        player_two_hand_selection,
    };
    
    print_game(&game);

    board[game.board_selection.0][game.board_selection.1] = game.player_one_hand.remove(0);

    print_game(&game);

    game.board_selection = (game.board_selection.0 + 2, game.board_selection.1);

    print_game(&game);

    board[game.board_selection.0][game.board_selection.1] = game.player_two_hand.remove(0);

    print_game(&game);

    let stdout = Term::buffered_stdout();
    const INDICES_TO_MOVE: usize = 2;

    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'w' => {
                    if game.board_selection.0 >= INDICES_TO_MOVE {
                        game.board_selection = (game.board_selection.0 - INDICES_TO_MOVE, game.board_selection.1);
                    }
                },
                'a' => {
                    if game.board_selection.1 >= INDICES_TO_MOVE {
                        game.board_selection = (game.board_selection.0, game.board_selection.1 - INDICES_TO_MOVE);
                    }
                },
                's' => {
                    if game.board_selection.0 < TOTAL_PIECES - INDICES_TO_MOVE {
                        game.board_selection = (game.board_selection.0 + INDICES_TO_MOVE, game.board_selection.1);
                    }
                },
                'd' => {
                    if game.board_selection.1 < TOTAL_PIECES - INDICES_TO_MOVE {
                        game.board_selection = (game.board_selection.0, game.board_selection.1 + INDICES_TO_MOVE);
                    }
                },
                _ => break 'game_loop,
            }
            print_game(&game);
        }
    }
    // print_board(board);

}

/////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
enum Bug {
    None,
    Grasshopper,
    Spider,
    Ant,
    Queen,
    Beetle,
    // Mosquito,
    // Pillbug,
    // Ladybug,
}

impl fmt::Display for Bug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bug::None => write!(f, " "),
            Bug::Grasshopper => write!(f, "G"),
            Bug::Spider => write!(f, "S"),
            Bug::Ant => write!(f, "A"),
            Bug::Queen => write!(f, "Q"),
            Bug::Beetle => write!(f, "B"),
        }
    }
}

/////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
enum Player {
    None,
    One,
    Two,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::None => write!(f, " "),
            Player::One => write!(f, "1"),
            Player::Two => write!(f, "2"),
        }
    }
}

/////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
struct Piece {
    bug: Bug,
    player: Player,
}

fn print_piece(piece: Piece, selected: bool) {
    if selected {
        print!(" |{}-{}| ", piece.bug, piece.player);
    } else {
        print!("  {}-{}  ", piece.bug, piece.player);
    }
}

fn create_piece(bug: Bug, player: Player) -> Piece {
    Piece {
        bug,
        player,
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Game {
    board: [[Piece; TOTAL_PIECES]; TOTAL_PIECES],
    board_selection: (usize, usize),
    player_one_hand: Vec<Piece>,
    player_one_hand_selection: usize,
    player_two_hand: Vec<Piece>,
    player_two_hand_selection: usize,
}

fn print_game(game: &Game) {
    print_board(game.board, game.board_selection);
    print_hand(&game.player_one_hand);
    print_hand(&game.player_two_hand);
}

////////////////////////////////////////////////////////////////////////

fn clamp(min: i32, value: i32, max: i32) -> i32 {
    if value < min {
        min
    }
    else if value > max {
        max
    }
    else {
        value
    }
}

fn print_board(board: [[Piece; TOTAL_PIECES]; TOTAL_PIECES], selection: (usize, usize)) {
    for (i, row) in board.iter().enumerate() {
        for (j, piece) in row.iter().enumerate() {
            let selected: bool = i == selection.0 && j == selection.1; 
            print_piece(*piece, selected);
        }
        println!();
    }
    println!();
}

fn print_hand(hand: &Vec<Piece>) {
    for piece in hand {
        print_piece(*piece, false);
    }
    println!();
}

// fn check_for_piece_in_hand(hand: [Piece; PLAYER_PIECES], desired_piece: Piece) -> bool {
//     for piece in hand {
//         if piece.bug == desired_piece.bug {
//             return true;
//         }
//     }
//     return false;
// }

fn create_hand(player: Player) -> Vec<Piece> {
    let mut hand: Vec<Piece> = vec![
        create_piece(Bug::Grasshopper, player),
        create_piece(Bug::Grasshopper, player),
        create_piece(Bug::Grasshopper, player),
        create_piece(Bug::Spider, player),
        create_piece(Bug::Spider, player),
        create_piece(Bug::Ant, player),
        create_piece(Bug::Ant, player),
        create_piece(Bug::Ant, player),
        create_piece(Bug::Queen, player),
        create_piece(Bug::Beetle, player),
        create_piece(Bug::Beetle, player),
    ];
    hand
}
