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

 #[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, PartialEq)]
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
                [Some(Piece::Pawn(Color::Black)); 8],
                [None; 8], [None; 8], [None; 8], [None; 8],
                [Some(Piece::Pawn(Color::White)); 8],
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
    
    /// Return the piece at the position, if there is one
    fn get_piece(&self, coords: [usize; 2]) -> Option<&Piece> {
        let piece = self.board[coords[1]][coords[0]].as_ref();
            if piece.is_none() {return None;} else {return Some(piece.unwrap());}
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
        let piece = self.get_piece(pos_from_string(&_postion));
        if piece.is_none() {
            return None
        }
        else {
            match piece.unwrap() {
                Piece::Pawn(_) => return self.pawn_possible(&_postion),
                _ => return None
            }
        }
    }
    
    fn pawn_possible(&self, position: &String) -> Option<Vec<String>> {
        let coords = pos_from_string(position);
        let piece = self.get_piece(pos_from_string(position)).unwrap();
        let iswhite = piece == &Piece::Pawn(Color::White);
        let mut string_positions = vec!();
        let mut diagonal_moves = vec!();

        // return None if piece at edge of board
        if coords[1] == 0 || coords[1] == 7 {return None;}

        // 1 tile forward
        let mut possible_positions = if iswhite {
            vec!([coords[0], coords[1]-1])
        }
        else {
            vec!([coords[0], coords[1]+1])
        };

        // 2 tiles forward
        if iswhite && self.get_piece([coords[0], coords[1]-1]).is_none() {
            possible_positions.push([coords[0], coords[1]-2]);
        }
        else if !iswhite && self.get_piece([coords[0], coords[1]+1]).is_none() {
            possible_positions.push([coords[0], coords[1]+2])
        }
        
        // remove occupied positions
        for position in possible_positions {
            if self.get_piece(position).is_none() {
                string_positions.push(pos_to_string(position));
            }
        }

        // diagonal moves if piece not at edge of board
        if iswhite {
            if coords[0] > 0 {
                diagonal_moves.push([coords[0]-1, coords[1]-1]);
            }
            if coords[0] < 7 {
                diagonal_moves.push([coords[0]+1, coords[1]-1]);
            }
        else {
            if coords[0] > 0 {
                diagonal_moves.push([coords[0]+1, coords[1]+1]);
            }
            if coords[0] < 7 {
                diagonal_moves.push([coords[0]+1, coords[1]+1]);
            }
        }
        }
        // diagnonal move possible if tile occupied

        // Is there a better way?
        for pos in diagonal_moves {
            if !self.get_piece(pos).is_none() {
                if !iswhite {
                    match piece {
                        Piece::Bishop(Color::White) => string_positions.push(pos_to_string(pos)),
                        Piece::Pawn(Color::White) => string_positions.push(pos_to_string(pos)),
                        Piece::King(Color::White) => string_positions.push(pos_to_string(pos)),
                        Piece::Queen(Color::White) => string_positions.push(pos_to_string(pos)),
                        Piece::Rook(Color::White) => string_positions.push(pos_to_string(pos)),
                        Piece::Knight(Color::White) => string_positions.push(pos_to_string(pos)),
                        _ => ()
                    }
                }
                else {
                    match piece {
                        Piece::Bishop(Color::Black) => string_positions.push(pos_to_string(pos)),
                        Piece::Pawn(Color::Black) => string_positions.push(pos_to_string(pos)),
                        Piece::King(Color::Black) => string_positions.push(pos_to_string(pos)),
                        Piece::Queen(Color::Black) => string_positions.push(pos_to_string(pos)),
                        Piece::Rook(Color::Black) => string_positions.push(pos_to_string(pos)),
                        Piece::Knight(Color::Black) => string_positions.push(pos_to_string(pos)),
                        _ => ()
                    }
                }
            }
        }
        return Some(string_positions);
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

fn pos_from_string(position: &String) -> [usize; 2]{
        let mut positions = position.chars();
        let file = positions.next().unwrap();
        let rank = positions.next().unwrap();
        let rank_idx = (8 - rank.to_digit(10).unwrap()) as usize;
        let file_idx = match file {
            'a' => 0 as usize, 'A' => 0 as usize, 'b' => 1 as usize, 'B' => 1 as usize,
            'c' => 2 as usize, 'C' => 2 as usize, 'd' => 3 as usize, 'D' => 3 as usize,
            'e' => 4 as usize, 'E' => 4 as usize, 'f' => 5 as usize, 'F' => 5 as usize,
            'g' => 6 as usize, 'G' => 6 as usize, 'h' => 7 as usize, 'H' => 7 as usize,
            _ => 8 as usize //why can this happen?
        };
        return [file_idx, rank_idx];
}

fn pos_to_string(coords: [usize; 2]) -> String {
    let mut position = "".to_string();
    let rank = 8 - coords[1];
    position.push_str(
        match coords[0] {
            0 => "a", 1 => "b", 2 => "c", 3 => "d",
            4 => "e", 5 => "f", 6 => "g", 7 => "h",
            _ => "i"
        }
    );
    position.push_str(&(rank).to_string());
    return position;
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;
    use super::pos_from_string;

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
    
    #[test]
    fn get_piece() {
        let game = Game::new();
        let position = &"A1".to_string();
        let piece = game.get_piece(pos_from_string(position));
        let icon = if piece.is_none() {"*".to_string()} else {piece.unwrap().icon()};
        println!("\n\nPiece at {}: {}\n", position, icon);
    }
    
    #[test]
    fn possib_moves() {
        let game = Game::new();
        let position = "A2".to_string().to_owned();
        let possible_moves = game.get_possible_moves(position).unwrap();
        println!();
        for pos in possible_moves {
            println!("{}", pos);
        }
    }
}
