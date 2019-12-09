use std::collections::HashMap;
use std::cmp;

fn main() {
    let input = include_str!("../input");
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
        println!("Found santa! Returning!");
        0
    } else {
        println!("No santa here :( Currently orbiting: {}", currently_orbiting);
        let move_in = orbits.get(currently_orbiting);
        println!("Further in is {:?}", move_in);
        let inward = move_in.map(|body| inner_num_transfers_to_santa(orbits, santa_orbiting, body));
        println!("Inward is {:?}", inward);

        let outward_candidates: Vec<_> = orbits.iter().filter(|(&k, &v)| v == currently_orbiting && k != "YOU").collect();
        println!("outward_candidates = {:?}", outward_candidates);
        7
        // .map(|(body, _)| inner_num_transfers_to_santa(orbits, santa_orbiting, body)).min();
        // println!("Outward is {:?}", outward);
        //
        // let returning = 1 + match (inward, outward) {
        //     (Some(i), Some(o)) => cmp::min(i, o),
        //     (Some(i), None) => i,
        //     (None, Some(o)) => o,
        //     (None, None) => unreachable!("Nowhere to move, something has gone terribly wrong"),
        // };
        // println!("returning {}", returning);
    }
}
