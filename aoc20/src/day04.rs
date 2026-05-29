use anyhow::Context;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const CID_OPTIONAL: u8 = 0b01111111;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut num_valid = 0;
    for passport in input.split("\n\n") {
        let mut passport_mask = 0;
        for field in passport.split_whitespace() {
            passport_mask |= field_mask(&field[..3]);
        }

        if passport_mask & CID_OPTIONAL == CID_OPTIONAL {
            num_valid += 1;
        }
    }
    Ok(num_valid)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut num_valid = 0;
    for passport in input.split("\n\n") {
        let mut passport_mask = 0;
        let mut fields_valid = true;
        for field in passport.split_whitespace() {
            passport_mask |= field_mask(&field[..3]);

            let (field_name, rhs) = field.split_once(':').context("invalid field")?;
            let field_valid = match field_name {
                "byr" => (1920..=2002).contains(&rhs.parse::<usize>()?),
                "iyr" => (2010..=2020).contains(&rhs.parse::<usize>()?),
                "eyr" => (2020..=2030).contains(&rhs.parse::<usize>()?),
                "hgt" => {
                    let amt = &rhs[..rhs.len() - 2];
                    match &rhs[rhs.len() - 2..] {
                        "cm" => (150..=193).contains(&amt.parse::<usize>()?),
                        "in" => (59..=76).contains(&amt.parse::<usize>()?),
                        _ => false,
                    }
                }
                "hcl" => rhs.starts_with('#') && rhs.bytes().skip(1).all(|c| c.is_ascii_hexdigit()),
                "ecl" => matches!(rhs, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
                "pid" => rhs.len() == 9 && rhs.bytes().all(|c| c.is_ascii_digit()),
                "cid" => true, // optional
                _ => unreachable!("field {field_name} unknown"),
            };
            fields_valid &= field_valid;
            if !fields_valid {
                break;
            }
        }

        let has_required_fields = passport_mask & CID_OPTIONAL == CID_OPTIONAL;
        if fields_valid && has_required_fields {
            num_valid += 1;
        }
    }
    Ok(num_valid)
}

fn field_mask(field_name: &str) -> u8 {
    match field_name {
        "byr" => 1,
        "iyr" => 2,
        "eyr" => 4,
        "hgt" => 8,
        "hcl" => 16,
        "ecl" => 32,
        "pid" => 64,
        "cid" => 0, // optional
        _ => unreachable!("field {field_name} unknown"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day04.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(170, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(103, ans);
    }
}
