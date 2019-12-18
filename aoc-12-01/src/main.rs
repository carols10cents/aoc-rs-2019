use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug, PartialEq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    dx: i32,
    dy: i32,
    dz: i32,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            x,
            y,
            z,
            dx: 0,
            dy: 0,
            dz: 0,
        }
    }

    fn parse(text: &str) -> Moon {
        // <x=2, y=-10, z=-7>

        let mut chars = text.chars().skip_while(|&c| c != '=');
        chars.next(); // discard the =

        let mut x_digits = vec![];
        let mut current_digit = chars.next().expect("Should have been some x digits");
        while current_digit != ',' {
            x_digits.push(current_digit);
            current_digit = chars
                .next()
                .expect("Should have been either more x digits or a comma");
        }
        let x_string: String = x_digits.iter().collect();
        let x: i32 = x_string
            .parse()
            .expect("Should have been able to parse x_string");

        let mut chars = chars.skip_while(|&c| c != '=');
        chars.next(); // discard the =

        let mut y_digits = vec![];
        let mut current_digit = chars.next().expect("Should have been some y digits");
        while current_digit != ',' {
            y_digits.push(current_digit);
            current_digit = chars
                .next()
                .expect("Should have been either more y digits or a comma");
        }
        let y_string: String = y_digits.iter().collect();
        let y: i32 = y_string
            .parse()
            .expect("Should have been able to parse y_string");

        let mut chars = chars.skip_while(|&c| c != '=');
        chars.next(); // discard the =

        let mut z_digits = vec![];
        let mut current_digit = chars.next().expect("Should have been some z digits");
        while current_digit != '>' {
            z_digits.push(current_digit);
            current_digit = chars
                .next()
                .expect("Should have been either more z digits or a >");
        }
        let z_string: String = z_digits.iter().collect();
        let z: i32 = z_string
            .parse()
            .expect("Should have been able to parse x_string");

        Moon::new(x, y, z)
    }

    fn apply_gravity(&mut self, moons: &[Moon]) {
        for moon in moons {
            self.dx += gravity_adjustment(self.x, moon.x);
            self.dy += gravity_adjustment(self.y, moon.y);
            self.dz += gravity_adjustment(self.z, moon.z);
        }
    }

    fn apply_velocity(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }
}

fn gravity_adjustment(current_coordinate: i32, other_coordinate: i32) -> i32 {
    match current_coordinate.cmp(&other_coordinate) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

fn time_step(moons: &mut [Moon], time: i32) -> i32 {
    let immutable_moons: Vec<_> = moons.iter().cloned().collect();

    for moon in moons.iter_mut() {
        moon.apply_gravity(&immutable_moons);
    }

    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }

    time + 1
}

fn parse_moons(text: &str) -> Vec<Moon> {
    text.trim()
        .lines()
        .map(|moon_text| Moon::parse(moon_text))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_moons_from_text() {
        let moons = parse_moons(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>\n",
        );
        let expected_moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        assert_eq!(moons, expected_moons);
    }

    #[test]
    fn one_step() {
        let mut moons = parse_moons(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>\n",
        );
        let expected_moons = vec![
            Moon { x: 2, y: -1, z: 1, dx: 3, dy: -1, dz: -1 },
            Moon { x: 3, y: -7, z: -4, dx: 1, dy: 3, dz: 3 },
            Moon { x: 1, y: -7, z: 5, dx: -3, dy: 1, dz: -3 },
            Moon { x: 2, y: 2, z: 0, dx: -1, dy: -3, dz: 1 },
        ];

        time_step(&mut moons, 0);

        assert_eq!(moons, expected_moons);
    }
}
