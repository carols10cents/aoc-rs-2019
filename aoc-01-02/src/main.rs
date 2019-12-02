use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let answer: i32 = input
        .lines()
        .map(|line| {
            fuel(
                line.parse()
                    .expect("input should have been parsed as a number"),
            )
        })
        .sum();
    println!("the answer is {}", answer);
    Ok(())
}

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::fuel;

    #[test]
    fn example_test_cases() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }
}
