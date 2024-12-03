fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut safe_count = 0;
    'reports: for report in input.lines() {
        let levels = report
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let all_increasing = levels.windows(2).all(|window| window[0] < window[1]);
        let all_decreasing = levels.windows(2).all(|window| window[0] > window[1]);
        if !all_increasing && !all_decreasing {
            continue;
        }
        for window in levels.windows(2) {
            if (window[0] - window[1]).abs() < 1 || (window[0] - window[1]).abs() > 3 {
                continue 'reports;
            }
        }
        println!("{} is safe", report);
        safe_count += 1;
    }
    safe_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "2");
    }
}
