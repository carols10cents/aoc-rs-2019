fn main() {
    println!("Hello, world!");
}

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

    fn apply_gravity(&mut self) {}

    fn apply_velocity(&mut self) {}
}

fn time_step(moons: &mut [Moon]) {
    for moon in moons.iter_mut() {
        moon.apply_gravity();
    }

    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}
