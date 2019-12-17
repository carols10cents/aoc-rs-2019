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
}
