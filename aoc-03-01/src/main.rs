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

fn locations_from_path(path: Vec<PathPart>) -> Vec<(i32, i32)> {
    let mut current_location = (0, 0);
    let mut locations = vec![];

    for part in path {
        match part {
            X(x) => {
                let (range, amount) = if x > 0 {
                    (0..x, 1)
                } else {
                    (x..0, -1)
                };

                for _ in range {
                    current_location = (current_location.0 + amount, current_location.1);
                    locations.push(current_location);
                }
            }
            Y(y) => unimplemented!(),
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
        assert_eq!(locations, vec![(1, 0)]);
    }

    #[test]
    fn left() {
        let path = vec![X(-1)];
        let locations = locations_from_path(path);
        assert_eq!(locations, vec![(-1, 0)]);
    }
}
