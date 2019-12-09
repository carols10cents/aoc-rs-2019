use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    println!("{}", num_transfers_to_santa(input));
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
    let santa_orbiting = *orbits.get("SAN").expect("SAN must be orbiting something");

    let mut candidates = vec![*orbits.get("YOU").expect("YOU must be orbiting something")];
    let mut num_transfers = 0;
    let mut visited = vec!["YOU"];

    while !candidates.contains(&santa_orbiting) {
        num_transfers += 1;

        let mut new_candidates = vec![];
        for c in candidates {
            visited.push(c);
        }

        for c in candidates {
            if let Some(inner) = orbits.get(c) {
                if !visited.contains(inner) && !new_candidates.contains(inner) {
                    new_candidates.push(inner);
                }
            }

            for (outer, _) in orbits
                .iter()
                .filter(|(k, &v)| v == c && !visited.contains(k) && !new_candidates.contains(k)) {
                new_candidates.push(outer);
            }
        }

        candidates = new_candidates;
    }
    num_transfers
}
