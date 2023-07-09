use std::str::FromStr;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    Ok(packages(input).map(|p| p.wrapping_paper_needed()).sum())
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    Ok(packages(input).map(|p| p.ribbon_needed()).sum())
}

#[inline]
fn packages(input: &str) -> impl Iterator<Item = Package> + '_ {
    input.lines().filter_map(|ln| Package::from_str(ln).ok())
}

struct Package(u32, u32, u32);

impl Package {
    fn wrapping_paper_needed(&self) -> u32 {
        let lw = self.0 * self.1;
        let wh = self.1 * self.2;
        let hl = self.2 * self.0;
        let slack = lw.min(wh).min(hl);
        2 * lw + 2 * wh + 2 * hl + slack
    }

    fn ribbon_needed(&self) -> u32 {
        let max_side = self.0.max(self.1).max(self.2);
        let perim = 2 * (self.0 + self.1 + self.2 - max_side);
        let vol = self.0 * self.1 * self.2;
        perim + vol
    }
}

impl FromStr for Package {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dimensions = s.splitn(3, "x").map(|s| u32::from_str(s).unwrap());
        Ok(Package(
            dimensions.next().unwrap(),
            dimensions.next().unwrap(),
            dimensions.next().unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_part01(#[case] input: &str, #[case] expected: u32) {
        let ans = input.parse::<Package>().unwrap().wrapping_paper_needed();
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_part02(#[case] input: &str, #[case] expected: u32) {
        let ans = input.parse::<Package>().unwrap().ribbon_needed();
        assert_eq!(expected, ans);
    }
}
