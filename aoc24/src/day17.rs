use arrayvec::ArrayVec;
use itertools::{iproduct, Itertools};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> String {
    let ((ra, rb, rc), program) = parse_input(input);
    let mut cpu = Cpu { ra, rb, rc };
    cpu.process(&program).into_iter().join(",")
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (_, program) = parse_input(input);

    // got help from reddit, but the general idea is that when looking at the decompilation you can
    // see that since we always output rb%8 and rb is derived from ra that we only depend on 3 bits
    // of ra for any given number in the program. so, to find the solution, iterate the program in
    // reverse and solve for each number.
    //
    // manual decompilation:
    // 2,4 => rb = a % 8
    // 1,1 => rb = rb ^ 1
    // 7,5 => rc = ra / 2^rb
    // 0,3 => ra = ra / 8       // ignore, ra not used after this line
    // 4,3 => rb = rb ^ rc
    // 1,6 => rb = rb ^ 6
    // 5,5 => output(rb % 8)
    // 3,0 => jump              // 0 so ignore (so loop ends when ra==0)

    let mut candidates = ArrayVec::<usize, 8>::new();
    candidates.push(0);
    for n in program.iter().rev() {
        // there could be multiple numbers that produce the correct result (especially towards the
        // beginning). we need to try all of them b/c some may not produce a number needed later.
        let mut next = ArrayVec::<usize, 8>::new();
        for (candidate, i) in iproduct!(candidates, 0..8usize) {
            // << 3 moves the already solved bits left, we then check 0..8 to determine what the
            // next number is
            let ra = (candidate << 3) + i;

            let mut cpu = Cpu { ra, rb: 0, rc: 0 };
            // ASSUMPTION: we trim the last jump instruction off of the program so that we only run
            // one iteration of the loop, this allows us to solve just this one number
            // You don't NEED to do this, but it's much more performant than running the program
            // to termination only to take the first output
            let output = cpu.process(&program[..program.len() - 2]);
            if output.first().unwrap() == n {
                next.push(ra);
            }
        }
        candidates = next;
    }
    Ok(*candidates.iter().min().unwrap())
}

#[derive(Debug)]
struct Cpu {
    ra: usize,
    rb: usize,
    rc: usize,
}

impl Cpu {
    fn process(&mut self, program: &[usize]) -> Vec<usize> {
        let mut output = Vec::new();
        let mut ip = 0;
        while ip < program.len() {
            let opcode = program[ip];
            let operand = program[ip + 1];
            let mut ip_next = ip + 2;
            match opcode {
                0 => self.ra /= 2usize.pow(self.decode_combo(operand) as u32),
                1 => self.rb ^= operand,
                2 => self.rb = self.decode_combo(operand) % 8,
                3 => {
                    if self.ra != 0 {
                        ip_next = operand;
                    }
                }
                4 => self.rb ^= self.rc, // ignores operand
                5 => output.push(self.decode_combo(operand) % 8),
                6 => self.rb = self.ra / 2usize.pow(self.decode_combo(operand) as u32),
                7 => self.rc = self.ra / 2usize.pow(self.decode_combo(operand) as u32),
                _ => unreachable!("unknown opcode {opcode}"),
            }
            ip = ip_next;
        }
        output
    }

    fn decode_combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            7 => unreachable!("combo(7): invalid instruction"),
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> ((usize, usize, usize), Vec<usize>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = registers
        .lines()
        .map(|s| s.split_whitespace().last().unwrap().parse().unwrap())
        .collect_tuple()
        .unwrap();
    let program = program
        .split_whitespace()
        .nth(1)
        .map(|nums| nums.split(',').map(|n| n.parse().unwrap()).collect())
        .unwrap();
    (registers, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day17.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!("2,0,7,3,0,3,1,3,7", ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(247839539763386, ans);
    }
}
