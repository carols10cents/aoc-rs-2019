fn main() {
    println!("Hello, world!");
}

fn run_intcode(mut program: Vec<i32>) -> Vec<i32> {
    let mut current_position = 0;
    let mut current_opcode = program[current_position];

    while current_opcode != 99 {
        match current_opcode {
            1 => {
                program[program[current_position + 3] as usize] =
                    program[current_position + 1] + program[current_position + 2]
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
    #[should_panic(expected = "Unknown opcode: 42")]
    fn unknown_opcode_panics() {
        let program = vec![42];
        run_intcode(program);
    }
}
