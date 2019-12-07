use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let program: Vec<_> = input
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input should have been a number"))
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut modified_program = program.clone();
            modified_program[1] = noun;
            modified_program[2] = verb;
            let (answer, _output) = run_intcode(modified_program, None);

            if answer[0] == 19690720 {
                println!(
                    "noun = {}, verb = {}, answer = {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                break;
            }
        }
    }

    Ok(())
}

fn run_intcode(mut program: Vec<i32>, input: Option<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut current_position = 0;
    let mut current_opcode = program[current_position];
    let mut output = vec![];

    while current_opcode != 99 {
        match current_opcode {
            1 => {
                let output_position = program[current_position + 3] as usize;
                let input_position_1 = program[current_position + 1] as usize;
                let input_position_2 = program[current_position + 2] as usize;
                let answer = program[input_position_1] + program[input_position_2];
                program[output_position] = answer;
                current_position += 4;
            }
            2 => {
                let output_position = program[current_position + 3] as usize;
                let input_position_1 = program[current_position + 1] as usize;
                let input_position_2 = program[current_position + 2] as usize;
                let answer = program[input_position_1] * program[input_position_2];
                program[output_position] = answer;
                current_position += 4;
            }
            3 => {
                let output_position = program[current_position + 1] as usize;
                program[output_position] = input.expect("Should have had input for opcode 3");
                current_position += 2;
            }
            4 => {
                let printing_position = program[current_position + 1] as usize;
                output.push(program[printing_position]);
                current_position += 2;
            }
            other => panic!("Unknown opcode: {}", other),
        }
        current_opcode = program[current_position];
    }

    (program, output)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: i32,
    modes: Vec<Mode>,
}

impl Instruction {
    fn mode(&self, parameter: usize) -> Mode {
        self.modes.get(parameter).copied().unwrap_or(Mode::Position)
    }
}

fn instruction(mut full_opcode: i32) -> Instruction {
    let opcode = full_opcode % 100;
    full_opcode /= 100;

    let mut modes = vec![];

    while full_opcode > 0 {
        let mode = match full_opcode % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            other => panic!("Unexpected parameter mode: {}", other),
        };
        modes.push(mode);
        full_opcode /= 10;
    }

    Instruction { opcode, modes }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_99_ends() {
        let program = vec![99];
        let (answer, _output) = run_intcode(program, None);
        assert_eq!(answer, vec![99]);
    }

    #[test]
    fn opcode_1_adds() {
        let program = vec![1, 0, 0, 0, 99];
        let (answer, _output) = run_intcode(program, None);
        assert_eq!(answer, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn opcode_2_multiplies() {
        let program = vec![2, 3, 0, 3, 99];
        let (answer, _output) = run_intcode(program, None);
        assert_eq!(answer, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn multiply_and_store_after_program() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let (answer, _output) = run_intcode(program, None);
        assert_eq!(answer, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn program_keeps_going_if_an_instruction_changes() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let (answer, _output) = run_intcode(program, None);
        assert_eq!(answer, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn opcode_3_takes_input() {
        let program = vec![3, 0, 99];
        let (answer, _output) = run_intcode(program, Some(7));
        assert_eq!(answer, vec![7, 0, 99]);
    }

    #[test]
    fn opcode_4_returns_output() {
        let program = vec![4, 2, 99];
        let (_answer, output) = run_intcode(program, None);
        assert_eq!(output, vec![99]);
    }

    #[test]
    #[should_panic(expected = "Unknown opcode: 42")]
    fn unknown_opcode_panics() {
        let program = vec![42];
        run_intcode(program, None);
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
    }

    #[test]
    fn use_parameter_modes_in_programs() {
        let program = vec![1002, 4, 3, 4, 33];
        let (answer, _output) = run_intcode(program, None);
        assert_eq!(answer, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn get_value_helper_function() {
        let program = vec![1, 0, 0, 0, 99];
        let instruction_pointer = 0;

        assert_eq!(get_value(&program, instruction_pointer, 0), 1);
    }
}

fn get_value(program: &[i32], instruction_pointer: usize, parameter_index: usize) -> i32 {
    let opcode = program[instruction_pointer];
    let inst = instruction(opcode);
    match inst.mode(parameter_index) {
        Mode::Position => program[program[instruction_pointer + parameter_index + 1] as usize],
        Mode::Immediate => unimplemented!(),
    }
}
