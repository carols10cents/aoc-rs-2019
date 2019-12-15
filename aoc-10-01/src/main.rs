use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

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
            width,
            height,
            asteroid_locations,
        }
    }

    fn get_seeing_counts(&mut self) {
        for &from_location in self.asteroid_locations.keys() {
            let mut count = 0;
            for &to_location in self.asteroid_locations.keys() {
                if self.can_see(from_location, to_location) {
                    count += 1;
                }
            }
            self.asteroid_locations.insert(from_location, count);
        }
    }

    fn can_see(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if from == to {
            false
        } else {
            let angle = reduced_angle(from, to);

            let mut check_x = from.0 as isize + angle.0;
            let mut check_y = from.1 as isize + angle.1;

            while !(check_x == to.0 as isize && check_y == to.1 as isize) {
                if check_x < 0 || check_y < 0 {
                    panic!(
                        "something when terribly wrong, tried to check ({}, {})",
                        check_x, check_y
                    );
                }
                if self
                    .asteroid_locations
                    .contains_key(&(check_x as usize, check_y as usize))
                {
                    return false;
                }
                check_x += angle.0;
                check_y += angle.1;
            }

            true
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

    #[test]
    fn cant_see_yourself() {
        let grid = Grid::new("#");
        assert!(!grid.can_see((0, 0), (0, 0)));
    }

    #[test]
    fn can_see_horizontally() {
        let grid = Grid::new("##");
        assert!(grid.can_see((0, 0), (1, 0)));
    }

    #[test]
    fn cant_see_with_asteroid_in_the_way() {
        let grid = Grid::new("###");
        assert!(!grid.can_see((0, 0), (2, 0)));
    }

    #[test]
    fn example() {
        let map = ".#..#\n.....\n#####\n....#\n...##";
        let grid = Grid::new(map);
        grid.get_seeing_counts();
        assert_eq!(grid.asteroid_locations[&(3, 4)], 8);
        assert_eq!(grid.asteroid_locations[&(1, 0)], 7);
        assert_eq!(grid.asteroid_locations[&(4, 0)], 7);
        assert_eq!(grid.asteroid_locations[&(0, 2)], 6);
        assert_eq!(grid.asteroid_locations[&(1, 2)], 7);
        assert_eq!(grid.asteroid_locations[&(2, 2)], 7);
        assert_eq!(grid.asteroid_locations[&(3, 2)], 7);
        assert_eq!(grid.asteroid_locations[&(4, 2)], 5);
        assert_eq!(grid.asteroid_locations[&(4, 3)], 7);
        assert_eq!(grid.asteroid_locations[&(4, 4)], 7);
    }

    #[test]
    fn greatest_common_factor() {
        assert_eq!(gcf(48, 18), 6);
        assert_eq!(gcf(18, 48), 6);
        assert_eq!(gcf(2, 0), 2);
        assert_eq!(gcf(0, 2), 2);
        assert_eq!(gcf(2, 3), 1);
        assert_eq!(gcf(3, 2), 1);
        assert_eq!(gcf(-10, 5), 5);
        assert_eq!(gcf(10, -5), 5);
    }

    #[test]
    fn reduced_angle_works() {
        assert_eq!(reduced_angle((0, 0), (1, 0)), (1, 0));
        assert_eq!(reduced_angle((0, 0), (2, 0)), (1, 0));

        assert_eq!(reduced_angle((1, 0), (0, 0)), (-1, 0));
        assert_eq!(reduced_angle((2, 0), (0, 0)), (-1, 0));

        assert_eq!(reduced_angle((1, 1), (3, 3)), (1, 1));
        assert_eq!(reduced_angle((3, 3), (1, 1)), (-1, -1));

        assert_eq!(reduced_angle((0, 0), (48, 18)), (8, 3));

        assert_eq!(reduced_angle((0, 0), (2, 3)), (2, 3));
    }
}

fn reduced_angle(from: (usize, usize), to: (usize, usize)) -> (isize, isize) {
    let from = (from.0 as isize, from.1 as isize);
    let to = (to.0 as isize, to.1 as isize);

    let x_diff = to.0 - from.0;
    let y_diff = to.1 - from.1;

    let gcf = gcf(x_diff, y_diff);

    (x_diff / gcf, y_diff / gcf)
}

fn gcf(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcf(b, a % b)
    }
}
