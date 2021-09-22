use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub enum Color {
    White,
    Black
}

pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Pawn(Color)
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: [[Option<Piece>; 8]; 8],
    active: Color
}

impl Piece {
    fn icon (&self) -> String {
        match self {
            Piece::King(Color::White) => "\u{2654}",
            Piece::Queen(Color::White) => "\u{2655}",
            Piece::Rook(Color::White) => "\u{2656}",
            Piece::Bishop(Color::White) => "\u{2657}",
            Piece::Knight(Color::White) => "\u{2658}",
            Piece::Pawn(Color::White) => "\u{2659}",

            Piece::King(Color::Black) => "\u{265A}",
            Piece::Queen(Color::Black) => "\u{265B}",
            Piece::Rook(Color::Black) => "\u{265C}",
            Piece::Bishop(Color::Black) => "\u{265D}",
            Piece::Knight(Color::Black) => "\u{265E}",
            Piece::Pawn(Color::Black) => "\u{265F}"
        }.to_owned()
    }
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active: Color::White,
            board: [
                [
                    Some(Piece::Rook(Color::Black)),
                    Some(Piece::Knight(Color::Black)),
                    Some(Piece::Bishop(Color::Black)),
                    Some(Piece::King(Color::Black)),
                    Some(Piece::Queen(Color::Black)),
                    Some(Piece::Bishop(Color::Black)),
                    Some(Piece::Knight(Color::Black)),
                    Some(Piece::Rook(Color::Black))
                ],
                [
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black))
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White))
                ],
                [
                    Some(Piece::Rook(Color::White)),
                    Some(Piece::Knight(Color::White)),
                    Some(Piece::Bishop(Color::White)),
                    Some(Piece::King(Color::White)),
                    Some(Piece::Queen(Color::White)),
                    Some(Piece::Bishop(Color::White)),
                    Some(Piece::Knight(Color::White)),
                    Some(Piece::Rook(Color::White))
                ]
            ]
        }
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        None
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    /* build board representation string */
    
    let mut repr: String = "".to_owned();
    let mut rank_idx = 9;

    repr.push_str("\n\n  A B C D E F G H\n");

    for rank in &self.board {
        rank_idx -= 1;
        repr.push_str(&rank_idx.to_string());
        repr.push_str(" ");

        for piece in rank {
            if piece.is_none() {repr.push_str("* ");} else {repr.push_str(&piece.as_ref().unwrap().icon()); repr.push_str(" ");}
        }
        repr.push_str("\n");
    }
    write!(f, "{}", repr)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();
        println!("{:?}", game);
        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
