use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

// # Plan
//
// - For each asteroid
//   - For each "angle" expressed by (-width..width)x(-height..height)
//   - If you see an asteroid along that angle, stop and add 1 to the count
//   - If you get to the end of the graph, stop
//   - Otherwise go another interval along that angle
// - Return asteroid with the max count

struct Grid {
    width: usize,
    height: usize,
    asteroid_locations: HashMap<(usize, usize), usize>,
}

impl Grid {
    fn new(text: &str) -> Grid {
        let mut width = 0;
        let mut height = 0;
        let mut asteroid_locations = HashMap::new();

        for line in text.lines() {
            width = 0;

            for c in line.chars() {
                if c == '#' {
                    asteroid_locations.insert((width, height), 0);
                }
                width += 1;
            }

            height += 1;
        }

        Grid {
            width, height, asteroid_locations
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_text_into_a_list_of_asteroids() {
        let text = "...\n...";
        let grid = Grid::new(text);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert!(grid.asteroid_locations.is_empty());

        let text = "#.\n..\n.#";
        let grid = Grid::new(text);
        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.asteroid_locations.len(), 2);
        assert!(grid.asteroid_locations.contains_key(&(0, 0)));
        assert!(grid.asteroid_locations.contains_key(&(1, 2)));
    }
}
