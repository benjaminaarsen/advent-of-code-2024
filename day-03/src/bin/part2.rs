use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .filter_map(|cap| {
            let match_index = cap.get(0).unwrap().start();
            let prefix = &input[..match_index];
            let last_do = prefix.rfind("do()");
            let last_dont = prefix.rfind("don't()");
            let last_pre_operation = match (last_do, last_dont) {
                (Some(a), Some(b)) => {
                    if a > b {
                        Some("do()")
                    } else {
                        Some("don't()")
                    }
                }
                (Some(_), None) => Some("do()"),
                (None, Some(_)) => Some("don't()"),
                (None, None) => None,
            };
            if last_pre_operation == Some("don't()") {
                None
            } else {
                let a = cap[1].parse::<i32>().unwrap();
                let b = cap[2].parse::<i32>().unwrap();
                Some(a * b)
            }
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }
}
