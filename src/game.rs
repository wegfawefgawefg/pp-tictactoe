#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    X,
    O,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::X => "X",
                Piece::O => "O",
            }
        )
    }
}

pub type Board = Vec<Vec<Option<Piece>>>;

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pos = position_to_move_code(self)
            .unwrap_or_else(|| panic!("Invalid position: ({}, {})", self.x, self.y));
        write!(f, "{}", pos)
    }
}

pub fn no_more_moves(board: &Board) -> bool {
    board
        .iter()
        .all(|row| row.iter().all(|cell| cell.is_some()))
}

pub fn is_game_won(board: &Board) -> Option<Piece> {
    // rows and columns
    for i in 0..3 {
        if let Some(piece) = check_line(&[board[i][0], board[i][1], board[i][2]]) {
            return Some(piece);
        }
        if let Some(piece) = check_line(&[board[0][i], board[1][i], board[2][i]]) {
            return Some(piece);
        }
    }

    // diagonals
    if let Some(piece) = check_line(&[board[0][0], board[1][1], board[2][2]]) {
        return Some(piece);
    }
    if let Some(piece) = check_line(&[board[0][2], board[1][1], board[2][0]]) {
        return Some(piece);
    }

    None
}

pub fn check_line(line: &[Option<Piece>]) -> Option<Piece> {
    if line[0] == line[1] && line[1] == line[2] {
        line[0]
    } else {
        None
    }
}

pub fn move_code_to_position(input: &str) -> Option<Position> {
    if input.len() != 1 {
        return None;
    }

    let num = match input.parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input, try again.");
            return None;
        }
    };

    // its a numpad, retard
    match num {
        7 => Some(Position { x: 0, y: 0 }),
        8 => Some(Position { x: 1, y: 0 }),
        9 => Some(Position { x: 2, y: 0 }),
        4 => Some(Position { x: 0, y: 1 }),
        5 => Some(Position { x: 1, y: 1 }),
        6 => Some(Position { x: 2, y: 1 }),
        1 => Some(Position { x: 0, y: 2 }),
        2 => Some(Position { x: 1, y: 2 }),
        3 => Some(Position { x: 2, y: 2 }),
        _ => None,
    }
}

pub fn position_to_move_code(pos: &Position) -> Option<u8> {
    match (pos.x, pos.y) {
        (0, 0) => Some(7),
        (1, 0) => Some(8),
        (2, 0) => Some(9),
        (0, 1) => Some(4),
        (1, 1) => Some(5),
        (2, 1) => Some(6),
        (0, 2) => Some(1),
        (1, 2) => Some(2),
        (2, 2) => Some(3),
        _ => None,
    }
}

pub fn is_valid_move(board: &Board, pos: &Position) -> bool {
    pos.x < 3 && pos.y < 3 && board[pos.y as usize][pos.x as usize].is_none()
}

pub fn update_board(board: &mut Board, pos: &Position, piece: Piece) {
    board[pos.y as usize][pos.x as usize] = Some(piece);
}

pub fn display_board(board: &Board) {
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                Some(piece) => print!("{}", piece),
                None => {
                    let pos = position_to_move_code(&Position {
                        x: x as u8,
                        y: y as u8,
                    })
                    .unwrap_or_else(|| panic!("Invalid position: ({}, {})", x, y));

                    print!("{}", pos);
                }
            }
        }
        println!();
    }
}
