use serde_json::Value;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<i64> {
    let value: Value = serde_json::from_str(input)?;
    Ok(sum_numbers_part01(&value))
}

pub fn part02(input: &str) -> anyhow::Result<i64> {
    let value: Value = serde_json::from_str(input)?;
    Ok(sum_numbers_part02(&value))
}

fn sum_numbers_part01(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().expect("non u64 number"),
        Value::String(_) => 0,
        Value::Array(array) => array.iter().map(sum_numbers_part01).sum(),
        Value::Object(object) => object.iter().map(|(_, v)| sum_numbers_part01(v)).sum(),
        _ => unreachable!("input shouldn't contain any other types"),
    }
}

fn sum_numbers_part02(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().expect("non u64 number"),
        Value::String(_) => 0,
        Value::Array(array) => array.iter().map(sum_numbers_part02).sum(),
        Value::Object(object) => {
            let mut sum = 0;
            for (k, v) in object.iter() {
                if k == "red" || v.as_str().unwrap_or("") == "red" {
                    return 0;
                }
                sum += sum_numbers_part02(v);
            }
            sum
        }
        _ => unreachable!("input shouldn't contain any other types"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case(r#"[-1,{"a":1}]"#, 0)]
    #[case(r#"[]"#, 0)]
    #[case(r#"{}"#, 0)]
    fn test_part01(#[case] input: &str, #[case] expected: i64) {
        let ans = part01(input).unwrap();
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case(r#"[-1,{"a":1}]"#, 0)]
    #[case(r#"[]"#, 0)]
    #[case(r#"{}"#, 0)]
    #[case(r#"[1,{"c":"red","b":2},3]"#, 4)]
    #[case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0)]
    #[case(r#"[1,"red",5]"#, 6)]
    fn test_part02(#[case] input: &str, #[case] expected: i64) {
        let ans = part02(input).unwrap();
        assert_eq!(expected, ans);
    }
}
