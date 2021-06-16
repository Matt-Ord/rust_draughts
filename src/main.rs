use std::{convert::TryInto, fmt, usize};
use ux::*;

#[derive(Copy, Clone)]
struct Board {
    squares: [[Square; 8]; 8],
}
#[derive(Copy, Clone)]
enum Square {
    Empty,
    Black,
    White,
}

struct BoardIndex(ux::u3, ux::u3);

const FIRST_INDEX: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
const SECOND_INDEX: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

impl TryInto<BoardIndex> for &str {
    type Error = String;
    fn try_into(self) -> Result<BoardIndex, String> {
        if self.len() != 2 {
            return Err(String::from("Cmd must be two characters"));
        }
        let col: u8 = self
            .chars()
            .nth(0)
            .and_then(|f| -> Option<usize> { FIRST_INDEX.iter().position(|i| f == *i) })
            .ok_or(String::from("Incorrect First Index"))?
            .try_into()
            .unwrap();

        let row: u8 = self
            .chars()
            .nth(1)
            .and_then(|f| SECOND_INDEX.iter().position(|i| f == *i))
            .ok_or(String::from("Incorrect Second Index"))?
            .try_into()
            .unwrap();

        return Ok(BoardIndex(u3::new(col), u3::new(row)));
    }
}

fn icon_from_square(square: Square, is_shaded: bool) -> String {
    match (square, is_shaded) {
        (Square::Empty, true) => String::from("░░░"),
        (Square::Empty, false) => String::from("   "),
        (Square::Black, true) => String::from("░x░"),
        (Square::Black, false) => String::from(" x "),
        (Square::White, true) => String::from("░o░"),
        (Square::White, false) => String::from(" o "),
    }
}

fn is_square_shaded(col: usize, row: usize) -> bool {
    (row % 2 == 0) ^ (col % 2 == 0)
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = self
            .squares
            .iter()
            .enumerate()
            .map(|(col, f)| {
                let s = f
                    .iter()
                    .enumerate()
                    .map(|(row, s)| icon_from_square(*s, is_square_shaded(col, row)))
                    .collect::<Vec<String>>()
                    .join("│")
                    + "│";
                String::from("│") + &s
            })
            .collect::<Vec<String>>()
            .join("\n├───┼───┼───┼───┼───┼───┼───┼───┤\n")
            + "\n└───┴───┴───┴───┴───┴───┴───┴───┘";

        write!(
            f,
            "{}",
            String::from("┌───┬───┬───┬───┬───┬───┬───┬───┐\n") + &board
        )
    }
}

const INITIAL_BOARD: Board = Board {
    squares: [
        [
            Square::Empty,
            Square::White,
            Square::Empty,
            Square::White,
            Square::Empty,
            Square::White,
            Square::Empty,
            Square::White,
        ],
        [
            Square::White,
            Square::Empty,
            Square::White,
            Square::Empty,
            Square::White,
            Square::Empty,
            Square::White,
            Square::Empty,
        ],
        [
            Square::Empty,
            Square::White,
            Square::Empty,
            Square::White,
            Square::Empty,
            Square::White,
            Square::Empty,
            Square::White,
        ],
        [Square::Empty; 8],
        [Square::Empty; 8],
        [
            Square::Black,
            Square::Empty,
            Square::Black,
            Square::Empty,
            Square::Black,
            Square::Empty,
            Square::Black,
            Square::Empty,
        ],
        [
            Square::Empty,
            Square::Black,
            Square::Empty,
            Square::Black,
            Square::Empty,
            Square::Black,
            Square::Empty,
            Square::Black,
        ],
        [
            Square::Black,
            Square::Empty,
            Square::Black,
            Square::Empty,
            Square::Black,
            Square::Empty,
            Square::Black,
            Square::Empty,
        ],
    ],
};

// fn is_move_command(cmd: &str) -> bool{
//     if i.starts_with("move"){
//         if i.split(" ").len() == 3{
//             if i.split(" ")[0] ==
//         }
//     }
//     false
// }

fn excecute_movement(cmd: &str, board: &mut Board) -> Result<(), String> {
    if !cmd.starts_with("move") {
        panic!()
    }
    let split_cmd: Vec<&str> = cmd.split(" ").collect();
    if split_cmd.len() != 3 {
        return Err(String::from(
            "move command must have both an origin and a target",
        ));
    }
    let from_index: BoardIndex = split_cmd[1].try_into()?;
    let to_index: BoardIndex = split_cmd[2].try_into()?;
    board.squares[u8::from(to_index.0) as usize][u8::from(to_index.1) as usize] =
        board.squares[u8::from(from_index.0) as usize][u8::from(from_index.1) as usize];
    board.squares[u8::from(from_index.0) as usize][u8::from(from_index.1) as usize] = Square::Empty;
    Ok(())
}

fn main() {
    let mut board = INITIAL_BOARD;
    println!("{}", board);
    'outer: loop {
        //println!("{}", b);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }

        match input.as_str() {
            "q" => break 'outer,
            i if i.starts_with("move") => {
                if let Err(s) = excecute_movement(i, &mut board) {
                    println!("{}", s);
                } else {
                    println!("{}", board);
                }
            }
            _ => println!("Command Not Yet Supported"),
        }
    }
}
