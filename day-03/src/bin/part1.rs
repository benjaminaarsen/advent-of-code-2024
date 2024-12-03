use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re
        .captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            (a, b)
        })
        .collect::<Vec<(i32, i32)>>();
    let total: i32 = matches.iter().map(|(a, b)| a * b).sum();
    // println!("{:?}", matches);
    total
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result =
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }
}
