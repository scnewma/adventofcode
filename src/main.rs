mod day01;
mod day02;

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
    println!("Day 02:");
    println!(
        "  P1: {}",
        day02::part01(include_str!("../inputs/2.input.txt"))
    );
    println!(
        "  P2: {}",
        day02::part02(include_str!("../inputs/2.input.txt"))
    );
}
