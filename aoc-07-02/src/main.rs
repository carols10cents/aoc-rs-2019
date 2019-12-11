use permute::permute;
use std::error::Error;
use std::fs;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let program_input = fs::read_to_string("input")?;
    let program: Vec<i32> = program_input
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input should have been a number"))
        .collect();

    let phase_settings = [5, 6, 7, 8, 9];
    let max = max_signal(&program, &phase_settings);

    println!("{:?}", max);

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
    fn run_looped_channel_intcode_computers() {
        let (send_input_original, receive_input1) = channel();
        let (send_output1, receive_input2) = channel();
        let (send_output2, receive_output_spy) = channel();

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

        // Run loop 3 times, which also sends 3 around to be doubled
        send_input_original.send(3).unwrap();

        let mut final_value = -1;

        while let Ok(received_value) = receive_output_spy.recv() {
            let _ = send_input_original.send(received_value);
            final_value = received_value;
        }

        assert_eq!(final_value, 192);
    }

    #[test]
    fn run_example_in_looped_incode_computers() {
        let phase_settings = [5, 6, 7, 8, 9];

        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let answer = max_signal(&program, &phase_settings);
        assert_eq!(answer, 139629729);

        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let answer = max_signal(&program, &phase_settings);
        assert_eq!(answer, 18216);
    }
}

fn max_signal(program: &[i32], phase_settings: &[i32]) -> i32 {
    permute(phase_settings.to_owned())
        .iter()
        .map(|setting_ordering| run_with_phase_settings(program, setting_ordering))
        .max()
        .expect("Must have had orderings")
}

fn run_with_phase_settings(program: &[i32], phase_settings: &[i32]) -> i32 {
    let (send_from_main, receive_in_amp_a) = channel();
    let (send_from_amp_a, receive_in_amp_b) = channel();
    let (send_from_amp_b, receive_in_amp_c) = channel();
    let (send_from_amp_c, receive_in_amp_d) = channel();
    let (send_from_amp_d, receive_in_amp_e) = channel();
    let (send_from_amp_e, receive_in_main) = channel();

    // Send phase settings
    send_from_main.send(phase_settings[0]).unwrap();
    send_from_amp_a.send(phase_settings[1]).unwrap();
    send_from_amp_b.send(phase_settings[2]).unwrap();
    send_from_amp_c.send(phase_settings[3]).unwrap();
    send_from_amp_d.send(phase_settings[4]).unwrap();

    // Set up threads
    let program_a = program.to_vec();
    thread::spawn(move || {
        run_intcode(program_a, receive_in_amp_a, send_from_amp_a);
    });

    let program_b = program.to_vec();
    thread::spawn(move || {
        run_intcode(program_b, receive_in_amp_b, send_from_amp_b);
    });

    let program_c = program.to_vec();
    thread::spawn(move || {
        run_intcode(program_c, receive_in_amp_c, send_from_amp_c);
    });

    let program_d = program.to_vec();
    thread::spawn(move || {
        run_intcode(program_d, receive_in_amp_d, send_from_amp_d);
    });

    let program_e = program.to_vec();
    thread::spawn(move || {
        run_intcode(program_e, receive_in_amp_e, send_from_amp_e);
    });

    // Send initial signal
    send_from_main.send(0).unwrap();

    // Loop until feedback stops
    let mut final_value = -1;

    while let Ok(received_value) = receive_in_main.recv() {
        let _ = send_from_main.send(received_value);
        final_value = received_value;
    }

    final_value
}
