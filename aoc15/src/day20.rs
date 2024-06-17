use num::integer::sqrt;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    let input = parse_input(input);

    for i in 1.. {
        let mut delivered = 0;
        do_factors(i, |f| delivered += f);
        delivered *= 10;
        if delivered >= input {
            return i;
        }
    }

    panic!("no solution found")
}

pub fn part02(input: &str) -> usize {
    let input = parse_input(input);

    for i in 1.. {
        let mut delivered = 0;
        do_factors(i, |f| {
            if f * 50 >= i {
                delivered += f
            }
        });
        delivered *= 11;
        if delivered >= input {
            return i;
        }
    }

    panic!("no solution found")
}

fn do_factors<F: FnMut(usize)>(x: usize, mut yld: F) {
    yld(1);
    yld(x);
    let root = sqrt(x);
    for n in 2..=root {
        if x % n == 0 {
            yld(n);
            let comp = x / n;
            if comp != n {
                yld(comp);
            }
        }
    }
}

fn parse_input(input: &str) -> usize {
    input.lines().next().unwrap().parse().unwrap()
}
