use std::fmt;

// const TOTAL_PIECES: usize = 28;
const TOTAL_PIECES: usize = 22;

/////////////////////////////////////////////////////////////////////////

fn main() {
    // TODO make this a 3d array
    let mut board: [[Piece; TOTAL_PIECES]; TOTAL_PIECES] = [[create_piece(Bug::None, Player::None); TOTAL_PIECES]; TOTAL_PIECES];
    let mut selection =  (TOTAL_PIECES / 2, TOTAL_PIECES / 2);
    print_board(board, selection);

    let mut player_one_hand = create_hand(Player::One);
    print_hand(player_one_hand);

    let mut player_two_hand = create_hand(Player::Two);
    print_hand(player_two_hand);

    // player_one_hand[0] = 
    // board[TOTAL_PIECES / 2][TOTAL_PIECES / 2] = ;

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

fn print_board(board: [[Piece; TOTAL_PIECES]; TOTAL_PIECES], selection: (usize, usize)) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let selected: bool = i == selection.0 && j == selection.1; 
            print_piece(board[i][j], selected);
        }
        println!();
    }
    println!();
}

fn print_hand(hand: Vec<Piece>) {
    for piece in hand {
        print_piece(piece, false);
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
