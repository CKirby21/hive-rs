use colored::Colorize;
use console::Term;
use std::collections::HashSet;
use std::fmt;

const BOARD_SIZE: usize = 40;
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
                State::SelectPiece => match character {
                    LEFT_KEY => {
                        game.move_piece_cursor(MoveDirection::Previous);
                    }
                    RIGHT_KEY => {
                        game.move_piece_cursor(MoveDirection::Next);
                    }
                    ADVANCE_KEY => {
                        game.state = State::SelectPlacingLocation;
                    }
                    _ => continue,
                },
                State::SelectPlacingLocation => match character {
                    LEFT_KEY => {
                        game.move_location_cursor(MoveDirection::Previous);
                    }
                    RIGHT_KEY => {
                        game.move_location_cursor(MoveDirection::Next);
                    }
                    ADVANCE_KEY => {
                        game.state = State::ConfirmPlacingLocation;
                    }
                    BACK_KEY => {
                        game.state = State::SelectPiece;
                    }
                    _ => continue,
                },
                State::ConfirmPlacingLocation => match character {
                    ADVANCE_KEY => {
                        game.place_selected_piece();
                        game.advance_turn();
                        game.clear_selections();
                        game.state = State::SelectPiece;
                    }
                    BACK_KEY => {
                        game.state = State::SelectPlacingLocation;
                    }
                    _ => continue,
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
            PlayerNumber::One => write!(f, "1"),
            PlayerNumber::Two => write!(f, "2"),
            PlayerNumber::None => write!(f, "!"),
        }
    }
}

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

    fn get_hand_selection_vec(&self) -> Vec<Selection> {
        let mut hand_selection_vec: Vec<Selection> = vec![];

        for (i, _piece) in self.hand.iter().enumerate() {
            hand_selection_vec.push(Selection {
                location: Location::Hand,
                row: 0,
                col: i,
            })
        }
        hand_selection_vec
    }

    fn print_hand(&self, selection: usize, show_selection: bool) {
        print!("                                                  ");
        for (i, piece) in self.hand.iter().enumerate() {
            let mut selected = i == selection;
            selected &= show_selection;
            piece.print(selected);
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

impl Piece {
    fn print(&self, selected: bool) {
        let piece_string = format!("{}", self.bug);
        let piece_string_colored = match self.player {
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

    fn new(bug: Bug, player: PlayerNumber) -> Self {
        Piece { bug, player }
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Location {
    Board,
    Hand,
    None,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Selection {
    location: Location,
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

const DIRECTION_ARR: [Direction; 6] = [
    Direction::North,
    Direction::Northeast,
    Direction::Southeast,
    Direction::South,
    Direction::Southwest,
    Direction::Northwest,
];

#[derive(Debug, Copy, Clone, PartialEq)]
enum FindError {
    NotFound,
}

fn move_selection(selection: Selection, direction: Direction) -> Result<Selection, Selection> {
    let mut moved_selection = selection.clone();
    if moved_selection.row < 2 || moved_selection.col < 2 {
        return Err(moved_selection);
    }
    match direction {
        Direction::North => {
            moved_selection.row -= 2;
        }
        Direction::Northeast => {
            moved_selection.row -= 1;
            moved_selection.col += 1;
        }
        Direction::Southeast => {
            moved_selection.row += 1;
            moved_selection.col += 1;
        }
        Direction::South => {
            moved_selection.row += 2;
        }
        Direction::Southwest => {
            moved_selection.row += 1;
            moved_selection.col -= 1;
        }
        Direction::Northwest => {
            moved_selection.row -= 1;
            moved_selection.col -= 1;
        }
    }
    Ok(moved_selection)
}

fn find_grasshopper_movable_location(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    selection: Selection,
) -> Vec<Selection> {
    let mut location_vec = vec![];
    for direction in DIRECTION_ARR {
        let result = test_grasshopper_direction(board, direction, selection);
        if result.is_ok() {
            location_vec.push(result.unwrap());
        }
    }
    location_vec
}

fn test_grasshopper_direction(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    direction: Direction,
    selection: Selection,
) -> Result<Selection, FindError> {
    let starting_selection = move_selection(selection, direction).unwrap();
    let mut current_selection = starting_selection;
    loop {
        if current_selection.row >= board.len() || current_selection.col >= board.len() {
            break;
        }
        let current_player = board[current_selection.row][current_selection.col].player;
        // println!("{}, {}, {}", i, j, current_player);
        if current_player == PlayerNumber::None {
            if current_selection.row != starting_selection.row
                || current_selection.col != starting_selection.col
            {
                return Ok(Selection {
                    location: Location::Board,
                    row: current_selection.row,
                    col: current_selection.col,
                });
            }
            break;
        }
        let move_result = move_selection(current_selection, direction);
        if move_result.is_err() {
            break;
        }
        current_selection = move_result.unwrap();
    }
    Err(FindError::NotFound)
}

fn find_slide_locations(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    selection: Selection,
    slides: i32,
) -> Vec<Selection> {
    let valid_location_vec = find_ant_locations(board, selection);
    let mut traversed_vec = vec![selection.clone()];
    let mut current_vec = traversed_vec.clone();

    for _ in 1..=slides {
        let clone_vec = current_vec.clone();
        current_vec.clear();
        for current_selection in clone_vec {
            for direction in DIRECTION_ARR {
                let result = move_selection(current_selection, direction);
                if result.is_err() {
                    continue;
                }
                let moved_selection = result.unwrap();
                if valid_location_vec.contains(&moved_selection)
                    && !traversed_vec.contains(&moved_selection)
                {
                    current_vec.push(moved_selection);
                    traversed_vec.push(moved_selection);
                }
            }
        }
    }

    current_vec
}

fn find_ant_locations(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    selection: Selection,
) -> Vec<Selection> {
    let mut board_clone = board.clone();
    board_clone[selection.row][selection.col] = Piece::new(Bug::None, PlayerNumber::None);

    let mut location_vec = vec![];
    for (i, row) in board_clone.iter().enumerate() {
        for (j, _piece) in row.iter().enumerate() {
            if check_for_occupied_location(board_clone, i, j) {
                continue;
            }

            if !check_for_neighboring_piece(board_clone, i, j) {
                continue;
            }

            if !check_for_slide_in(board_clone, i, j) {
                continue;
            }

            location_vec.push(Selection {
                location: Location::Board,
                row: i,
                col: j,
            })
        }
    }
    location_vec
}

fn find_queen_locations(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    selection: Selection,
) -> Vec<Selection> {
    let ant_location_vec = find_ant_locations(board, selection);
    let mut location_vec = vec![];
    for direction in DIRECTION_ARR {
        let result = move_selection(selection, direction);
        if result.is_ok() {
            let current_selection = result.unwrap();
            if ant_location_vec.contains(&current_selection) {
                location_vec.push(current_selection);
            }
        }
    }
    location_vec
}

fn get_neighboring_piece_vec(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    row: usize,
    col: usize,
) -> Vec<Piece> {
    let mut neighboring_piece_vec: Vec<Piece> = vec![];
    // North
    if row >= 2 {
        neighboring_piece_vec.push(board[row - 2][col])
    }
    // Northwest
    if row >= 1 && col >= 1 {
        neighboring_piece_vec.push(board[row - 1][col - 1])
    }
    // Northeast
    if row >= 1 && col <= board.len() - 2 {
        neighboring_piece_vec.push(board[row - 1][col + 1])
    }
    // Southwest
    if row <= board.len() - 2 && col >= 1 {
        neighboring_piece_vec.push(board[row + 1][col - 1])
    }
    // Southeast
    if row <= board.len() - 2 && col <= board.len() - 2 {
        neighboring_piece_vec.push(board[row + 1][col + 1])
    }
    // South
    if row <= board.len() - 3 {
        neighboring_piece_vec.push(board[row + 2][col])
    }
    return neighboring_piece_vec;
}

//////////////////////////////////////////////////////////////////////
/// Rules
//////////////////////////////////////////////////////////////////////

fn check_for_neighboring_piece(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    row: usize,
    col: usize,
) -> bool {
    let neighboring_piece_vec = get_neighboring_piece_vec(board, row, col);
    let mut neighboring_piece = false;
    for neighbor in neighboring_piece_vec {
        if neighbor.player != PlayerNumber::None {
            neighboring_piece = true;
        }
    }
    neighboring_piece
}

fn check_for_slide_in(board: [[Piece; BOARD_SIZE]; BOARD_SIZE], row: usize, col: usize) -> bool {
    let neighboring_piece_vec = get_neighboring_piece_vec(board, row, col);
    let mut neighboring_piece_count = 0;
    for neighbor in neighboring_piece_vec {
        if neighbor.player != PlayerNumber::None {
            neighboring_piece_count += 1;
        }
    }
    let slide_in: bool = neighboring_piece_count <= 4;
    slide_in
}

fn check_for_occupied_location(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    row: usize,
    col: usize,
) -> bool {
    let occupied_location = board[row][col].player != PlayerNumber::None;
    occupied_location
}

// Recursive function to navigate the board and fill a set of connected pieces from the starting selection.
// May bite me in the future but for now it seems to get the job done
fn discover(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    selection: Selection,
    set: &mut HashSet<Selection>,
) {
    // println!("Called {} {}", selection.row, selection.col);
    set.insert(selection);
    for direction in DIRECTION_ARR {
        let result = move_selection(selection, direction);
        if result.is_ok() {
            let moved_selection = result.unwrap();
            if !check_for_occupied_location(board, moved_selection.row, moved_selection.col) {
                continue;
            }
            if set.contains(&Selection {
                location: Location::Board,
                row: moved_selection.row,
                col: moved_selection.col,
            }) {
                continue;
            }
            discover(board, moved_selection, set);
        }
    }
}

// FIXME
fn check_for_broken_hive_if_empty(
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    row: usize,
    col: usize,
) -> bool {
    let mut board_clone = board.clone();
    board_clone[row][col] = Piece::new(Bug::None, PlayerNumber::None);

    let mut occupied_locations = vec![];
    for (i, row) in board_clone.iter().enumerate() {
        for (j, _piece) in row.iter().enumerate() {
            if check_for_occupied_location(board_clone, i, j) {
                occupied_locations.push(Selection {
                    location: Location::Board,
                    row: i,
                    col: j,
                });
            }
        }
    }

    let mut occupied_location_set = HashSet::new();
    discover(
        board_clone,
        occupied_locations[0],
        &mut occupied_location_set,
    );

    let broken_hive = occupied_location_set.len() != occupied_locations.len();
    broken_hive
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
        let board: [[Piece; BOARD_SIZE]; BOARD_SIZE] =
            [[Piece::new(Bug::None, PlayerNumber::None); BOARD_SIZE]; BOARD_SIZE];
        let player_with_turn = Player::new(PlayerNumber::One);
        let player_without_turn = Player::new(PlayerNumber::Two);
        let state = State::SelectPiece;
        let piece_destination_vec_index: usize = 0;
        let piece_destination_vec = vec![Selection {
            location: Location::Board,
            row: 0,
            col: 0,
        }];
        let piece_source_vec_index: usize = 0;
        let piece_source_vec = vec![Selection {
            location: Location::Hand,
            row: 0,
            col: 0,
        }];

        Game {
            board,
            player_with_turn,
            player_without_turn,
            state,
            piece_destination_vec_index,
            piece_destination_vec,
            piece_source_vec_index,
            piece_source_vec,
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
                col: 0,
            };
        }
        self.piece_source_vec[self.piece_source_vec_index]
    }

    fn get_piece_destination(&self) -> Selection {
        if self.piece_destination_vec.is_empty() {
            return Selection {
                location: Location::None,
                row: 0,
                col: 0,
            };
        }
        self.piece_destination_vec[self.piece_destination_vec_index]
    }

    fn move_piece_cursor(&mut self, move_direction: MoveDirection) {
        match move_direction {
            MoveDirection::Next => {
                if self.piece_source_vec_index >= self.piece_source_vec.len() - 1 {
                    self.piece_source_vec_index = 0;
                } else {
                    self.piece_source_vec_index += 1;
                }
            }
            MoveDirection::Previous => {
                if self.piece_source_vec_index == 0 {
                    self.piece_source_vec_index = self.piece_source_vec.len() - 1
                } else {
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
                } else {
                    self.piece_destination_vec_index += 1;
                }
            }
            MoveDirection::Previous => {
                if self.piece_destination_vec_index == 0 {
                    self.piece_destination_vec_index = self.piece_destination_vec.len() - 1
                } else {
                    self.piece_destination_vec_index -= 1;
                }
            }
        }
    }

    // Not sure I like this function
    fn place_selected_piece(&mut self) {
        let selection = self.get_piece_source();
        let piece_to_place = match selection.location {
            Location::Board => self.board[selection.row][selection.col],
            Location::Hand => self.player_with_turn.hand.remove(selection.col),
            Location::None => {
                panic!();
            }
        };
        let piece_destination = self.get_piece_destination();
        self.board[selection.row][selection.col] = Piece::new(Bug::None, PlayerNumber::None);
        self.board[piece_destination.row][piece_destination.col] = piece_to_place;
        self.piece_source_vec_index = 0;
    }

    fn advance_turn(&mut self) {
        let temp_player = self.player_with_turn.clone();
        self.player_with_turn = self.player_without_turn.clone();
        self.player_without_turn = temp_player;
    }

    fn get_board_selections(&mut self) -> Vec<Selection> {
        let mut board_selection_vec = vec![];

        for (i, row) in self.board.iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                if piece.player != self.player_with_turn.number {
                    continue;
                }

                if check_for_broken_hive_if_empty(self.board, i, j) {
                    continue;
                }

                board_selection_vec.push(Selection {
                    location: Location::Board,
                    row: i,
                    col: j,
                });
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
        // FIXME at some point
        match piece_to_move.bug {
            Bug::Grasshopper => {
                moveable_location_vec = find_grasshopper_movable_location(self.board, selection);
            }
            Bug::Spider => {
                moveable_location_vec = find_slide_locations(self.board, selection, 3);
            }
            Bug::Ant => {
                moveable_location_vec = find_ant_locations(self.board, selection);
            }
            Bug::Queen => {
                moveable_location_vec = find_slide_locations(self.board, selection, 1);
            }
            _ => {}
        }
        moveable_location_vec
    }

    fn find_placeable_locations(&mut self) -> Vec<Selection> {
        let mut placeable_location_vec: Vec<Selection> = vec![];
        let selection = self.get_piece_source();
        if selection.location != Location::Hand {
            return placeable_location_vec;
        }

        let mut locations_occupied = 0;

        for (i, row) in self.board.iter().enumerate() {
            for (j, _piece) in row.iter().enumerate() {
                let current_location_occupied = self.board[i][j].player != PlayerNumber::None;
                if current_location_occupied {
                    locations_occupied += 1;
                    continue;
                }

                let neighboring_piece_vec = get_neighboring_piece_vec(self.board, i, j);

                let mut neighboring_piece_from_another_player = false;
                let mut neighboring_piece_from_same_player = false;
                for neighbor in neighboring_piece_vec {
                    if neighbor.player == self.player_with_turn.number {
                        neighboring_piece_from_same_player = true;
                    } else if neighbor.player == PlayerNumber::None {
                        // Do nothing
                    } else {
                        neighboring_piece_from_another_player = true;
                    }
                }

                if neighboring_piece_from_another_player {
                    continue;
                }
                if !neighboring_piece_from_same_player {
                    continue;
                }

                placeable_location_vec.push(Selection {
                    location: Location::Board,
                    row: i,
                    col: j,
                });
            }
        }

        // Handles the first turn for each player where they have no existing
        // pieces to play off of
        if locations_occupied <= 1 {
            if self.player_with_turn.number == PlayerNumber::One {
                placeable_location_vec.push(Selection {
                    location: Location::Board,
                    row: FIRST_LOCATION.0,
                    col: FIRST_LOCATION.1,
                });
            } else if self.player_with_turn.number == PlayerNumber::Two {
                placeable_location_vec.push(Selection {
                    location: Location::Board,
                    row: FIRST_LOCATION.0 - 2,
                    col: FIRST_LOCATION.1,
                }); // North
                placeable_location_vec.push(Selection {
                    location: Location::Board,
                    row: FIRST_LOCATION.0 - 1,
                    col: FIRST_LOCATION.1 - 1,
                }); // Northwest
                placeable_location_vec.push(Selection {
                    location: Location::Board,
                    row: FIRST_LOCATION.0 - 1,
                    col: FIRST_LOCATION.1 + 1,
                }); // Northeast
                placeable_location_vec.push(Selection {
                    location: Location::Board,
                    row: FIRST_LOCATION.0 + 2,
                    col: FIRST_LOCATION.1,
                }); // South
                placeable_location_vec.push(Selection {
                    location: Location::Board,
                    row: FIRST_LOCATION.0 + 1,
                    col: FIRST_LOCATION.1 - 1,
                }); // Southwest
                placeable_location_vec.push(Selection {
                    location: Location::Board,
                    row: FIRST_LOCATION.0 + 1,
                    col: FIRST_LOCATION.1 + 1,
                }); // Southeast
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
                    State::SelectPiece => piece.print(source_selected),
                    State::SelectPlacingLocation => piece.print(destination_selected),
                    State::ConfirmPlacingLocation => piece.print(destination_selected),
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
        State::ConfirmPlacingLocation => {
            format!("Player {}: Are you quite sure about that?", player_turn)
        }
    };
    let prompt_string_colored = if player_turn == PlayerNumber::One {
        prompt_string.blue()
    } else {
        prompt_string.red()
    };
    println!(
        "                                                  {}",
        prompt_string_colored
    );
}

////////////////////////////////////////////////////////////////////////

// fn clamp(min: i32, value: i32, max: i32) -> i32 {
//     if value < min {
//         min
//     }
//     else if value > max {
//         max
//     }
//     else {
//         value
//     }
// }

fn create_hand(player: PlayerNumber) -> Vec<Piece> {
    let hand: Vec<Piece> = vec![
        Piece::new(Bug::Grasshopper, player),
        Piece::new(Bug::Grasshopper, player),
        Piece::new(Bug::Grasshopper, player),
        Piece::new(Bug::Spider, player),
        Piece::new(Bug::Spider, player),
        Piece::new(Bug::Ant, player),
        Piece::new(Bug::Ant, player),
        Piece::new(Bug::Ant, player),
        Piece::new(Bug::Queen, player),
        Piece::new(Bug::Beetle, player),
        Piece::new(Bug::Beetle, player),
    ];
    hand
}
