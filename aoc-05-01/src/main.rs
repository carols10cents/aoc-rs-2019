use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
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
            let answer = run_intcode(modified_program);

            if answer[0] == 19690720 {
                println!("noun = {}, verb = {}, answer = {}", noun, verb, 100 * noun + verb);
                break;
            }
        }
    }

    Ok(())
}

fn run_intcode(mut program: Vec<i32>) -> Vec<i32> {
    let mut current_position = 0;
    let mut current_opcode = program[current_position];

    while current_opcode != 99 {
        match current_opcode {
            1 => {
                let output_position = program[current_position + 3] as usize;
                let input_position_1 = program[current_position + 1] as usize;
                let input_position_2 = program[current_position + 2] as usize;
                let answer = program[input_position_1] + program[input_position_2];
                program[output_position] = answer;
            }
            2 => {
                let output_position = program[current_position + 3] as usize;
                let input_position_1 = program[current_position + 1] as usize;
                let input_position_2 = program[current_position + 2] as usize;
                let answer = program[input_position_1] * program[input_position_2];
                program[output_position] = answer;
            }
            other => panic!("Unknown opcode: {}", other),
        }
        current_position += 4;
        current_opcode = program[current_position];
    }

    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_99_ends() {
        let program = vec![99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![99]);
    }

    #[test]
    fn opcode_1_adds() {
        let program = vec![1, 0, 0, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn opcode_2_multiplies() {
        let program = vec![2, 3, 0, 3, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn multiply_and_store_after_program() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn program_keeps_going_if_an_instruction_changes() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn opcode_3_takes_input() {
        let program = vec![3, 0, 99];
        let answer = run_intcode(program, 7);
        assert_eq!(answer, vec![7, 0, 99]);
    }

    #[test]
    #[should_panic(expected = "Unknown opcode: 42")]
    fn unknown_opcode_panics() {
        let program = vec![42];
        run_intcode(program);
    }
}
