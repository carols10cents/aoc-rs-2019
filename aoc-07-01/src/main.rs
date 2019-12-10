use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let program_input = fs::read_to_string("input")?;
    let program: Vec<_> = program_input
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input should have been a number"))
        .collect();

    let simulated_stdin = vec![5];
    let (_answer, output) = run_intcode(program, simulated_stdin);
    println!("{:?}", output);

    Ok(())
}

fn run_intcode(mut program: Vec<i32>, mut input: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut current_position = 0;
    let mut current_inst = instruction(program[current_position]);
    let mut output = vec![];
    input = input.into_iter().rev().collect();

    while current_inst.opcode != 99 {
        match current_inst.opcode {
            1 => {
                let output_position = program[current_position + 3] as usize;
                let input1 = get_value(&program, current_position, &current_inst, 0);
                let input2 = get_value(&program, current_position, &current_inst, 1);
                let answer = input1 + input2;
                program[output_position] = answer;
                current_position += 4;
            }
            2 => {
                let output_position = program[current_position + 3] as usize;
                let input1 = get_value(&program, current_position, &current_inst, 0);
                let input2 = get_value(&program, current_position, &current_inst, 1);
                let answer = input1 * input2;
                program[output_position] = answer;
                current_position += 4;
            }
            3 => {
                let output_position = program[current_position + 1] as usize;
                program[output_position] = input.pop().expect("Should have had enough input for opcode 3");
                current_position += 2;
            }
            4 => {
                let printing_value = get_value(&program, current_position, &current_inst, 0);
                output.push(printing_value);
                current_position += 2;
            }
            5 => { // jump-if-true
                let test_value = get_value(&program, current_position, &current_inst, 0);
                if test_value != 0 {
                    let jump_location = get_value(&program, current_position, &current_inst, 1);
                    current_position = jump_location as usize;
                } else {
                    current_position += 3;
                }
            }
            6 => { // jump-if-false
                let test_value = get_value(&program, current_position, &current_inst, 0);
                if test_value == 0 {
                    let jump_location = get_value(&program, current_position, &current_inst, 1);
                    current_position = jump_location as usize;
                } else {
                    current_position += 3;
                }
            }
            7 => { // less-than
                let output_position = program[current_position + 3] as usize;
                let input1 = get_value(&program, current_position, &current_inst, 0);
                let input2 = get_value(&program, current_position, &current_inst, 1);
                let answer = if input1 < input2 { 1 } else { 0 };
                program[output_position] = answer;
                current_position += 4;
            }
            8 => { // equals
                let output_position = program[current_position + 3] as usize;
                let input1 = get_value(&program, current_position, &current_inst, 0);
                let input2 = get_value(&program, current_position, &current_inst, 1);
                let answer = if input1 == input2 { 1 } else { 0 };
                program[output_position] = answer;
                current_position += 4;
            }
            other => panic!("Unknown opcode: {}", other),
        }
        current_inst = instruction(program[current_position]);
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

fn get_value(program: &[i32], instruction_pointer: usize, inst: &Instruction, parameter_index: usize) -> i32 {
    let parameter_location = instruction_pointer + parameter_index + 1;

    match inst.mode(parameter_index) {
        Mode::Position => program[program[parameter_location] as usize],
        Mode::Immediate => program[parameter_location],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_99_ends() {
        let program = vec![99];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![99]);
    }

    #[test]
    fn opcode_1_adds() {
        let program = vec![1, 0, 0, 0, 99];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn opcode_2_multiplies() {
        let program = vec![2, 3, 0, 3, 99];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn multiply_and_store_after_program() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn program_keeps_going_if_an_instruction_changes() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn opcode_3_takes_input() {
        let program = vec![3, 0, 99];
        let (answer, _output) = run_intcode(program, vec![7]);
        assert_eq!(answer, vec![7, 0, 99]);
    }

    #[test]
    fn programs_can_have_arbitrary_numbers_of_opcode_3_with_enough_input() {
        let program = vec![3, 0, 3, 1, 99];
        let (answer, _output) = run_intcode(program, vec![7, 8]);
        assert_eq!(answer, vec![7, 8, 3, 1, 99]);
    }

    #[test]
    #[should_panic(expected = "Should have had enough input for opcode 3")]
    fn programs_panics_if_opcode_3_doesnt_have_input() {
        let program = vec![3, 0, 3, 1, 99];
        run_intcode(program, vec![7]);
    }

    #[test]
    fn opcode_4_returns_output() {
        let program = vec![4, 2, 99];
        let (_answer, output) = run_intcode(program, vec![]);
        assert_eq!(output, vec![99]);
    }

    #[test]
    fn opcode_5_jumps_if_true() {
        // Test value is false; 42 gets printed
        let program = vec![1005, 6, 5, 104, 42, 99, 0];
        let (_answer, output) = run_intcode(program, vec![]);
        assert_eq!(output, vec![42]);

        // Test value is true; print gets jumped over
        let program = vec![1005, 6, 5, 104, 42, 99, 3];
        let (_answer, output) = run_intcode(program, vec![]);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn opcode_6_jumps_if_false() {
        // Test value is false; print gets jumped over
        let program = vec![1006, 6, 5, 104, 42, 99, 0];
        let (_answer, output) = run_intcode(program, vec![]);
        assert_eq!(output, vec![]);

        // Test value is true; 42 gets printed
        let program = vec![1006, 6, 5, 104, 42, 99, 3];
        let (_answer, output) = run_intcode(program, vec![]);
        assert_eq!(output, vec![42]);
    }

    #[test]
    fn opcode_7_less_than() {
        let program = vec![1107, 4, 5, 3, 99];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![1107, 4, 5, 1, 99]);

        let program = vec![1107, 5, 4, 3, 99];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![1107, 5, 4, 0, 99]);
    }

    #[test]
    fn opcode_8_equals() {
        let program = vec![1108, 4, 4, 3, 99];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![1108, 4, 4, 1, 99]);

        let program = vec![1108, 5, 4, 3, 99];
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![1108, 5, 4, 0, 99]);
    }

    #[test]
    #[should_panic(expected = "Unknown opcode: 42")]
    fn unknown_opcode_panics() {
        let program = vec![42];
        run_intcode(program, vec![]);
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
        let (answer, _output) = run_intcode(program, vec![]);
        assert_eq!(answer, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn get_value_helper_function() {
        let program = vec![1, 0, 0, 0, 99];
        let inst = instruction(program[0]);
        let instruction_pointer = 0;

        assert_eq!(get_value(&program, instruction_pointer, &inst, 0), 1);

        let program = vec![104, 18, 99];
        let inst = instruction(program[0]);
        assert_eq!(get_value(&program, instruction_pointer, &inst, 0), 18);
    }

    #[test]
    fn chain_intcode_runs() {
        let program = vec![3, 11, 3, 12, 1, 11, 12, 13, 4, 13, 99, -1, -2, -3];
        let phase_settings = vec![20, 22, 24];
        let output = run_with_phase_settings(&program, &phase_settings);
        assert_eq!(output, 66);
    }
}

fn run_with_phase_settings(program: &[i32], phase_settings: &[i32]) -> i32 {
    let mut carried_input = 0;

    for setting in phase_settings {
        let inputs = vec![setting, carried_input];

        let (_answer, output) = run_intcode(program.to_owned(), inputs);
        carried_input = output.first().expect("Program must return some output");
    }

    carried_input
}
