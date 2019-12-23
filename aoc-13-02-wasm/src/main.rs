use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let program_input = fs::read_to_string("input")?;
    let program: Vec<_> = program_input
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input should have been a number"))
        .collect();

    let mut computer = aoc_13_02::Computer::new(program);
    computer.run();
    println!("{}", computer);

    println!("blocks remaining: {}", computer.num_blocks());
    println!("score: {}", computer.score);

    Ok(())
}
