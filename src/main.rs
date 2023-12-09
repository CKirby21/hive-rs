use std::fmt;

use console::Term;
use colored::Colorize;

const BOARD_SIZE: usize = 35;
const FIRST_LOCATION: (usize, usize) = (BOARD_SIZE / 2, BOARD_SIZE / 2);
const ADVANCE_KEY: char = 'e';
const BACK_KEY: char = 'q';
// const UP_KEY: char = 'w';
const LEFT_KEY: char = 'a';
// const DOWN_KEY: char = 's';
const RIGHT_KEY: char = 'd';

/////////////////////////////////////////////////////////////////////////

fn main() {

    let mut game = Game::new();
    game.print();
    game.update();

    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(character) = stdout.read_char() {
            match game.state {
                State::SelectPiece => {
                    match character {
                        LEFT_KEY => {
                            game.move_piece_cursor(MoveDirection::Previous);
                        },
                        RIGHT_KEY => {
                            game.move_piece_cursor(MoveDirection::Next);
                        },
                        ADVANCE_KEY => {
                            game.state = State::SelectPlacingLocation;
                        },
                        _ => continue,
                    }
                },
                State::SelectPlacingLocation => {
                    match character {
                        LEFT_KEY => {
                            game.move_location_cursor(MoveDirection::Previous);
                        },
                        RIGHT_KEY => {
                            game.move_location_cursor(MoveDirection::Next);
                        },
                        ADVANCE_KEY => {
                            game.state = State::ConfirmPlacingLocation;
                        },
                        BACK_KEY => {
                            game.state = State::SelectPiece;
                        },
                        _ => continue,
                    }
                },
                State::ConfirmPlacingLocation => {
                    match character {
                        ADVANCE_KEY => {
                            game.place_selected_piece();
                            game.advance_turn();
                            game.clear_selections();
                            game.state = State::SelectPiece;
                        },
                        BACK_KEY => {
                            game.state = State::SelectPlacingLocation;
                        },
                        _ => continue,
                    }
                },
            }
            game.update();
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
}

impl Player {
    fn new(number: PlayerNumber) -> Self {
        Player {
            number,
            hand: create_hand(number),
        }
    }

    fn get_hand_selection_vec(&self) -> Vec<Selection>{
        
        let mut hand_selection_vec: Vec<Selection> = vec![];

        for (i, _piece) in self.hand.iter().enumerate() {
            hand_selection_vec.push(
                Selection {
                    location: Location::Hand,
                    row: 0,
                    col: i
                }
            )
        }
        hand_selection_vec
    }

    fn print_hand(&self, selection: usize, show_selection: bool) {
        print!("                                                  ");
        for (i, piece) in self.hand.iter().enumerate() {
            let mut selected = i == selection;
            selected &= show_selection;
            print_piece(*piece, selected);
        }
        println!();
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Location {
    Board,
    Hand,
    None
}

#[derive(Debug, Copy, Clone)]
struct Selection {
    location: Location,
    row: usize,
    col: usize,
}

////////////////////////////////////////////////////////////////////////
enum MoveDirection {
    Next,
    Previous,
}

#[derive(Debug)]
struct Game {
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    player_with_turn: Player,
    player_without_turn: Player,
    state: State,
    piece_destination_vec_index: usize,
    piece_destination_vec: Vec<Selection>,
    piece_source_vec_index: usize,
    piece_source_vec: Vec<Selection>,
}

impl Game {
    
    fn new() -> Self {
        // TODO make this a 3d array
        let board: [[Piece; BOARD_SIZE]; BOARD_SIZE] = [[create_piece(Bug::None, PlayerNumber::None); BOARD_SIZE]; BOARD_SIZE];
        let player_with_turn = Player::new(PlayerNumber::One);
        let player_without_turn = Player::new(PlayerNumber::Two);
        let state = State::SelectPiece;
        let piece_destination_vec_index: usize = 0;
        let piece_destination_vec = vec![Selection{location: Location::Board, row: 0, col: 0}];
        let piece_source_vec_index: usize = 0;
        let piece_source_vec = vec![Selection{location: Location::Hand, row: 0, col: 0}];
        
        Game {
            board,
            player_with_turn,
            player_without_turn,
            state,
            piece_destination_vec_index,
            piece_destination_vec,
            piece_source_vec_index,
            piece_source_vec
        }
    }

    fn clear_selections(&mut self) {
        self.piece_source_vec = vec![];
        self.piece_destination_vec = vec![];
        self.piece_source_vec_index = 0;
        self.piece_destination_vec_index = 0;
    }

    fn update(&mut self) {
        self.find_piece_sources();
        self.find_piece_destinations();
    }

    fn get_piece_source(&self) -> Selection {
        if self.piece_source_vec.is_empty() {
            return Selection {
                location: Location::None,
                row: 0,
                col: 0
            }
        }
        self.piece_source_vec[self.piece_source_vec_index]
    }

    fn get_piece_destination(&self) -> Selection {
        if self.piece_destination_vec.is_empty() {
            return Selection {
                location: Location::None,
                row: 0,
                col: 0
            }
        }
        self.piece_destination_vec[self.piece_destination_vec_index]
    }

    fn move_piece_cursor(&mut self, move_direction: MoveDirection) {
        
        match move_direction {
            MoveDirection::Next => {
                if self.piece_source_vec_index >= self.piece_source_vec.len() - 1 {
                    self.piece_source_vec_index = 0;
                }
                else {
                    self.piece_source_vec_index += 1;
                }
            }
            MoveDirection::Previous => {
                if self.piece_source_vec_index == 0 {
                    self.piece_source_vec_index = self.piece_source_vec.len() - 1
                }
                else {
                    self.piece_source_vec_index -= 1;
                }
            }
        }
    }

    fn move_location_cursor(&mut self, move_direction: MoveDirection) {

        // TODO Handle when player has nowhere to place
        assert!(!self.piece_destination_vec.is_empty());
        
        match move_direction {
            MoveDirection::Next => {
                if self.piece_destination_vec_index >= self.piece_destination_vec.len() - 1 {
                    self.piece_destination_vec_index = 0;
                }
                else {
                    self.piece_destination_vec_index += 1;
                }
            }
            MoveDirection::Previous => {
                if self.piece_destination_vec_index == 0 {
                    self.piece_destination_vec_index = self.piece_destination_vec.len() - 1
                }
                else {
                    self.piece_destination_vec_index -= 1;
                }
            }
        }
    }

    // Not sure I like this function
    fn place_selected_piece(&mut self) {

        let selection = self.get_piece_source();
        let piece_to_place = match selection.location {
            Location::Board => {
                self.board[selection.row][selection.col]
            },
            Location::Hand => {
                self.player_with_turn.hand.remove(selection.col)
            },
            Location::None => {
                panic!();
            }
        };
        let piece_destination = self.get_piece_destination();
        self.board[selection.row][selection.col] = create_piece(Bug::None, PlayerNumber::None);
        self.board[piece_destination.row][piece_destination.col] = piece_to_place;
        self.piece_source_vec_index = 0;
    }

    fn advance_turn(&mut self) {
        let temp_player = self.player_with_turn.clone();
        self.player_with_turn = self.player_without_turn.clone();
        self.player_without_turn = temp_player;
    }

    fn get_board_selections(&mut self) -> Vec<Selection>{
        let mut board_selection_vec = vec![];

        for (i, row) in self.board.iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                
                if piece.player == self.player_with_turn.number {
                    board_selection_vec.push(
                        Selection {
                            location: Location::Board,
                            row: i,
                            col: j
                        }
                    );
                }
            }
        }
        board_selection_vec
    }

    fn find_piece_sources(&mut self) {
        
        let mut piece_source_vec = self.player_with_turn.get_hand_selection_vec();
        let board_selection_vec = self.get_board_selections();
        piece_source_vec.extend(board_selection_vec);
        self.piece_source_vec = piece_source_vec;
    }

    fn find_piece_destinations(&mut self) {
        
        let mut piece_destination_vec = vec![];
        let placeable_location_vec = self.find_placeable_locations();
        piece_destination_vec.extend(placeable_location_vec);
        let movable_location_vec = self.find_movable_locations();
        piece_destination_vec.extend(movable_location_vec);
        self.piece_destination_vec = piece_destination_vec;
    }

    fn find_movable_locations(&mut self) -> Vec<Selection> {
        let mut moveable_location_vec: Vec<Selection> = vec![];
        let selection = self.get_piece_source();
        if selection.location != Location::Board {
            return moveable_location_vec;
        }
        let piece_to_move = self.board[selection.row][selection.col];
        match piece_to_move.bug {
            Bug::Grasshopper => {

            },
            Bug::Ant => {
                for (i, row) in self.board.iter().enumerate() {
                    for (j, _piece) in row.iter().enumerate() {
                        // FIXME Ignores the indexes in the corners of the board
                        if i < 2 || self.board.len() - 2 <= i || j < 2 || self.board.len() - 2 <= j {
                            continue;
                        }

                        let current_location_occupied = self.board[i][j].player != PlayerNumber::None;
                        if current_location_occupied {
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

                        let mut neighboring_piece = false;
                        for neighbor in neighboring_piece_vec {
                            if neighbor != PlayerNumber::None {
                                neighboring_piece = true;
                            }
                        }

                        if !neighboring_piece {
                            continue;
                        }

                        moveable_location_vec.push(
                            Selection {
                                location: Location::Board,
                                row: i,
                                col: j
                            }
                        )
                    }
                }
            }
            _ => {}
        }
        moveable_location_vec
    }

    fn find_placeable_locations(&mut self) -> Vec<Selection>{

        let mut placeable_location_vec: Vec<Selection> = vec![];
        let selection = self.get_piece_source();
        if selection.location != Location::Hand {
            return placeable_location_vec;
        }
    
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
    
                placeable_location_vec.push(Selection { location: Location::Board, row: i, col: j });
            }
        }
    
        // Handles the first turn for each player where they have no existing
        // pieces to play off of
        if locations_occupied <= 1 {
            if self.player_with_turn.number == PlayerNumber::One {
                placeable_location_vec.push(Selection { location: Location::Board, row: FIRST_LOCATION.0, col: FIRST_LOCATION.1 });
            }
            else if self.player_with_turn.number == PlayerNumber::Two {
                placeable_location_vec.push(Selection { location: Location::Board, row: FIRST_LOCATION.0 - 2, col: FIRST_LOCATION.1 });     // North
                placeable_location_vec.push(Selection { location: Location::Board, row: FIRST_LOCATION.0 - 1, col: FIRST_LOCATION.1 - 1 }); // Northwest
                placeable_location_vec.push(Selection { location: Location::Board, row: FIRST_LOCATION.0 - 1, col: FIRST_LOCATION.1 + 1 }); // Northeast
                placeable_location_vec.push(Selection { location: Location::Board, row: FIRST_LOCATION.0 + 2, col: FIRST_LOCATION.1 });     // South
                placeable_location_vec.push(Selection { location: Location::Board, row: FIRST_LOCATION.0 + 1, col: FIRST_LOCATION.1 - 1 }); // Southwest
                placeable_location_vec.push(Selection { location: Location::Board, row: FIRST_LOCATION.0 + 1, col: FIRST_LOCATION.1 + 1 }); // Southeast
            } else {
                panic!();
            }
        }
        placeable_location_vec
    }

    fn print_board(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                let piece_destination = self.get_piece_destination();
                let piece_source = self.get_piece_source();
                let destination_selected = i == piece_destination.row && j == piece_destination.col;
                let mut source_selected = i == piece_source.row && j == piece_source.col;
                source_selected &= piece_source.location == Location::Board;
                match self.state {
                    State::SelectPiece => print_piece(*piece, source_selected),
                    State::SelectPlacingLocation => print_piece(*piece, destination_selected),
                    State::ConfirmPlacingLocation => print_piece(*piece, destination_selected),
                    _ => print_piece(*piece, false),
                }
            }
            println!();
        }
    }
    
    fn print(&mut self) {
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
        let piece_source = self.get_piece_source();
        self.player_with_turn.print_hand(piece_source.col, true);
        self.player_without_turn.print_hand(piece_source.col, false);
        println!();
        self.print_board();
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    // DecideToPlaceOrMove,
    SelectPiece,
    SelectPlacingLocation,
    ConfirmPlacingLocation,
    // SelectPieceOnBoard,
    // SelectMovingLocation,
    // ConfirmMovingLocation,
}

fn print_prompt(state: &State, player_turn: PlayerNumber) {
    let prompt_string = match state {
        State::SelectPiece => format!("Player {}: Select a bug", player_turn),
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