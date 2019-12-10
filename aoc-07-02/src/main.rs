use permute::permute;
use std::error::Error;
use std::fs;
use std::sync::mpsc::{Receiver, Sender};

fn main() -> Result<(), Box<dyn Error>> {
    let program_input = fs::read_to_string("input")?;
    let program: Vec<i32> = program_input
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input should have been a number"))
        .collect();

    Ok(())
}

fn run_intcode(mut program: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) {
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
                output
                    .send(printing_value)
                    .expect("Sender shouldn't be closed");
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
    use std::sync::mpsc::channel;
    use std::thread;

    #[test]
    fn run_intcode_with_channels() {
        let (send_input, receive_input) = channel();
        let (send_output, receive_output) = channel();

        thread::spawn(move || {
            let program = vec![3, 12, 4, 12, 1001, 12, -1, 12, 1005, 12, 2, 99, -1];
            run_intcode(program, receive_input, send_output);
        });

        send_input.send(3).unwrap();

        assert_eq!(receive_output.recv().unwrap(), 3);
        assert_eq!(receive_output.recv().unwrap(), 2);
        assert_eq!(receive_output.recv().unwrap(), 1);
        assert!(receive_output.recv().is_err());
    }

    #[test]
    fn run_intcode_that_expects_multiple_inputs_and_sends_multiple_outputs() {
        let (send_input, receive_input) = channel();
        let (send_output, receive_output) = channel();

        thread::spawn(move || {
            let program = vec![
                3, 18, 3, 19, 1002, 19, 2, 19, 4, 19, 1001, 18, -1, 18, 1005, 18, 2, 99, -1, -2,
            ];
            run_intcode(program, receive_input, send_output);
        });

        // Run loop 3 times
        send_input.send(3).unwrap();

        send_input.send(5).unwrap();
        assert_eq!(receive_output.recv().unwrap(), 10);

        send_input.send(25).unwrap();
        assert_eq!(receive_output.recv().unwrap(), 50);

        send_input.send(1).unwrap();
        assert_eq!(receive_output.recv().unwrap(), 2);

        assert!(receive_output.recv().is_err());
    }

    #[test]
    fn run_chained_channel_intcode_computers() {
        let (send_input1, receive_input1) = channel();
        let (send_output1, receive_input2) = channel();
        let (send_output2, receive_output2) = channel();

        let program1 = vec![
            3, 20, 4, 20, 3, 21, 1002, 21, 2, 21, 4, 21, 1001, 20, -1, 20, 1005, 20, 4, 99, -1, -2,
        ];
        let program2 = program1.clone();

        thread::spawn(move || {
            run_intcode(program1, receive_input1, send_output1);
        });

        thread::spawn(move || {
            run_intcode(program2, receive_input2, send_output2);
        });

        // Run loop 3 times
        send_input1.send(3).unwrap();

        send_input1.send(5).unwrap();
        assert_eq!(receive_output2.recv().unwrap(), 20);

        send_input1.send(25).unwrap();
        assert_eq!(receive_output2.recv().unwrap(), 100);

        send_input1.send(1).unwrap();
        assert_eq!(receive_output2.recv().unwrap(), 4);

        assert!(receive_output2.recv().is_err());
    }
}
