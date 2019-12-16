use std::error::Error;
use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let program_input = fs::read_to_string("input")?;
    let program: Vec<_> = program_input
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input should have been a number"))
        .collect();

    let simulated_stdin = Some(2);
    let (_answer, output) = Computer::new(program, simulated_stdin).run();
    println!("{:?}", output);

    Ok(())
}

enum Color {
    Black = 0,
    White = 1,
}

struct Computer {
    program: HashMap<usize, i64>,
    output: Vec<i64>,
    current_position: usize,
    relative_base: usize,
    input: Option<i64>,
}

impl Computer {
    fn new(program: Vec<i64>, input: Option<i64>) -> Computer {
        let program: HashMap<usize, i64> = program.into_iter().enumerate().collect();

        Computer {
            program,
            output: vec![],
            current_position: 0,
            relative_base: 0,
            input,
        }
    }

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

    fn current_square_color(&self) -> Color {
        Color::Black
    }

    fn run(&mut self) -> (HashMap<usize, i64>, Vec<i64>) {
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
                    let value = self.current_square_color() as i64;
                    self.set_value(0, value);
                    self.current_position += 2;
                }
                4 => {
                    let printing_value = self.get_value(0);
                    self.output.push(printing_value);
                    self.current_position += 2;
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

        (self.program.clone(), self.output.clone())
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
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&0], 99);
    }

    #[test]
    fn opcode_1_adds() {
        let program = vec![1, 0, 0, 0, 99];
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&0], 2);
        assert_eq!(answer[&4], 99);
    }

    #[test]
    fn opcode_2_multiplies() {
        let program = vec![2, 3, 0, 3, 99];
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&3], 6);
    }

    #[test]
    fn multiply_and_store_after_program() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&5], 9801);
    }

    #[test]
    fn program_keeps_going_if_an_instruction_changes() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&0], 30);
        assert_eq!(answer[&4], 2);
    }

    #[test]
    fn opcode_3_takes_input() {
        let program = vec![3, 0, 99];
        let (answer, _output) = Computer::new(program, Some(7)).run();
        assert_eq!(answer[&0], Color::Black as i64);
    }

    #[test]
    fn opcode_4_returns_output() {
        let program = vec![4, 2, 99];
        let (_answer, output) = Computer::new(program, None).run();
        assert_eq!(output, vec![99]);
    }

    #[test]
    fn opcode_5_jumps_if_true() {
        // Test value is false; 42 gets printed
        let program = vec![1005, 6, 5, 104, 42, 99, 0];
        let (_answer, output) = Computer::new(program, None).run();
        assert_eq!(output, vec![42]);

        // Test value is true; print gets jumped over
        let program = vec![1005, 6, 5, 104, 42, 99, 3];
        let (_answer, output) = Computer::new(program, None).run();
        assert_eq!(output, vec![]);
    }

    #[test]
    fn opcode_6_jumps_if_false() {
        // Test value is false; print gets jumped over
        let program = vec![1006, 6, 5, 104, 42, 99, 0];
        let (_answer, output) = Computer::new(program, None).run();
        assert_eq!(output, vec![]);

        // Test value is true; 42 gets printed
        let program = vec![1006, 6, 5, 104, 42, 99, 3];
        let (_answer, output) = Computer::new(program, None).run();
        assert_eq!(output, vec![42]);
    }

    #[test]
    fn opcode_7_less_than() {
        let program = vec![1107, 4, 5, 3, 99];
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&3], 1);

        let program = vec![1107, 5, 4, 3, 99];
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&3], 0);
    }

    #[test]
    fn opcode_8_equals() {
        let program = vec![1108, 4, 4, 3, 99];
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&3], 1);

        let program = vec![1108, 5, 4, 3, 99];
        let (answer, _output) = Computer::new(program, None).run();
        assert_eq!(answer[&3], 0);
    }

    #[test]
    #[should_panic(expected = "Unknown opcode: 42")]
    fn unknown_opcode_panics() {
        let program = vec![42];
        Computer::new(program, None).run();
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
        let (answer, _output) = Computer::new(program, None).run();
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

    #[test]
    fn stress_tests() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let (_answer, output) = Computer::new(program, None).run();
        assert_eq!(output, vec![1219070632396864]);

        let program = vec![104, 1125899906842624, 99];
        let (_answer, output) = Computer::new(program, None).run();
        assert_eq!(output, vec![1125899906842624]);

        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let (_answer, output) = Computer::new(program, None).run();
        assert_eq!(
            output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }
}
