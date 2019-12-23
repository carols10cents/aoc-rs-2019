fn main() {
    let mut screen = aoc_13_02::Screen::new();
    let mut game_over = screen.run();

    while !game_over {
        println!("{}", screen);
        game_over = screen.run();
    }

    println!("score: {}", screen.score());
}
