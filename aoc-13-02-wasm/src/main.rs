fn main() {
    let mut computer = aoc_13_02::Computer::new(36, 24);
    computer.run();
    println!("{}", computer);

    println!("blocks remaining: {}", computer.num_blocks());
    println!("score: {}", computer.score);
}
