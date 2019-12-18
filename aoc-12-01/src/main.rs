use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone)]
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
