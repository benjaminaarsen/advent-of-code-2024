fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn check_report(report: &Vec<i32>) -> bool {
    let all_increasing = report.windows(2).all(|window| window[0] < window[1]);
    let all_decreasing = report.windows(2).all(|window| window[0] > window[1]);
    if !all_increasing && !all_decreasing {
        return false;
    }
    for window in report.windows(2) {
        if (window[0] - window[1]).abs() < 1 || (window[0] - window[1]).abs() > 3 {
            return false;
        }
    }
    true
}

fn part2(input: &str) -> String {
    let mut safe_count = 0;
    for report in input.lines() {
        let report = report
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if !check_report(&report) {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if check_report(&new_report) {
                    safe_count += 1;
                    break;
                }
            }
        } else {
            safe_count += 1;
        }
    }
    safe_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let result = part2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "4");
    }
}
