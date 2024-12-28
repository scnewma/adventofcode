#!/usr/bin/env bash
set -euo pipefail

INPUT="$1"
DAY="$(printf '%02d' "$INPUT")"

PKG="aoc19"

sed -i "" -e "s%// pub mod day$DAY;%pub mod day$DAY;%" $PKG/src/lib.rs
sed -i "" -e "s%// day$DAY::run,%day$DAY::run,%" $PKG/src/main.rs

touch "$PKG/inputs/day$DAY.example.txt"
touch "$PKG/inputs/day$DAY.input.txt"

cat << EOF > "$PKG/src/day$DAY.rs"
pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
  Ok(0)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
  Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day$DAY.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
EOF

GEN_BENCHES_LINE="gen_benches!["
i=1
while (( i <= INPUT )); do
    if (( i != 1 )); then
        GEN_BENCHES_LINE="$GEN_BENCHES_LINE, "
    fi
    GEN_BENCHES_LINE="${GEN_BENCHES_LINE}day$(printf '%02d' $i)"
    i=$((i + 1))
done
GEN_BENCHES_LINE="${GEN_BENCHES_LINE}];"
sed -i "" -e "s/gen_benches!\[.*\];/$GEN_BENCHES_LINE/" $PKG/benches/bench.rs

cargo fmt
