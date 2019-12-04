use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
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
        let wire1_path = vec![X(8), Y(5), X(-5), Y(-3)];
        let wire2_path = vec![Y(7), X(6), Y(-4), X(-4)];

        let wire1_locations = locations_from_path(wire1_path);
        let wire2_locations = locations_from_path(wire2_path);

        let intersections: HashSet<_> = wire1_locations
            .intersection(&wire2_locations)
            .cloned()
            .collect();
        let expected: HashSet<_> = [(3, 3), (6, 5)].iter().cloned().collect();
        assert_eq!(intersections, expected);
    }
}
