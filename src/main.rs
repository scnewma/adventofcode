mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

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
    println!("Day 03:");
    println!(
        "  P1: {}",
        day03::part01(include_str!("../inputs/3.input.txt"))
    );
    println!(
        "  P2: {}",
        day03::part02(include_str!("../inputs/3.input.txt"))
    );
    println!("Day 04:");
    println!(
        "  P1: {}",
        day04::part01(include_str!("../inputs/4.input.txt"))
    );
    println!(
        "  P2: {}",
        day04::part02(include_str!("../inputs/4.input.txt"))
    );
    println!("Day 05:");
    println!(
        "  P1: {}",
        day05::part01(include_str!("../inputs/5.input.txt"))
    );
    println!(
        "  P2: {}",
        day05::part02(include_str!("../inputs/5.input.txt"))
    );
}
