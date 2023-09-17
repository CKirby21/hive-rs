use std::fmt;

use console::Term;
use colored::Colorize;

const BOARD_SIZE: usize = 35;
const ADVANCE_KEY: char = 'e';
const BACK_KEY: char = 'q';
const UP_KEY: char = 'w';
const LEFT_KEY: char = 'a';
const DOWN_KEY: char = 's';
const RIGHT_KEY: char = 'd';

/////////////////////////////////////////////////////////////////////////

fn main() {
    // TODO make this a 3d array
    let board: [[Piece; BOARD_SIZE]; BOARD_SIZE] = [[create_piece(Bug::None, PlayerNumber::None); BOARD_SIZE]; BOARD_SIZE];
    let board_selection =  (BOARD_SIZE / 2, BOARD_SIZE / 2);
    let player_one = Player::new(PlayerNumber::One);
    let player_two = Player::new(PlayerNumber::Two);
    let state = State::PlayerOneSelectFirstPiece;
    let old_piece_location = (0, 0); // FIXME I dont like this variable

    let mut game = Game {
        board,
        board_selection,
        player_one,
        player_two,
        state,
        old_piece_location,
    };
    
    print_game(&game);

    let stdout = Term::buffered_stdout();

    'game_loop: loop {
        print_game(&game);
        if let Ok(character) = stdout.read_char() {
            match game.state {
                State::PlayerOneSelectFirstPiece => {
                    match character {
                        LEFT_KEY => {
                            game.player_one.move_left_in_hand();
                        },
                        RIGHT_KEY => {
                            game.player_one.move_right_in_hand();
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
                    assert!(!game.player_one.hand.is_empty());
                    match character {
                        LEFT_KEY => {
                            game.player_one.move_left_in_hand();
                        },
                        RIGHT_KEY => {
                            game.player_one.move_right_in_hand();
                        },
                        ADVANCE_KEY => {
                            game.state = State::PlayerOneSelectPlacingLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneSelectPlacingLocation => {
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
                            game.state = State::PlayerOneConfirmPlacingLocation;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerOneSelectPiece;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneConfirmPlacingLocation => {
                    match character {
                        ADVANCE_KEY => {
                            place_player_one_selected_piece(&mut game);
                            // TODO Handle when vector is size 0
                            game.player_one.hand_selection = 0;
                            // TODO Remember if queen has been played
                            game.state = State::PlayerTwoSelectPiece;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerOneSelectPlacingLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneSelectPieceOnBoard => {
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
                            game.old_piece_location = game.board_selection;
                            game.state = State::PlayerOneSelectMovingLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneSelectMovingLocation => {
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
                            game.state = State::PlayerOneConfirmMovingLocation;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerOneSelectPieceOnBoard;
                        },
                        _ => continue,
                    }
                },
                State::PlayerOneConfirmMovingLocation => {
                    match character {
                        ADVANCE_KEY => {
                            move_piece(&mut game);
                            game.state = State::PlayerOneSelectPiece;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerOneSelectMovingLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerTwoSelectPiece => {
                    assert!(!game.player_two.hand.is_empty());
                    match character {
                        LEFT_KEY => {
                            game.player_two.move_left_in_hand();
                        },
                        RIGHT_KEY => {
                            game.player_two.move_right_in_hand();
                        },
                        ADVANCE_KEY => {
                            game.state = State::PlayerTwoSelectPlacingLocation;
                        },
                        _ => continue,
                    }
                },
                State::PlayerTwoSelectPlacingLocation => {
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
                            game.state = State::PlayerTwoConfirmPlacingLocation;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerTwoSelectPiece;
                        },
                        _ => continue,
                    }
                },
                State::PlayerTwoConfirmPlacingLocation => {
                    match character {
                        ADVANCE_KEY => {
                            place_player_two_selected_piece(&mut game);
                            // TODO Handle when vector is size 0
                            game.player_two.hand_selection = 0;
                            // TODO Remember if queen has been played for player two
                            game.state = State::PlayerOneSelectPiece;
                        },
                        BACK_KEY => {
                            game.state = State::PlayerTwoSelectPlacingLocation;
                        },
                        _ => continue,
                    }
                },
            }
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
enum PlayerNumber {
    None,
    One,
    Two,
}

#[derive(Debug)]
struct Player {
    number: PlayerNumber,
    hand: Vec<Piece>,
    hand_selection: usize,
}

impl Player {
    fn new(number: PlayerNumber) -> Self {
        Player {
            number,
            hand: create_hand(number),
            hand_selection: 0,
        }
    }

    fn move_left_in_hand(&mut self) {
        if self.hand_selection > 0 {
            self.hand_selection -= 1;
        }
    }
    fn move_right_in_hand(&mut self) {
        if self.hand_selection < self.hand.len() - 1 {
            self.hand_selection += 1;
        }
    }
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
    player_two: Player,
    state: State,
    old_piece_location: (usize, usize), // FIXME move somewhere else
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

fn place_player_one_selected_piece(game: &mut Game) {
    game.board[game.board_selection.0][game.board_selection.1] = game.player_one.hand.remove(game.player_one.hand_selection);
}

fn place_player_two_selected_piece(game: &mut Game) {
    game.board[game.board_selection.0][game.board_selection.1] = game.player_two.hand.remove(game.player_one.hand_selection);
}

fn move_piece(game: &mut Game) {
    // TODO Check that the selection can be moved to by the piece
    game.board[game.board_selection.0][game.board_selection.1] = game.board[game.old_piece_location.0][game.old_piece_location.1];
    game.board[game.old_piece_location.0][game.old_piece_location.1] = create_piece(Bug::None, PlayerNumber::None);
}

fn print_game(game: &Game) {
    let mut player_one_show_selection = false;
    let mut player_two_show_selection = false;
    let mut board_show_selection = false;
    match game.state {
        State::PlayerOneSelectFirstPiece => player_one_show_selection = true,
        State::PlayerOneConfirmFirstPiece => board_show_selection = true,
        State::PlayerOneSelectPiece => player_one_show_selection = true,
        State::PlayerOneSelectPlacingLocation => board_show_selection = true,
        State::PlayerOneConfirmPlacingLocation => board_show_selection = true,
        State::PlayerOneSelectPieceOnBoard => board_show_selection = true,
        State::PlayerOneSelectMovingLocation => board_show_selection = true,
        State::PlayerOneConfirmMovingLocation => board_show_selection = true,
        State::PlayerTwoSelectPiece => player_two_show_selection = true,
        State::PlayerTwoSelectPlacingLocation => board_show_selection = true,
        State::PlayerTwoConfirmPlacingLocation => board_show_selection = true,
    };
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
    print_hand(&game.player_one.hand, game.player_one.hand_selection, player_one_show_selection);
    print_hand(&game.player_two.hand, game.player_two.hand_selection, player_two_show_selection);
    println!();
    print_board(game.board, game.board_selection, board_show_selection);
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
enum State {
    PlayerOneSelectFirstPiece,
    PlayerOneConfirmFirstPiece,
    PlayerOneSelectPiece,
    PlayerOneSelectPlacingLocation,
    PlayerOneConfirmPlacingLocation,
    PlayerOneSelectPieceOnBoard,
    PlayerOneSelectMovingLocation,
    PlayerOneConfirmMovingLocation,
    PlayerTwoSelectPiece,
    PlayerTwoSelectPlacingLocation,
    PlayerTwoConfirmPlacingLocation,
}

fn print_prompt(state: &State) {
    let prompt_string = match state {
        State::PlayerOneSelectFirstPiece => "Player 1: Select a bug",
        State::PlayerOneConfirmFirstPiece => "Player 1: Are you quite sure about that?",
        State::PlayerOneSelectPiece => "Player 1: Select a bug",
        State::PlayerOneSelectPlacingLocation => "Player 1: Choose a location",
        State::PlayerOneConfirmPlacingLocation => "Player 1: Are you quite sure about that?",
        State::PlayerOneSelectPieceOnBoard => "Player 1: Select a bug",
        State::PlayerOneSelectMovingLocation => "Player 1: Choose a location",
        State::PlayerOneConfirmMovingLocation => "Player 1: Are you quite sure about that?",
        State::PlayerTwoSelectPiece => "Player 2: Select a bug",
        State::PlayerTwoSelectPlacingLocation => "Player 2: Choose a location",
        State::PlayerTwoConfirmPlacingLocation => "Player 2: Are you quite sure about that?",
    };
    let player_ones_turn = match state {
        State::PlayerOneSelectFirstPiece => true,
        State::PlayerOneConfirmFirstPiece => true,
        State::PlayerOneSelectPiece => true,
        State::PlayerOneSelectPlacingLocation => true,
        State::PlayerOneConfirmPlacingLocation => true,
        State::PlayerOneSelectPieceOnBoard => true,
        State::PlayerOneSelectMovingLocation => true,
        State::PlayerOneConfirmMovingLocation => true,
        State::PlayerTwoSelectPiece => false,
        State::PlayerTwoSelectPlacingLocation => false,
        State::PlayerTwoConfirmPlacingLocation => false,
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

fn print_board(board: [[Piece; BOARD_SIZE]; BOARD_SIZE], selection: (usize, usize), show_selection: bool) {
    for (i, row) in board.iter().enumerate() {
        for (j, piece) in row.iter().enumerate() {
            let mut selected: bool = i == selection.0 && j == selection.1;
            selected &= show_selection;
            print_piece(*piece, selected);
        }
        println!();
    }
}

fn print_hand(hand: &[Piece], selection: usize, show_selection: bool) {
    print!("                                                  ");
    for (i, piece) in hand.iter().enumerate() {
        let mut selected = i == selection;
        selected &= show_selection;
        print_piece(*piece, selected);
    }
    println!();
}

fn create_hand(player: PlayerNumber) -> Vec<Piece> {
    let hand: Vec<Piece> = vec![
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
