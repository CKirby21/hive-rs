use std::fmt;

use console::Term;

// const TOTAL_PIECES: usize = 28;
const TOTAL_PIECES: usize = 22;
const ADVANCE_KEY: char = ' ';
const BACK_KEY: char = '\t';

/////////////////////////////////////////////////////////////////////////

fn main() {
    // TODO make this a 3d array
    let mut board: [[Piece; TOTAL_PIECES]; TOTAL_PIECES] = [[create_piece(Bug::None, Player::None); TOTAL_PIECES]; TOTAL_PIECES];
    let mut board_selection =  (TOTAL_PIECES / 2, TOTAL_PIECES / 2);
    
    let mut player_one_hand = create_hand(Player::One);
    let mut player_one_hand_selection = 0;
    
    let mut player_two_hand = create_hand(Player::Two);
    let mut player_two_hand_selection = 0;

    let mut state = State::PlayerOneSelectPiece;

    let mut game = Game {
        board,
        board_selection,
        player_one_hand,
        player_one_hand_selection,
        player_two_hand,
        player_two_hand_selection,
        state,
    };
    
    print_game(&game);

    let stdout = Term::buffered_stdout();
    const INDICES_TO_MOVE: usize = 1;

    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match game.state {
                State::PlayerOneSelectPiece => {
                    match character {
                        'a' => {
                            if game.player_one_hand_selection > 0 {
                                game.player_one_hand_selection -= 1;
                            }
                        },
                        'd' => {
                            if game.player_one_hand_selection < game.player_one_hand.len() - 1 {
                                game.player_one_hand_selection += 1;
                            }
                        },
                        ADVANCE_KEY => {
                            game.state = State::PlayerOneSelectPieceLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneSelectPieceLocation => {
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
                        ADVANCE_KEY => {
                            game.state = State::PlayerOneConfirmPieceLocation;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerOneSelectPiece;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneConfirmPieceLocation => {
                    match character {
                        ADVANCE_KEY => {
                            game.board[game.board_selection.0][game.board_selection.1] = game.player_one_hand.remove(game.player_one_hand_selection);
                            // TODO Handle when vector is size 0
                            game.player_one_hand_selection = 0;
                            game.state = State::PlayerTwoSelectPiece;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerOneSelectPieceLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerTwoSelectPiece => {
                    match character {
                        'a' => {
                            if game.player_two_hand_selection > 0 {
                                game.player_two_hand_selection -= 1;
                            }
                        },
                        'd' => {
                            if game.player_two_hand_selection < game.player_two_hand.len() - 1 {
                                game.player_two_hand_selection += 1;
                            }
                        },
                        ADVANCE_KEY => {
                            game.state = State::PlayerTwoSelectPieceLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerTwoSelectPieceLocation => {
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
                        ADVANCE_KEY => {
                            game.state = State::PlayerTwoConfirmPieceLocation;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerTwoSelectPiece;
                        },
                        _ => continue,
                    }
                },
                State::PlayerTwoConfirmPieceLocation => {
                    match character {
                        ADVANCE_KEY => {
                            game.board[game.board_selection.0][game.board_selection.1] = game.player_two_hand.remove(game.player_two_hand_selection);
                            // TODO Handle when vector is size 0
                            game.player_two_hand_selection = 0;
                            game.state = State::PlayerOneSelectPiece;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerTwoSelectPieceLocation;
                        },
                        _ => continue,
                    }
                },
            }
            print_game(&game);
        }
    }
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
        print!("|{}-{}|", piece.bug, piece.player);
    } else {
        print!(" {}-{} ", piece.bug, piece.player);
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
    state: State,
}

fn print_game(game: &Game) {
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!("                                           {}", game.state);
    println!();
    print_hand(&game.player_one_hand, game.player_one_hand_selection);
    print_hand(&game.player_two_hand, game.player_two_hand_selection);
    println!();
    print_board(game.board, game.board_selection);
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
enum State {
    PlayerOneSelectPiece,
    PlayerOneSelectPieceLocation,
    PlayerOneConfirmPieceLocation,
    PlayerTwoSelectPiece,
    PlayerTwoSelectPieceLocation,
    PlayerTwoConfirmPieceLocation
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::PlayerOneSelectPiece => write!(f, "Player 1: Select a bug"),
            State::PlayerOneSelectPieceLocation => write!(f, "Player 1: Choose a location"),
            State::PlayerOneConfirmPieceLocation => write!(f, "Player 1: Are you quite sure about that?"),
            State::PlayerTwoSelectPiece => write!(f, "Player 2: Select a bug"),
            State::PlayerTwoSelectPieceLocation => write!(f, "Player 2: Choose a location"),
            State::PlayerTwoConfirmPieceLocation => write!(f, "Player 2: Are you quite sure about that?"),
        }
    }
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

fn print_hand(hand: &[Piece], selection: usize) {
    print!("                              ");
    for (i, piece) in hand.iter().enumerate() {
        let selected = i == selection;
        print_piece(*piece, selected);
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
