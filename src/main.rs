use std::{convert::TryInto, fmt, ops::Not, usize};
use ux::*;

struct Board {
    squares: BoardSquare,
    current_square: SquareColor,
}

struct BoardSquare([[Option<(SquareColor, SquareType)>; 8]; 8]);

#[derive(Copy, Clone, PartialEq)]
enum SquareType {
    Single,
    Double,
}

#[derive(Copy, Clone, PartialEq)]
enum SquareColor {
    White,
    Black,
}

impl Not for SquareColor {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            SquareColor::Black => SquareColor::White,
            SquareColor::White => SquareColor::Black,
        }
    }
}

impl BoardSquare {
    fn usize_form_of_index(index: BoardIndex) -> (usize, usize) {
        let i_0 = u8::from(index.0) as usize;
        let i_1 = u8::from(index.1) as usize;
        (i_0, i_1)
    }
    fn get(&self, index: BoardIndex) -> Option<(SquareColor, SquareType)> {
        let (i_0, i_1) = BoardSquare::usize_form_of_index(index);
        self.0[i_0][i_1]
    }
    fn set(&mut self, index: BoardIndex, val: Option<(SquareColor, SquareType)>) -> () {
        let (i_0, i_1) = BoardSquare::usize_form_of_index(index);
        self.0[i_0][i_1] = val;
        ()
    }
}

impl Board {
    fn try_select_intitial_square(
        &mut self,
        from_index: BoardIndex,
    ) -> Result<(SquareColor, SquareType), String> {
        let from_square = self
            .squares
            .get(from_index)
            .ok_or(String::from("No Piece In This Index"))?;
        if from_square.0 != self.current_square {
            return Err(format!(
                "Not Possible to Controll Piece at {}",
                String::from(from_index)
            ));
        }
        Ok(from_square)
    }
    fn try_select_final_square(&mut self, to_index: BoardIndex) -> Result<(), String> {
        if self.squares.get(to_index).is_some() {
            return Err(String::from("Final Square Is Not Empty"));
        }
        Ok(())
    }

    fn get_displacement(from_index: BoardIndex, to_index: BoardIndex) -> (i32, i32) {
        let from_0 = u8::from(from_index.0) as i32;
        let from_1 = u8::from(from_index.1) as i32;
        let to_0 = u8::from(to_index.0) as i32;
        let to_1 = u8::from(to_index.1) as i32;

        (to_0 - from_0, to_1 - from_1)
    }
    fn try_perform_step(
        &mut self,
        from_index: BoardIndex,
        to_index: BoardIndex,
    ) -> Result<(), (bool, String)> {
        let from_square = self
            .try_select_intitial_square(from_index)
            .or_else(|s| Err((false, s)))?;
        let (d0, d1) = Board::get_displacement(from_index, to_index);
        let is_forward = (from_square.0 == SquareColor::Black) ^ (d0 > 0);
        if !is_forward && (from_square.1 == SquareType::Single) {
            if from_square.1 == SquareType::Single {
                return Err((
                    false,
                    format!(
                        "Cannot Step Backwards from {} to {}",
                        String::from(from_index),
                        String::from(to_index)
                    ),
                ));
            }
        }

        match (d0, d1) {
            (d0, d1) if (d0.abs() == 1) && (d1.abs() == 1) => {}
            _ => {
                return Err((
                    true,
                    format!(
                        "Not possible to step from {} to {}",
                        String::from(from_index),
                        String::from(to_index)
                    ),
                ));
            }
        }
        Board::try_select_final_square(self, to_index).or_else(|s| Err((false, s)))?;
        self.squares.set(to_index, Some(from_square));
        self.squares.set(from_index, None);
        Ok(())
    }
    fn try_perform_hop(
        &mut self,
        from_index: BoardIndex,
        to_index: BoardIndex,
    ) -> Result<(), String> {
        let from_square = self.try_select_intitial_square(from_index)?;
        let (d0, d1) = Board::get_displacement(from_index, to_index);

        let from_0 = u8::from(from_index.0) as i32;
        let from_1 = u8::from(from_index.1) as i32;

        let is_forward = (from_square.0 == SquareColor::Black) ^ (d0 > 0);
        if !is_forward && (from_square.1 == SquareType::Single) {
            if from_square.1 == SquareType::Single {
                return Err(format!(
                    "Cannot Hop Backwards from {} to {}",
                    String::from(from_index),
                    String::from(to_index)
                ));
            }
        }

        match (d0, d1) {
            (d0, d1) if (d0.abs() == 2) && (d1.abs() == 2) => {
                let middle_index = BoardIndex(
                    u3::new((from_0 + d0 / 2).try_into().unwrap()),
                    u3::new((from_1 + d1 / 2).try_into().unwrap()),
                );
                if let Some((c, _)) = self.squares.get(middle_index) {
                    if c == !self.current_square {
                        self.squares.set(middle_index, None)
                    } else {
                        return Err(format!(
                            "Cannot hop from {} To {}, is black {}",
                            String::from(from_index),
                            String::from(to_index),
                            c == SquareColor::Black
                        ));
                    }
                } else {
                    return Err(format!(
                        "Cannot hop from {} To {}",
                        String::from(to_index),
                        String::from(from_index)
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "Not possible to hop from {} to {}",
                    String::from(from_index),
                    String::from(to_index)
                ));
            }
        }
        Board::try_select_final_square(self, to_index)?;
        self.squares.set(to_index, Some(from_square));
        self.squares.set(from_index, None);
        Ok(())
    }
    fn try_move_single(
        &mut self,
        from_index: BoardIndex,
        to_index: BoardIndex,
    ) -> Result<(), String> {
        self.try_perform_step(from_index, to_index)
            .or_else(|(should_try_hop, s)| {
                if should_try_hop {
                    return self.try_perform_hop(from_index, to_index);
                }
                return Err(s);
            })?;
        self.end_turn();
        Ok(())
    }
    fn try_move_multiple(
        &mut self,
        mut from_index: BoardIndex,
        to_indexes: Vec<BoardIndex>,
    ) -> Result<(), String> {
        for to_index in to_indexes.iter() {
            self.try_perform_hop(from_index, *to_index)?;
            from_index = *to_index;
        }
        self.end_turn();
        Ok(())
    }
    fn end_turn(&mut self) -> () {
        self.current_square = !self.current_square;
    }
    fn try_excecute_move_command(&mut self, cmd: &str) -> Result<(), String> {
        println!("{}", cmd);
        let mut split_cmd = cmd.split("to");
        let from_index: BoardIndex = split_cmd
            .next()
            .ok_or(String::from("no index to move from"))?
            .try_into()?;
        println!("{}, {}", from_index.0, from_index.1);
        let to_indexes: Vec<BoardIndex> = split_cmd
            .next()
            .ok_or(String::from("no index to move to"))?
            .split("then")
            .map(|s| s.try_into())
            .collect::<Result<Vec<BoardIndex>, String>>()?;
        match to_indexes.len() {
            0 => {
                return Err(String::from("no index to move to"));
            }
            1 => {
                self.try_move_single(from_index, to_indexes[0])?;
            }
            _ => {
                self.try_move_multiple(from_index, to_indexes)?;
            }
        }
        Ok(())
    }
    fn try_excecute_double_command(&mut self, cmd: &str) -> Result<(), String> {
        println!("{}", cmd);
        let index: BoardIndex = cmd.try_into()?;

        println!("{}, {}", index.0, index.1);
        let initial_square = self.try_select_intitial_square(index)?;
        if initial_square.1 == SquareType::Double {
            return Err(String::from("Unable to Double A Double"));
        }
        match (u8::from(index.0), self.current_square) {
            (0, SquareColor::Black) => {}
            (7, SquareColor::White) => {}
            _ => {
                return Err(format!(
                    "Unable to Double in position {}",
                    String::from(index)
                ));
            }
        }
        self.squares
            .set(index, Some((self.current_square, SquareType::Double)));
        Ok(())
    }
}

fn generate_initial_row(
    filled_with: SquareColor,
    row_number: u8,
) -> [Option<(SquareColor, SquareType)>; 8] {
    let mut row = [None; 8];
    for x in 0..8 {
        if x % 2 != (row_number % 2) as usize {
            row[x] = Some((filled_with, SquareType::Single));
        }
    }
    row
}
impl Default for Board {
    fn default() -> Self {
        Board {
            squares: BoardSquare([
                generate_initial_row(SquareColor::White, 0),
                generate_initial_row(SquareColor::White, 1),
                generate_initial_row(SquareColor::White, 2),
                [None; 8],
                [None; 8],
                generate_initial_row(SquareColor::Black, 5),
                generate_initial_row(SquareColor::Black, 6),
                generate_initial_row(SquareColor::Black, 7),
            ]),
            current_square: SquareColor::White,
        }
    }
}

#[derive(Copy, Clone)]
struct BoardIndex(ux::u3, ux::u3);

const FIRST_INDEX: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
const SECOND_INDEX: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

impl TryInto<BoardIndex> for &str {
    type Error = String;
    fn try_into(self) -> Result<BoardIndex, String> {
        let stripped: String = self.to_uppercase().split_whitespace().collect();
        if stripped.len() != 2 {
            return Err(String::from("Cmd must be two characters"));
        }
        let col: u8 = stripped
            .chars()
            .nth(0)
            .and_then(|f| -> Option<usize> { FIRST_INDEX.iter().position(|i| f == *i) })
            .ok_or(String::from("Incorrect First Index"))?
            .try_into()
            .unwrap();

        let row: u8 = stripped
            .chars()
            .nth(1)
            .and_then(|f| SECOND_INDEX.iter().position(|i| f == *i))
            .ok_or(String::from("Incorrect Second Index"))?
            .try_into()
            .unwrap();

        return Ok(BoardIndex(u3::new(col), u3::new(row)));
    }
}

impl From<BoardIndex> for String {
    fn from(index: BoardIndex) -> String {
        return format!(
            "{}{}",
            FIRST_INDEX[u8::from(index.0) as usize],
            SECOND_INDEX[u8::from(index.1) as usize]
        );
    }
}

fn icon_from_square(square: Option<(SquareColor, SquareType)>, is_shaded: bool) -> String {
    match (square, is_shaded) {
        (None, true) => String::from("░░░"),
        (None, false) => String::from("   "),
        (Some((SquareColor::Black, SquareType::Single)), true) => String::from("░x░"),
        (Some((SquareColor::Black, SquareType::Single)), false) => String::from(" x "),
        (Some((SquareColor::Black, SquareType::Double)), true) => String::from("░X░"),
        (Some((SquareColor::Black, SquareType::Double)), false) => String::from(" X "),
        (Some((SquareColor::White, SquareType::Single)), true) => String::from("░o░"),
        (Some((SquareColor::White, SquareType::Single)), false) => String::from(" o "),
        (Some((SquareColor::White, SquareType::Double)), true) => String::from("░O░"),
        (Some((SquareColor::White, SquareType::Double)), false) => String::from(" O "),
    }
}

fn is_square_shaded(col: usize, row: usize) -> bool {
    (row % 2 == 0) ^ (col % 2 == 0)
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = self
            .squares.0
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
                format!("│ {} │", FIRST_INDEX[col]) + &s
            })
            .collect::<Vec<String>>()
            .join("\n├───┼───┼───┼───┼───┼───┼───┼───┼───┤\n")
            + "\n└───┼───┼───┼───┼───┼───┼───┼───┼───┤\n    │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │ 8 │\n    └───┴───┴───┴───┴───┴───┴───┴───┘";

        write!(
            f,
            "{}",
            String::from("┌───┬───┬───┬───┬───┬───┬───┬───┬───┐\n") + &board
        )
    }
}

fn excecute_command(cmd: &str, board: &mut Board) -> bool {
    match cmd {
        "q" => {
            return true;
        }
        i if i.starts_with("move") => {
            if let Err(s) = board.try_excecute_move_command(i.get(5..).unwrap_or("")) {
                println!("{}", s);
            } else {
                println!("{}", board);
            }
        }
        i if i.starts_with("double") => {
            if let Err(s) = board.try_excecute_double_command(i.get(7..).unwrap_or("")) {
                println!("{}", s);
            } else {
                println!("{}", board);
            }
        }
        _ => println!("Command Not Yet Supported"),
    };
    false
}
fn main() {
    let mut board = Board::default();
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
        if true == excecute_command(&input, &mut board) {
            break 'outer;
        }
    }
}

#[test]
fn test_case_1() {
    let mut board = Board::default();
    let commands = [
        "move c2 to d1",
        "move f1 to e2",
        "move b1 to c2",
        "move f3 to e4",
        "move d1 to f3 then d5",
        "move f5 to e4",
        "move d5 to f3",
        "move g2 to f1",
        "move c8 to D7",
        "move h1 to g2",
        "move f3 to h1",
        "move f1 to e2",
        "double h1",
    ];
    for cmd in commands {
        excecute_command(cmd, &mut board);
    }
}
