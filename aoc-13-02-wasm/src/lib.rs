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

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum Joystick {
    Left = 0,
    Neutral = 1,
    Right = 2,
}

impl Joystick {
    fn as_intcode_value(&self) -> i64 {
        *self as i64 - 1
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
            data: vec![Tile::Empty; SCREEN_WIDTH * SCREEN_HEIGHT],
            intcode_computer: Computer::new(),
        }
    }

    pub fn run(&mut self) -> bool {
        self.intcode_computer.run(&mut self.data)
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn tiles(&self) -> *const Tile {
        self.data.as_ptr()
    }

    pub fn score(&self) -> i64 {
        self.intcode_computer.score
    }

    pub fn set_joystick(&mut self, joystick: Joystick) {
        self.intcode_computer.set_joystick(joystick);
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
    joystick: Joystick,
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
            joystick: Joystick::Neutral,
        }
    }

    fn set_joystick(&mut self, joystick: Joystick) {
        self.joystick = joystick;
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
                    self.set_value(0, self.joystick.as_intcode_value());
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
                                if index == data.len() - 1 {
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
