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

}

impl Grid {
    fn new(text: &str) -> Grid {
        Grid {}
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
    }
}
