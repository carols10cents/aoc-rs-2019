use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let mut lines = input.lines();

    let w1 = lines
        .next()
        .expect("Should have been at least 1 line in the input");
    let w2 = lines
        .next()
        .expect("Should have been at least 2 lines in the input");

    let closest_crossed_wires = closest_crossed_wires(&w1, &w2);

    println!(
        "Distance to the closest crossed wires is {}",
        closest_crossed_wires
    );

    Ok(())
}

// ## Assumptions/thoughts
//
// - The central port is at (0, 0)
// - Right and Up amounts can be represented by positive numbers
// - Left and Down amounts can be represented by negative numbers
// - Right and Left are in the X direction
// - Up and Down are in the Y direction
//
// ## Plan
//
// - Write code to parse "R2,D5" strings into a more convenient data structure
// - Write code that can turn paths into a list of points that each wire touches
// - Find the points that both wires have in their list of points
// - The Manhattan distance of any point to (0, 0) is x + y
// - Sort the intersection points by Manhattan distance and take the min

#[derive(Debug, PartialEq)]
enum PathPart {
    X(i32),
    Y(i32),
}
use PathPart::{X, Y};

fn locations_from_path(path: Vec<PathPart>) -> HashSet<(i32, i32)> {
    let mut current_location = (0, 0);
    let mut locations = HashSet::new();

    for part in path {
        match part {
            X(x) => {
                let (range, amount) = if x > 0 { (0..x, 1) } else { (x..0, -1) };

                for _ in range {
                    current_location = (current_location.0 + amount, current_location.1);
                    locations.insert(current_location);
                }
            }
            Y(y) => {
                let (range, amount) = if y > 0 { (0..y, 1) } else { (y..0, -1) };

                for _ in range {
                    current_location = (current_location.0, current_location.1 + amount);
                    locations.insert(current_location);
                }
            }
        }
    }

    locations
}

fn min_manhattan_distance(points: &HashSet<(i32, i32)>) -> i32 {
    points
        .iter()
        .map(|point| point.0.abs() + point.1.abs())
        .min()
        .expect("List should have had items in it")
}

fn parse_path(path_str: &str) -> Vec<PathPart> {
    path_str
        .split(",")
        .map(|segment| {
            let mut chars = segment.chars();
            let direction = chars.next().expect(&format!(
                "Segment should have had a direction char: [{}]",
                segment
            ));
            let magnitude: i32 = chars.collect::<String>().parse().expect(&format!(
                "Segment after direction char should have parsed as an i32: [{}]",
                segment
            ));

            match direction {
                'R' => X(magnitude),
                'L' => X(-magnitude),
                'U' => Y(magnitude),
                'D' => Y(-magnitude),
                _ => panic!("Unknown direction: [{}]", direction),
            }
        })
        .collect()
}

fn closest_crossed_wires(path_str1: &str, path_str2: &str) -> i32 {
    let path1 = parse_path(path_str1);
    let path2 = parse_path(path_str2);

    let locations1 = locations_from_path(path1);
    let locations2 = locations_from_path(path2);

    let intersections: HashSet<_> = locations1.intersection(&locations2).cloned().collect();

    min_manhattan_distance(&intersections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right() {
        let path = vec![X(1)];
        let locations = locations_from_path(path);
        let expected: HashSet<_> = [(1, 0)].iter().cloned().collect();
        assert_eq!(locations, expected);
    }

    #[test]
    fn left() {
        let path = vec![X(-1)];
        let locations = locations_from_path(path);
        let expected: HashSet<_> = [(-1, 0)].iter().cloned().collect();
        assert_eq!(locations, expected);
    }

    #[test]
    fn up() {
        let path = vec![Y(1)];
        let locations = locations_from_path(path);
        let expected: HashSet<_> = [(0, 1)].iter().cloned().collect();
        assert_eq!(locations, expected);
    }

    #[test]
    fn down() {
        let path = vec![Y(-1)];
        let locations = locations_from_path(path);
        let expected: HashSet<_> = [(0, -1)].iter().cloned().collect();
        assert_eq!(locations, expected);
    }

    #[test]
    fn example_from_problem_description() {
        let wire1_path_string = "R8,U5,L5,D3";
        let wire2_path_string = "U7,R6,D4,L4";

        let wire1_path = parse_path(wire1_path_string);
        assert_eq!(wire1_path, vec![X(8), Y(5), X(-5), Y(-3)]);

        let wire2_path = parse_path(wire2_path_string);
        assert_eq!(wire2_path, vec![Y(7), X(6), Y(-4), X(-4)]);

        let wire1_locations = locations_from_path(wire1_path);
        let wire2_locations = locations_from_path(wire2_path);

        let intersections: HashSet<_> = wire1_locations
            .intersection(&wire2_locations)
            .cloned()
            .collect();
        let expected: HashSet<_> = [(3, 3), (6, 5)].iter().cloned().collect();
        assert_eq!(intersections, expected);

        assert_eq!(min_manhattan_distance(&intersections), 6);
    }

    #[test]
    fn example_from_problem_description_all_at_once() {
        let wire1_path_string = "R8,U5,L5,D3";
        let wire2_path_string = "U7,R6,D4,L4";

        let closest_crossed_wires = closest_crossed_wires(wire1_path_string, wire2_path_string);
        assert_eq!(closest_crossed_wires, 6);
    }

    #[test]
    fn other_examples_from_problem_description() {
        let w1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let w2 = "U62,R66,U55,R34,D71,R55,D58,R83";

        let closest = closest_crossed_wires(w1, w2);
        assert_eq!(closest, 159);

        let w1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let w2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let closest = closest_crossed_wires(w1, w2);
        assert_eq!(closest, 135);
    }
}
