fn main() {
    println!("Hello, world!");
}

fn run_intcode(program: Vec<i32>) -> Vec<i32> {
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
}