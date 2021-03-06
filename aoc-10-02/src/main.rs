use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    let mut grid = Grid::new(input.trim(), (37, 25));
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    asteroid_locations: HashMap<(usize, usize), f64>,
    laser_location: (usize, usize),
}

impl Grid {
    fn new(text: &str, laser_location: (usize, usize)) -> Grid {
        let mut width = 0;
        let mut height = 0;
        let mut asteroid_locations = HashMap::new();

        for line in text.lines() {
            width = 0;

            for c in line.chars() {
                if c == '#' {
                    asteroid_locations.insert((width, height), 0.0);
                }
                width += 1;
            }

            height += 1;
        }

        asteroid_locations.remove(&laser_location);

        Grid {
            width,
            height,
            asteroid_locations,
            laser_location,
        }
    }

    fn compute_angles(&mut self) {
        let locations: Vec<(usize, usize)> = self.asteroid_locations.keys().cloned().collect();
        for to in locations {

            let x_diff = (to.0 as isize - self.laser_location.0 as isize) as f64;
            let y_diff = (to.1 as isize - self.laser_location.1 as isize) as f64;

            let angle = (y_diff / x_diff).atan();
            self.asteroid_locations.insert(to, angle);
        }
    }

    fn asteroids_sorted_by_angle(&self) -> Vec<(usize, usize)> {
        let mut vec: Vec<_> = self.asteroid_locations.iter().collect();
        vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        vec.iter().map(|&(location, _)| location).copied().collect()
    }

    fn destroy_order(&mut self) -> Vec<(usize, usize)> {
        let mut destroy_order = vec![];

        while !self.asteroid_locations.is_empty() {
            let candidates = self.asteroids_sorted_by_angle();
            for asteroid in candidates {
                if self.can_see(self.laser_location, asteroid) {
                    destroy_order.push(asteroid);
                }
            }

            for destroyed in &destroy_order {
                self.asteroid_locations.remove(&destroyed);
            }
        }

        destroy_order
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
    fn example() {
        let input = "\
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let mut grid = Grid::new(input, (8, 3));
        grid.compute_angles();

        // dbg!(grid.asteroids_sorted_by_angle());
        // assert!(false);

        assert_eq!(grid.destroy_order(), vec![
            (8, 1), (9, 0), (9, 1), (10, 0), (9, 2), (11, 1), (12, 1), (11, 2), (15, 1),
            (12, 2), (13, 2), (14, 2), (15, 2), (12, 3), (16, 4), (15, 4), (10, 4), (4, 4),
            (2, 4), (2, 3), (0, 2), (1, 2), (0, 1), (1, 1), (5, 2), (1, 0), (5, 1),
            (6, 1), (6, 0), (7, 0), (8, 0), (10, 1), (14, 0), (16, 1), (13, 3), (14, 3)
        ]);
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
