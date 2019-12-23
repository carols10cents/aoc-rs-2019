fn main() {
    let mut computer = aoc_13_02::Computer::new();
    let mut game_over = computer.run();

    while !game_over {
        println!("{}", computer);
        game_over = computer.run();
    }

    println!("blocks remaining: {}", computer.num_blocks());
    println!("score: {}", computer.score);
}
