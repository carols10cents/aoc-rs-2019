use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("{}", orbit_count(input));
}

fn orbit_count(input: &str) -> usize {
    let mut orbits = HashMap::new();

    for line in input.lines() {
        let mut bodies = line.split(")");

        let orbited = bodies.next().expect("Must have a body being orbited");
        let orbiting = bodies.next().expect("Must have a body doing the orbiting");

        orbits.insert(orbiting, orbited);
    }

    let mut total = 0;
    for body in orbits.keys() {
        let mut current = body;
        while *current != "COM" {
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

    #[test]
    fn one_direct_and_one_indirect() {
        let input = "COM)A\nA)B";
        let total = orbit_count(input);
        assert_eq!(total, 3);
    }

    #[test]
    fn orbits_specified_in_example() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let total = orbit_count(input);
        assert_eq!(total, 42);
    }
}
