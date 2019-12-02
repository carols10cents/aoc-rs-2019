use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let answer: i32 = input
        .lines()
        .map(|line| {
            fuel_for_mass_and_fuel(
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

fn fuel_for_mass_and_fuel(mass: i32) -> i32 {
    FuelAccumulator { current_mass: mass }.sum()
}

struct FuelAccumulator {
    current_mass: i32,
}

impl Iterator for FuelAccumulator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_mass = fuel(self.current_mass);
        if self.current_mass > 0 {
            Some(self.current_mass)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test_cases() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn example_fuel_for_mass_and_fuel_test_cases() {
        assert_eq!(fuel_for_mass_and_fuel(14), 2);
        assert_eq!(fuel_for_mass_and_fuel(1969), 966);
        assert_eq!(fuel_for_mass_and_fuel(100756), 50346);
    }
}
