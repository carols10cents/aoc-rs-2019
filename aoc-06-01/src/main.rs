fn main() {
    println!("Hello, world!");
}

fn orbit_count(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_direct_orbit_only() {
        let input = "COM)A";
        let total = orbit_count(input);
        assert_eq!(total, 1);
    }
}
