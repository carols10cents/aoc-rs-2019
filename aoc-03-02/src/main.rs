use std::collections::{HashMap, HashSet};
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

    let min_steps = min_steps_to_crossed_wires(&w1, &w2);

    println!("Min steps to the closest crossed wires is {}", min_steps);

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

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Location {
    fn from(f: (i32, i32)) -> Location {
        Location { x: f.0, y: f.1 }
    }
}

fn locations_from_path(path: Vec<PathPart>) -> HashMap<Location, i32> {
    let mut current_location = Location { x: 0, y: 0 };
    let mut steps = 0;
    let mut locations = HashMap::new();

    for part in path {
        match part {
            X(x) => {
                let (range, amount) = if x > 0 { (0..x, 1) } else { (x..0, -1) };

                for _ in range {
                    current_location = Location {
                        x: current_location.x + amount,
                        ..current_location
                    };
                    steps += 1;
                    locations.insert(current_location, steps);
                }
            }
            Y(y) => {
                let (range, amount) = if y > 0 { (0..y, 1) } else { (y..0, -1) };

                for _ in range {
                    current_location = Location {
                        y: current_location.y + amount,
                        ..current_location
                    };
                    steps += 1;
                    locations.insert(current_location, steps);
                }
            }
        }
    }

    locations
}

fn min_manhattan_distance(points: &HashSet<Location>) -> i32 {
    points
        .iter()
        .map(|point| point.x.abs() + point.y.abs())
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

    let locations1_with_steps = locations_from_path(path1);
    let locations2_with_steps = locations_from_path(path2);

    let locations1: HashSet<_> = locations1_with_steps.keys().cloned().collect();
    let locations2: HashSet<_> = locations2_with_steps.keys().cloned().collect();

    let intersections: HashSet<_> = locations1.intersection(&locations2).cloned().collect();

    min_manhattan_distance(&intersections)
}

fn min_steps(
    intersections: &HashSet<Location>,
    loc_steps1: &HashMap<Location, i32>,
    loc_steps2: &HashMap<Location, i32>,
) -> i32 {
    intersections
        .iter()
        .map(|i| {
            let steps1 = loc_steps1.get(&i).expect("Intersection must be in path1");
            let steps2 = loc_steps2.get(&i).expect("Intersection must be in path2");
            steps1 + steps2
        })
        .min()
        .expect("Must be at least 1 intersection")
}

fn min_steps_to_crossed_wires(path_str1: &str, path_str2: &str) -> i32 {
    let path1 = parse_path(path_str1);
    let path2 = parse_path(path_str2);

    let locations1_with_steps = locations_from_path(path1);
    let locations2_with_steps = locations_from_path(path2);

    let locations1: HashSet<_> = locations1_with_steps.keys().cloned().collect();
    let locations2: HashSet<_> = locations2_with_steps.keys().cloned().collect();

    let intersections: HashSet<_> = locations1.intersection(&locations2).cloned().collect();
    min_steps(
        &intersections,
        &locations1_with_steps,
        &locations2_with_steps,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right() {
        let path = vec![X(1)];
        let locations = locations_from_path(path);
        assert_eq!(locations.len(), 1);
        assert_eq!(locations.get(&(1, 0).into()), Some(&1));
    }

    #[test]
    fn left() {
        let path = vec![X(-1)];
        let locations = locations_from_path(path);
        assert_eq!(locations.len(), 1);
        assert_eq!(locations.get(&(-1, 0).into()), Some(&1));
    }

    #[test]
    fn up() {
        let path = vec![Y(1)];
        let locations = locations_from_path(path);
        assert_eq!(locations.len(), 1);
        assert_eq!(locations.get(&(0, 1).into()), Some(&1));
    }

    #[test]
    fn down() {
        let path = vec![Y(-1)];
        let locations = locations_from_path(path);
        assert_eq!(locations.len(), 1);
        assert_eq!(locations.get(&(0, -1).into()), Some(&1));
    }

    #[test]
    fn example_from_problem_description() {
        let wire1_path_string = "R8,U5,L5,D3";
        let wire2_path_string = "U7,R6,D4,L4";

        let wire1_path = parse_path(wire1_path_string);
        assert_eq!(wire1_path, vec![X(8), Y(5), X(-5), Y(-3)]);

        let wire2_path = parse_path(wire2_path_string);
        assert_eq!(wire2_path, vec![Y(7), X(6), Y(-4), X(-4)]);

        let locations1_with_steps = locations_from_path(wire1_path);
        let locations2_with_steps = locations_from_path(wire2_path);

        let wire1_locations: HashSet<_> = locations1_with_steps.keys().cloned().collect();
        let wire2_locations: HashSet<_> = locations2_with_steps.keys().cloned().collect();

        let intersections: HashSet<_> = wire1_locations
            .intersection(&wire2_locations)
            .cloned()
            .collect();
        let expected: HashSet<_> = [(3, 3).into(), (6, 5).into()].iter().cloned().collect();
        assert_eq!(intersections, expected);

        assert_eq!(min_manhattan_distance(&intersections), 6);
        assert_eq!(
            min_steps(
                &intersections,
                &locations1_with_steps,
                &locations2_with_steps
            ),
            30
        );
    }

    #[test]
    fn example_from_problem_description_all_at_once() {
        let wire1_path_string = "R8,U5,L5,D3";
        let wire2_path_string = "U7,R6,D4,L4";

        let closest_crossed_wires = closest_crossed_wires(wire1_path_string, wire2_path_string);
        assert_eq!(closest_crossed_wires, 6);

        let min_steps_to_crossed_wires =
            min_steps_to_crossed_wires(wire1_path_string, wire2_path_string);
        assert_eq!(min_steps_to_crossed_wires, 30);
    }

    #[test]
    fn other_examples_from_problem_description() {
        let w1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let w2 = "U62,R66,U55,R34,D71,R55,D58,R83";

        let closest = closest_crossed_wires(w1, w2);
        assert_eq!(closest, 159);

        let min_steps = min_steps_to_crossed_wires(w1, w2);
        assert_eq!(min_steps, 610);

        let w1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let w2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let closest = closest_crossed_wires(w1, w2);
        assert_eq!(closest, 135);

        let min_steps = min_steps_to_crossed_wires(w1, w2);
        assert_eq!(min_steps, 410);
    }
}
