use std::fmt;

use console::Term;
use colored::Colorize;

const BOARD_SIZE: usize = 35;
const FIRST_LOCATION: (usize, usize) = (BOARD_SIZE / 2, BOARD_SIZE / 2);
const ADVANCE_KEY: char = 'e';
const BACK_KEY: char = 'q';
const UP_KEY: char = 'w';
const LEFT_KEY: char = 'a';
const DOWN_KEY: char = 's';
const RIGHT_KEY: char = 'd';

/////////////////////////////////////////////////////////////////////////

fn main() {

    let mut game = Game::new();
    game.print();

    let stdout = Term::buffered_stdout();

    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            game.find_placeable_locations();
            match game.state {
                State::SelectPieceInHand => {
                    assert!(!game.player_with_turn.hand.is_empty());
                    match character {
                        LEFT_KEY => {
                            game.player_with_turn.move_left_in_hand();
                        },
                        RIGHT_KEY => {
                            game.player_with_turn.move_right_in_hand();
                        },
                        ADVANCE_KEY => {
                            game.state = State::SelectPlacingLocation;
                            game.board_selection = game.placeable_location_vec[0]
                        },
                        _ => continue,
                    }
                },
                State::SelectPlacingLocation => {
                    match character {
                        LEFT_KEY => {
                            game.move_placeable_location_cursor(MoveDirection::Previous);
                        },
                        RIGHT_KEY => {
                            game.move_placeable_location_cursor(MoveDirection::Next);
                        },
                        ADVANCE_KEY => {
                            game.state = State::ConfirmPlacingLocation;
                        },
                        BACK_KEY => {
                            game.state = State::SelectPieceInHand;
                        },
                        _ => continue,
                    }
                },
                State::ConfirmPlacingLocation => {
                    match character {
                        ADVANCE_KEY => {
                            game.place_selected_piece();
                            game.advance_turn();
                            game.state = State::SelectPieceInHand;
                        },
                        BACK_KEY => {
                            game.state = State::SelectPlacingLocation;
                        },
                        _ => continue,
                    }
                },
            }
            game.print();
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
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
enum MoveDirection {
    Next,
    Previous,
}

#[derive(Debug)]
struct Game {
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    board_selection: (usize, usize),
    player_with_turn: Player,
    player_without_turn: Player,
    state: State,
    old_piece_location: (usize, usize), // FIXME move somewhere else
    placeable_location_selection: usize,
    placeable_location_vec: Vec<(usize, usize)>,
}

impl Game {
    
    fn new() -> Self {
        // TODO make this a 3d array
        let board: [[Piece; BOARD_SIZE]; BOARD_SIZE] = [[create_piece(Bug::None, PlayerNumber::None); BOARD_SIZE]; BOARD_SIZE];
        let player_with_turn = Player::new(PlayerNumber::One);
        let player_without_turn = Player::new(PlayerNumber::Two);
        let board_selection = FIRST_LOCATION;
        let state = State::SelectPieceInHand;
        let old_piece_location = (0, 0); // FIXME I dont like this variable
        let placeable_location_selection: usize = 0;
        let placeable_location_vec = vec![(BOARD_SIZE / 2, BOARD_SIZE / 2)];
        
        Game {
            board,
            board_selection,
            player_with_turn,
            player_without_turn,
            state,
            old_piece_location,
            placeable_location_selection,
            placeable_location_vec,
        }
    }

    fn move_placeable_location_cursor(&mut self, move_direction: MoveDirection) {

        // TODO Handle when player has nowhere to place
        assert!(!self.placeable_location_vec.is_empty());
        
        match move_direction {
            MoveDirection::Next => {
                if self.placeable_location_selection >= self.placeable_location_vec.len() - 1 {
                    self.placeable_location_selection = 0;
                }
                else {
                    self.placeable_location_selection += 1;
                }
            }
            MoveDirection::Previous => {
                if self.placeable_location_selection == 0 {
                    self.placeable_location_selection = self.placeable_location_vec.len() - 1
                }
                else {
                    self.placeable_location_selection -= 1;
                }
            }
        }
    
        // FIXME update board selection better
        self.board_selection = self.placeable_location_vec[self.placeable_location_selection];
    }

    // Not sure I like this function
    fn place_selected_piece(&mut self) {
        self.board[self.board_selection.0][self.board_selection.1] = self.player_with_turn.hand.remove(self.player_with_turn.hand_selection);
        self.player_with_turn.hand_selection = 0;
        // TODO Remember if queen has been played
    }

    fn advance_turn(&mut self) {
        let temp_player = self.player_with_turn.clone();
        self.player_with_turn = self.player_without_turn.clone();
        self.player_without_turn = temp_player;
    }

    fn find_placeable_locations(&mut self) {

        let mut placeable_location_vec: Vec<(usize, usize)> = vec![];
    
        let mut locations_occupied = 0;
    
        for (i, row) in self.board.iter().enumerate() {
            for (j, _piece) in row.iter().enumerate() {
                // FIXME Ignores the indexes in the corners of the board
                if i < 2 || self.board.len() - 2 <= i || j < 2 || self.board.len() - 2 <= j {
                    continue;
                }
    
                let current_location_occupied = self.board[i][j].player != PlayerNumber::None;
                if current_location_occupied {
                    locations_occupied += 1;
                    continue;
                }
    
                let neighboring_piece_vec = vec![
                    self.board[i - 2][j].player,     // North
                    self.board[i - 1][j - 1].player, // Northwest
                    self.board[i - 1][j + 1].player, // Northeast
                    self.board[i + 2][j].player,     // South
                    self.board[i + 1][j - 1].player, // Southwest
                    self.board[i + 1][j + 1].player, // Southeast
                ];
    
                let mut neighboring_piece_from_another_player = false;
                let mut neighboring_piece_from_same_player = false;
                for neighbor in neighboring_piece_vec {
                    if neighbor == self.player_with_turn.number {
                        neighboring_piece_from_same_player = true;
                    }
                    else if neighbor == PlayerNumber::None {
                        // Do nothing
                    }
                    else {
                        neighboring_piece_from_another_player = true;
                    }
                }
    
                if neighboring_piece_from_another_player {
                    continue;
                }
                if !neighboring_piece_from_same_player {
                    continue;
                }
    
                placeable_location_vec.push((i, j));
            }
        }
    
        // Handles the first turn for each player where they have no existing
        // pieces to play off of
        if locations_occupied <= 1 {
            if self.player_with_turn.number == PlayerNumber::One {
                placeable_location_vec.push(FIRST_LOCATION);
            }
            else if self.player_with_turn.number == PlayerNumber::Two {
                placeable_location_vec.push((FIRST_LOCATION.0 - 2, FIRST_LOCATION.1));     // North
                placeable_location_vec.push((FIRST_LOCATION.0 - 1, FIRST_LOCATION.1 - 1)); // Northwest
                placeable_location_vec.push((FIRST_LOCATION.0 - 1, FIRST_LOCATION.1 + 1)); // Northeast
                placeable_location_vec.push((FIRST_LOCATION.0 + 2, FIRST_LOCATION.1));     // South
                placeable_location_vec.push((FIRST_LOCATION.0 + 1, FIRST_LOCATION.1 - 1)); // Southwest
                placeable_location_vec.push((FIRST_LOCATION.0 + 1, FIRST_LOCATION.1 + 1)); // Southeast
            } else {
                panic!();
            }
        }
        self.placeable_location_vec = placeable_location_vec;
    }
    
    fn print(&mut self) {
        let mut board_show_selection = false;
        if self.state == State::SelectPlacingLocation || self.state == State::ConfirmPlacingLocation {
            board_show_selection = true;
        }
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
        print_prompt(&self.state, self.player_with_turn.number);
        println!();
        print_hand(&self.player_with_turn.hand, self.player_with_turn.hand_selection, true);
        print_hand(&self.player_without_turn.hand, self.player_without_turn.hand_selection, false);
        println!();
        print_board(self.board, self.board_selection, board_show_selection);
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    // DecideToPlaceOrMove,
    SelectPieceInHand,
    SelectPlacingLocation,
    ConfirmPlacingLocation,
    // SelectPieceOnBoard,
    // SelectMovingLocation,
    // ConfirmMovingLocation,
}

fn print_prompt(state: &State, player_turn: PlayerNumber) {
    let prompt_string = match state {
        State::SelectPieceInHand => format!("Player {}: Select a bug", player_turn),
        State::SelectPlacingLocation => format!("Player {}: Choose a location", player_turn),
        State::ConfirmPlacingLocation => format!("Player {}: Are you quite sure about that?", player_turn),
    };
    let prompt_string_colored = if player_turn == PlayerNumber::One { prompt_string.blue() } else { prompt_string.red() };
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