use std::fmt;

use console::Term;
use colored::Colorize;

const BOARD_SIZE: usize = 35;
const ADVANCE_KEY: char = ' ';
const BACK_KEY: char = '\t';
const UP_KEY: char = 'w';
const LEFT_KEY: char = 'a';
const DOWN_KEY: char = 's';
const RIGHT_KEY: char = 'd';

/////////////////////////////////////////////////////////////////////////

fn main() {
    // TODO make this a 3d array
    let board: [[Piece; BOARD_SIZE]; BOARD_SIZE] = [[create_piece(Bug::None, PlayerNumber::None); BOARD_SIZE]; BOARD_SIZE];
    let board_selection =  (BOARD_SIZE / 2, BOARD_SIZE / 2);
    
    let player_one = Player {
        number: PlayerNumber::One,
        hand: create_hand(PlayerNumber::One),
        hand_selection: 0,
    };
    
    let player_two_hand = create_hand(PlayerNumber::Two);
    let player_two_hand_selection = 0;

    let state = State::PlayerOneSelectFirstPiece;

    let mut game = Game {
        board,
        board_selection,
        player_one,
        player_two_hand,
        player_two_hand_selection,
        state,
    };
    
    print_game(&game);

    let stdout = Term::buffered_stdout();

    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match game.state {
                State::PlayerOneSelectFirstPiece => {
                    match character {
                        LEFT_KEY => {
                            move_left_in_player_hand(&mut game.player_one);
                        },
                        RIGHT_KEY => {
                            move_right_in_player_hand(&mut game.player_one);
                        },
                        ADVANCE_KEY => {
                            game.state = State::PlayerOneConfirmFirstPiece;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneConfirmFirstPiece => {
                    match character {
                        ADVANCE_KEY => {
                            place_player_one_selected_piece(&mut game);
                            // TODO Handle when vector is size 0
                            game.player_one.hand_selection = 0;
                            // TODO Remember if queen has been played
                            game.state = State::PlayerTwoSelectPiece;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerOneSelectFirstPiece;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneSelectPiece => {
                    match character {
                        LEFT_KEY => {
                            move_left_in_player_hand(&mut game.player_one);
                        },
                        RIGHT_KEY => {
                            move_right_in_player_hand(&mut game.player_one);
                        },
                        ADVANCE_KEY => {
                            game.state = State::PlayerOneSelectPieceLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneSelectPieceLocation => {
                    match character {
                        UP_KEY => {
                            move_up_on_the_board(&mut game);
                        },
                        LEFT_KEY => {
                            move_left_on_the_board(&mut game);
                        },
                        DOWN_KEY => {
                            move_down_on_the_board(&mut game);
                        },
                        RIGHT_KEY => {
                            move_right_on_the_board(&mut game);
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
                            place_player_one_selected_piece(&mut game);
                            // TODO Handle when vector is size 0
                            game.player_one.hand_selection = 0;
                            // TODO Remember if queen has been played
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
                        LEFT_KEY => {
                            move_left_in_player_two_hand(&mut game);
                        },
                        RIGHT_KEY => {
                            move_right_in_player_two_hand(&mut game);
                        },
                        ADVANCE_KEY => {
                            game.state = State::PlayerTwoSelectPieceLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerTwoSelectPieceLocation => {
                    match character {
                        UP_KEY => {
                            move_up_on_the_board(&mut game);
                        },
                        LEFT_KEY => {
                            move_left_on_the_board(&mut game);
                        },
                        DOWN_KEY => {
                            move_down_on_the_board(&mut game);
                        },
                        RIGHT_KEY => {
                            move_right_on_the_board(&mut game);
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
                            place_player_two_selected_piece(&mut game);
                            // TODO Handle when vector is size 0
                            game.player_two_hand_selection = 0;
                            // TODO Remember if queen has been played for player two
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

#[derive(Debug)]
struct Player {
    number: PlayerNumber,
    hand: Vec<Piece>,
    hand_selection: usize,
}

#[derive(Debug, Copy, Clone)]
enum PlayerNumber {
    None,
    One,
    Two,
}

impl fmt::Display for PlayerNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerNumber::None => write!(f, " "),
            PlayerNumber::One => write!(f, "1"),
            PlayerNumber::Two => write!(f, "2"),
        }
    }
}

/////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
struct Piece {
    bug: Bug,
    player: PlayerNumber,
}

fn print_piece(piece: Piece, selected: bool) {
    let piece_string = format!("{}{}", piece.bug, piece.player);
    let piece_string_colored = match piece.player {
        PlayerNumber::One => piece_string.blue(),
        PlayerNumber::Two => piece_string.red(),
        _ => piece_string.white(),
    };

    if selected {
        print!("|{}|", piece_string_colored);
    } else {
        print!(" {} ", piece_string_colored);
    }
}

fn create_piece(bug: Bug, player: PlayerNumber) -> Piece {
    Piece {
        bug,
        player,
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Game {
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    board_selection: (usize, usize),
    player_one: Player,
    player_two_hand: Vec<Piece>,
    player_two_hand_selection: usize,
    state: State,
}

fn move_up_on_the_board(game: &mut Game) {
    if game.board_selection.0 >= 1 {
        game.board_selection = (game.board_selection.0 - 1, game.board_selection.1);
    }
}

fn move_left_on_the_board(game: &mut Game) {
    if game.board_selection.1 >= 1 {
        game.board_selection = (game.board_selection.0, game.board_selection.1 - 1);
    }
}

fn move_down_on_the_board(game: &mut Game) {
    if game.board_selection.0 < BOARD_SIZE - 1 {
        game.board_selection = (game.board_selection.0 + 1, game.board_selection.1);
    }
}

fn move_right_on_the_board(game: &mut Game) {
    if game.board_selection.1 < BOARD_SIZE - 1 {
        game.board_selection = (game.board_selection.0, game.board_selection.1 + 1);
    }
}

fn move_left_in_player_hand(player: &mut Player) {
    if player.hand_selection > 0 {
        player.hand_selection -= 1;
    }
}

fn move_right_in_player_hand(player: &mut Player) {
    if player.hand_selection < player.hand.len() - 1 {
        player.hand_selection += 1;
    }
}

fn move_left_in_player_two_hand(game: &mut Game) {
    if game.player_two_hand_selection > 0 {
        game.player_two_hand_selection -= 1;
    }
}

fn move_right_in_player_two_hand(game: &mut Game) {
    if game.player_two_hand_selection < game.player_two_hand.len() - 1 {
        game.player_two_hand_selection += 1;
    }
}

fn place_player_one_selected_piece(game: &mut Game) {
    game.board[game.board_selection.0][game.board_selection.1] = game.player_one.hand.remove(game.player_one.hand_selection);
}

fn place_player_two_selected_piece(game: &mut Game) {
    game.board[game.board_selection.0][game.board_selection.1] = game.player_two_hand.remove(game.player_two_hand_selection);
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
    print_prompt(&game.state);
    println!();
    print_hand(&game.player_one.hand, game.player_one.hand_selection);
    print_hand(&game.player_two_hand, game.player_two_hand_selection);
    println!();
    print_board(game.board, game.board_selection);
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
enum State {
    PlayerOneSelectFirstPiece,
    PlayerOneConfirmFirstPiece,
    PlayerOneSelectPiece,
    PlayerOneSelectPieceLocation,
    PlayerOneConfirmPieceLocation,
    PlayerTwoSelectPiece,
    PlayerTwoSelectPieceLocation,
    PlayerTwoConfirmPieceLocation
}

fn print_prompt(state: &State) {
    let prompt_string = match state {
        State::PlayerOneSelectFirstPiece => "Player 1: Select a bug",
        State::PlayerOneConfirmFirstPiece => "Player 1: Are you quite sure about that?",
        State::PlayerOneSelectPiece => "Player 1: Select a bug",
        State::PlayerOneSelectPieceLocation => "Player 1: Choose a location",
        State::PlayerOneConfirmPieceLocation => "Player 1: Are you quite sure about that?",
        State::PlayerTwoSelectPiece => "Player 2: Select a bug",
        State::PlayerTwoSelectPieceLocation => "Player 2: Choose a location",
        State::PlayerTwoConfirmPieceLocation => "Player 2: Are you quite sure about that?",
    };
    let player_ones_turn = match state {
        State::PlayerOneSelectFirstPiece => true,
        State::PlayerOneConfirmFirstPiece => true,
        State::PlayerOneSelectPiece => true,
        State::PlayerOneSelectPieceLocation => true,
        State::PlayerOneConfirmPieceLocation => true,
        State::PlayerTwoSelectPiece => false,
        State::PlayerTwoSelectPieceLocation => false,
        State::PlayerTwoConfirmPieceLocation => false,
    };
    let prompt_string_colored = if player_ones_turn { prompt_string.blue() } else { prompt_string.red() };
    println!("                                                  {}", prompt_string_colored);
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

fn print_board(board: [[Piece; BOARD_SIZE]; BOARD_SIZE], selection: (usize, usize)) {
    for (i, row) in board.iter().enumerate() {
        for (j, piece) in row.iter().enumerate() {
            let selected: bool = i == selection.0 && j == selection.1; 
            print_piece(*piece, selected);
        }
        println!();
    }
}

fn print_hand(hand: &[Piece], selection: usize) {
    print!("                                                  ");
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

fn create_hand(player: PlayerNumber) -> Vec<Piece> {
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
