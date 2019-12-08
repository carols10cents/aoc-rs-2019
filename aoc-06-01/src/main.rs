use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn orbit_count(input: &str) -> usize {
    let mut orbits = HashMap::new();

    for line in input.lines() {
        let bodies = line.split(")");

        let orbited = bodies.next().expect("Must have a body being orbited");
        let orbiting = bodies.next().expect("Must have a body doing the orbiting");

        orbits.insert(orbiting, orbited);
    }

    let mut total = 0;
    for body in orbits.keys() {
        let mut current = body;
        while current != "COM" {
            total += 1;
            current = orbits.get(current).expect("Must be orbiting something");
        }
    }
    total
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
