use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
}

fn orbit_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut orbits = HashMap::new();

    for line in input.lines() {
        let mut bodies = line.split(")");

        let orbited = bodies.next().expect("Must have a body being orbited");
        let orbiting = bodies.next().expect("Must have a body doing the orbiting");

        orbits.entry(orbiting).or_insert(Vec::new()).push(orbited);
        orbits.entry(orbited).or_insert(Vec::new()).push(orbiting);
    }

    orbits
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn one_further_in_than_santa_is_1() {
        let input = "COM)YOU\nCOM)A\nA)SAN";
        let total = num_transfers_to_santa(input);
        assert_eq!(total, 1);
    }
}

fn num_transfers_to_santa(input: &str) -> usize {
    let orbits = orbit_graph(input);
    let santa_orbiting = orbits.get("SAN").first().expect("SAN must be orbiting something");

    let currently_orbiting = orbits.get("YOU").expect("YOU must be orbiting something");

    inner_num_transfers_to_santa(&orbits, santa_orbiting, currently_orbiting)
}

fn inner_num_transfers_to_santa(
    orbits: &HashMap<&str, Vec<&str>>,
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
