mod day01;

fn main() {
    println!("Day 01:");
    println!(
        "  P1: {}",
        day01::part01(include_str!("../inputs/1.input.txt"))
    );
    println!(
        "  P2: {}",
        day01::part02(include_str!("../inputs/1.input.txt"))
    );
}
