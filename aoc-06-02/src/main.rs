use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("{}", orbit_count(input));
}

fn orbit_graph(input: &str) -> HashMap<&str, &str> {
    let mut orbits = HashMap::new();

    for line in input.lines() {
        let mut bodies = line.split(")");

        let orbited = bodies.next().expect("Must have a body being orbited");
        let orbiting = bodies.next().expect("Must have a body doing the orbiting");

        orbits.insert(orbiting, orbited);
    }

    orbits
}

fn orbit_count(input: &str) -> usize {
    let orbits = orbit_graph(input);

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

    #[test]
    fn you_orbiting_same_as_santa_is_0() {
        let input = "COM)SAN\nCOM)YOU";
        let total = num_transfers_to_santa(input);
        assert_eq!(total, 0);
    }

    #[test]
    fn one_further_out_than_santa_is_1() {
        let input = "COM)SAN\nCOM)A\nA)YOU";
        let total = num_transfers_to_santa(input);
        assert_eq!(total, 1);
    }
}

fn num_transfers_to_santa(input: &str) -> usize {
    let orbits = orbit_graph(input);
    let santa_orbiting = orbits.get("SAN").expect("SAN must be orbiting something");

    let currently_orbiting = orbits.get("YOU").expect("YOU must be orbiting something");

    inner_num_transfers_to_santa(&orbits, santa_orbiting, currently_orbiting)
}

fn inner_num_transfers_to_santa(
    orbits: &HashMap<&str, &str>,
    santa_orbiting: &str,
    currently_orbiting: &str,
) -> usize {
    if currently_orbiting == santa_orbiting {
        0
    } else {
        1 + inner_num_transfers_to_santa(
            orbits,
            santa_orbiting,
            orbits
                .get(currently_orbiting)
                .expect("Recursive orbiting must orbit something"),
        )
    }
}
