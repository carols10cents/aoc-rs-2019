use permute::permute;
use std::error::Error;
use std::fs;
use std::sync::mpsc::{Sender, Receiver};

fn main() -> Result<(), Box<dyn Error>> {
    let program_input = fs::read_to_string("input")?;
    let program: Vec<i32> = program_input
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input should have been a number"))
        .collect();

    Ok(())
}

fn run_intcode(mut program: Vec<i32>, mut input: Receiver<i32>, output: Sender<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut current_position = 0;
    let mut current_inst = instruction(program[current_position]);

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
                program[output_position] = input
                    .recv()
                    .expect("Should have had enough input for opcode 3");
                current_position += 2;
            }
            4 => {
                let printing_value = get_value(&program, current_position, &current_inst, 0);
                output.send(printing_value).expect("Sender shouldn't be closed");
                current_position += 2;
            }
            5 => {
                // jump-if-true
                let test_value = get_value(&program, current_position, &current_inst, 0);
                if test_value != 0 {
                    let jump_location = get_value(&program, current_position, &current_inst, 1);
                    current_position = jump_location as usize;
                } else {
                    current_position += 3;
                }
            }
            6 => {
                // jump-if-false
                let test_value = get_value(&program, current_position, &current_inst, 0);
                if test_value == 0 {
                    let jump_location = get_value(&program, current_position, &current_inst, 1);
                    current_position = jump_location as usize;
                } else {
                    current_position += 3;
                }
            }
            7 => {
                // less-than
                let output_position = program[current_position + 3] as usize;
                let input1 = get_value(&program, current_position, &current_inst, 0);
                let input2 = get_value(&program, current_position, &current_inst, 1);
                let answer = if input1 < input2 { 1 } else { 0 };
                program[output_position] = answer;
                current_position += 4;
            }
            8 => {
                // equals
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

fn get_value(
    program: &[i32],
    instruction_pointer: usize,
    inst: &Instruction,
    parameter_index: usize,
) -> i32 {
    let parameter_location = instruction_pointer + parameter_index + 1;

    match inst.mode(parameter_index) {
        Mode::Position => program[program[parameter_location] as usize],
        Mode::Immediate => program[parameter_location],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::mpsc::channel;

    #[test]
    fn run_intcode_with_channels() {
        let (send_input, receive_input) = channel();
        let (send_output, receive_output) = channel();

        thread::spawn(move ||  {
            let program = vec![3, 12, 4, 12, 2, 12, 1, 12, 1005, 12, 2, 99, -1];
            run_intcode(program, receive_input, send_output);
        });

        send_input.send(3).unwrap();

        assert_eq!(receive_output.recv().unwrap(), 3);
        assert_eq!(receive_output.recv().unwrap(), 2);
        assert_eq!(receive_output.recv().unwrap(), 1);
        assert!(receive_output.recv().is_err());

        // Create a simple streaming channel
        let (tx, rx) = channel();
        thread::spawn(move|| {
            tx.send(10).unwrap();
        });
        assert_eq!(rx.recv().unwrap(), 10);
    }
}
