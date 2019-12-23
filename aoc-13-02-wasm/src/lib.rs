mod utils;

use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::fmt;

#[wasm_bindgen]
#[repr(u8)]
#[derive(PartialEq, Copy, Clone, Debug, Eq)]
pub enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '*',
            Tile::Paddle => 'T',
            Tile::Ball => 'o',
        }
    }
}

impl From<i64> for Tile {
    fn from(val: i64) -> Self {
        match val {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            other => panic!("Unknown tile: {}", other),
        }
    }
}

#[derive(Copy, Clone)]
enum Joystick {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

impl From<i64> for Joystick {
    fn from(val: i64) -> Self {
        match val {
            0 => Joystick::Neutral,
            -1 => Joystick::Left,
            1 => Joystick::Right,
            other => panic!("Unknown joystick: {}", other),
        }
    }
}

const SCREEN_WIDTH: usize = 36;
const SCREEN_HEIGHT: usize = 24;

#[wasm_bindgen]
pub struct Screen {
    width: usize,
    height: usize,
    data: Vec<Tile>,
    intcode_computer: Computer,
}

#[wasm_bindgen]
impl Screen {
    pub fn new() -> Screen {
        Screen {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            data: vec![Tile::Empty; 36 * 24],
            intcode_computer: Computer::new(),
        }
    }

    pub fn run(&mut self) -> bool {
        self.intcode_computer.run(&mut self.data)
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn score(&self) -> i64 {
        self.intcode_computer.score
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.data.as_slice().chunks(self.width as usize) {
            for &tile in line {
                let symbol = match tile {
                    Tile::Empty => " ",
                    Tile::Wall => "█",
                    Tile::Block => "□",
                    Tile::Paddle => "_",
                    Tile::Ball => "o",
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


struct Computer {
    program: HashMap<usize, i64>,
    current_position: usize,
    relative_base: usize,
    output_x: Option<i64>,
    output_y: Option<i64>,
    score: i64,
    initial_render_complete: bool,
}

impl Computer {
    fn new() -> Computer {
        let program_input = include_str!("../input");
        let program: HashMap<usize, i64> = program_input
            .trim()
            .split(",")
            .map(|n| n.parse().expect("input should have been a number"))
            .enumerate()
            .collect();

        Computer {
            program,
            current_position: 0,
            relative_base: 0,
            output_x: None,
            output_y: None,
            score: 0,
            initial_render_complete: false,
        }
    }

    fn run(&mut self, data: &mut [Tile]) -> bool {
        let mut current_inst = self.current_instruction();

        while current_inst.opcode != 99 {
            match current_inst.opcode {
                1 => {
                    let input1 = self.get_value(0);
                    let input2 = self.get_value(1);
                    let answer = input1 + input2;
                    self.set_value(2, answer);
                    self.current_position += 4;
                }
                2 => {
                    let input1 = self.get_value(0);
                    let input2 = self.get_value(1);
                    let answer = input1 * input2;
                    self.set_value(2, answer);
                    self.current_position += 4;
                }
                3 => {
                    // let direction = getch();
                    //
                    // let value = match direction {
                    //     KEY_LEFT => {
                    //         Joystick::Left
                    //     },
                    //     KEY_RIGHT => {
                    //         Joystick::Right
                    //     },
                    //     _ => {
                    //         Joystick::Neutral
                    //     },
                    // };

                    let value = Joystick::Neutral;

                    self.set_value(0, value as i64);
                    self.current_position += 2;
                }
                4 => {
                    let value = self.get_value(0);
                    self.current_position += 2;

                    match (self.output_x, self.output_y) {
                        (None, None) => {
                            self.output_x = Some(value);
                        }
                        (Some(_), None) => {
                            self.output_y = Some(value);
                        }
                        (Some(-1), Some(0)) => {
                            self.score = value;
                            // mvprintw(LINES() - 1, 0, format!("score = {}", value).as_ref());

                            self.output_x = None;
                            self.output_y = None;
                        }
                        (Some(x), Some(y)) => {
                            let tile_value: Tile = value.into();

                            let index = y as usize * SCREEN_WIDTH + x as usize;
                            data[index] = tile_value;

                            self.output_x = None;
                            self.output_y = None;

                            if self.initial_render_complete {
                                return false;
                            } else {
                                if tile_value == Tile::Paddle {
                                    self.initial_render_complete = true;
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                5 => {
                    // jump-if-true
                    let test_value = self.get_value(0);
                    if test_value != 0 {
                        let jump_location = self.get_value(1);
                        self.current_position = jump_location as usize;
                    } else {
                        self.current_position += 3;
                    }
                }
                6 => {
                    // jump-if-false
                    let test_value = self.get_value(0);
                    if test_value == 0 {
                        let jump_location = self.get_value(1);
                        self.current_position = jump_location as usize;
                    } else {
                        self.current_position += 3;
                    }
                }
                7 => {
                    // less-than
                    let input1 = self.get_value(0);
                    let input2 = self.get_value(1);
                    let answer = if input1 < input2 { 1 } else { 0 };
                    self.set_value(2, answer);
                    self.current_position += 4;
                }
                8 => {
                    // equals
                    let input1 = self.get_value(0);
                    let input2 = self.get_value(1);
                    let answer = if input1 == input2 { 1 } else { 0 };
                    self.set_value(2, answer);
                    self.current_position += 4;
                }
                9 => {
                    // relative base adjustment
                    let input1 = self.get_value(0);
                    let new_rel_base = self.relative_base as i64 + input1;
                    self.relative_base = new_rel_base as usize;
                    self.current_position += 2;
                }
                other => panic!("Unknown opcode: {}", other),
            }
            current_inst = self.current_instruction();
        }

        return true;
    }
}

impl Computer {
    fn current_instruction(&self) -> Instruction {
        instruction(self.read_at(self.current_position))
    }

    fn get_value(&self, parameter_index: usize) -> i64 {
        get_value(&self.program, self.current_position, &self.current_instruction(), parameter_index, self.relative_base)
    }

    fn set_value(&mut self, parameter_index: usize, value: i64) {
        let instruction = self.current_instruction();
        set_value(&mut self.program, self.current_position, &instruction, parameter_index, self.relative_base, value);
    }

    fn read_at(&self, index: usize) -> i64 {
        self.program.get(&index).copied().unwrap_or(0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: i64,
    modes: Vec<Mode>,
}

impl Instruction {
    fn mode(&self, parameter: usize) -> Mode {
        self.modes.get(parameter).copied().unwrap_or(Mode::Position)
    }
}

fn instruction(mut full_opcode: i64) -> Instruction {
    let opcode = full_opcode % 100;
    full_opcode /= 100;

    let mut modes = vec![];

    while full_opcode > 0 {
        let mode = match full_opcode % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            other => panic!("Unexpected parameter mode: {}", other),
        };
        modes.push(mode);
        full_opcode /= 10;
    }

    Instruction { opcode, modes }
}

fn get_value(
    program: &HashMap<usize, i64>,
    instruction_pointer: usize,
    inst: &Instruction,
    parameter_index: usize,
    relative_base: usize,
) -> i64 {
    let parameter_location = instruction_pointer + parameter_index + 1;

    match inst.mode(parameter_index) {
        Mode::Position => {
            let position = program.get(&parameter_location).copied().unwrap_or(0) as usize;
            program.get(&position).copied().unwrap_or(0)
        },
        Mode::Immediate => program.get(&parameter_location).copied().unwrap_or(0),
        Mode::Relative => {
            let offset = program.get(&parameter_location).copied().unwrap_or(0);
            let memory_location = offset + relative_base as i64;
            if memory_location < 0 {
                panic!("Cannot access memory at {}", memory_location);
            }
            program.get(&(memory_location as usize)).copied().unwrap_or(0)
        },
    }
}

fn set_value(
    program: &mut HashMap<usize, i64>,
    instruction_pointer: usize,
    inst: &Instruction,
    parameter_index: usize,
    relative_base: usize,
    value: i64,
) {
    let parameter_location = instruction_pointer + parameter_index + 1;

    match inst.mode(parameter_index) {
        Mode::Position => {
            let position = program.get(&parameter_location).copied().unwrap_or(0) as usize;
            program.insert(position, value);
        },
        Mode::Immediate => unreachable!("Can't set values in immediate mode"),
        Mode::Relative => {
            let offset = program.get(&parameter_location).copied().unwrap_or(0);
            let memory_location = offset + relative_base as i64;
            if memory_location < 0 {
                panic!("Cannot access memory at {}", memory_location);
            }
            program.insert(memory_location as usize, value);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_99_ends() {
        let program = vec![99];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&0], 99);
    }

    #[test]
    fn opcode_1_adds() {
        let program = vec![1, 0, 0, 0, 99];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&0], 2);
        assert_eq!(answer[&4], 99);
    }

    #[test]
    fn opcode_2_multiplies() {
        let program = vec![2, 3, 0, 3, 99];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&3], 6);
    }

    #[test]
    fn multiply_and_store_after_program() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&5], 9801);
    }

    #[test]
    fn program_keeps_going_if_an_instruction_changes() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&0], 30);
        assert_eq!(answer[&4], 2);
    }

    #[test]
    fn opcode_7_less_than() {
        let program = vec![1107, 4, 5, 3, 99];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&3], 1);

        let program = vec![1107, 5, 4, 3, 99];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&3], 0);
    }

    #[test]
    fn opcode_8_equals() {
        let program = vec![1108, 4, 4, 3, 99];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&3], 1);

        let program = vec![1108, 5, 4, 3, 99];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&3], 0);
    }

    #[test]
    #[should_panic(expected = "Unknown opcode: 42")]
    fn unknown_opcode_panics() {
        let program = vec![42];
        Computer::new(program).run();
    }

    #[test]
    fn interpret_parameter_modes_all_position_mode() {
        let inst = instruction(2);
        assert_eq!(inst.opcode, 2);
        assert_eq!(inst.mode(0), Mode::Position);
        assert_eq!(inst.mode(1), Mode::Position);
        assert_eq!(inst.mode(2), Mode::Position);

        let inst = instruction(4);
        assert_eq!(inst.opcode, 4);
        assert_eq!(inst.mode(0), Mode::Position);
        assert_eq!(inst.mode(1), Mode::Position);
        assert_eq!(inst.mode(2), Mode::Position);

        let inst = instruction(99);
        assert_eq!(inst.opcode, 99);
    }

    #[test]
    fn interpret_parameter_modes_that_have_some_immediate_mode() {
        let inst = instruction(104);
        assert_eq!(inst.opcode, 4);
        assert_eq!(inst.mode(0), Mode::Immediate);

        let inst = instruction(1002);
        assert_eq!(inst.opcode, 2);
        assert_eq!(inst.mode(0), Mode::Position);
        assert_eq!(inst.mode(1), Mode::Immediate);
        assert_eq!(inst.mode(2), Mode::Position);

        let inst = instruction(204);
        assert_eq!(inst.opcode, 4);
        assert_eq!(inst.mode(0), Mode::Relative);
    }

    #[test]
    fn use_parameter_modes_in_programs() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut computer = Computer::new(program);
        computer.run();
        let answer = computer.program;
        assert_eq!(answer[&4], 99);
    }

    #[test]
    fn get_value_helper_function() {
        let mut program = HashMap::new();
        program.insert(0, 1);
        program.insert(4, 99);
        let inst = instruction(*program.get(&0).unwrap());
        let instruction_pointer = 0;

        assert_eq!(get_value(&program, instruction_pointer, &inst, 0, 0), 1);

        let mut program = HashMap::new();
        program.insert(0, 104);
        program.insert(1, 18);
        program.insert(2, 99);
        let inst = instruction(*program.get(&0).unwrap());
        assert_eq!(get_value(&program, instruction_pointer, &inst, 0, 0), 18);

        let mut program = HashMap::new();
        program.insert(0, 109);
        program.insert(1, 1);
        program.insert(2, 204);
        program.insert(3, -1);
        let inst = instruction(*program.get(&2).unwrap());
        assert_eq!(get_value(&program, 2, &inst, 0, 1), 109);
    }
}
