fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut left_ids: Vec<i32> = Vec::new();
    let mut right_ids: Vec<i32> = Vec::new();
    let input_iter = input.lines();
    let mut total_diff = 0;
    for line in input_iter {
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let left = nums.next().unwrap();
        let right = nums.next().unwrap();
        left_ids.push(left);
        right_ids.push(right);
    }
    left_ids.sort();
    right_ids.sort();
    assert!(left_ids.len() == right_ids.len());

    for i in 0..left_ids.len() {
        let left = left_ids[i];
        let right = right_ids[i];
        let diff = (left - right).abs();
        total_diff += diff;
    }
    total_diff.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, "11");
    }
}
