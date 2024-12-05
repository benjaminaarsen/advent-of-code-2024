fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let page_ordering_rules: Vec<(i32, i32)> = data[0]
        .lines()
        .map(|line| {
            let mut iter = line.split("|").map(|x| x.parse::<i32>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect();
    let updates = data[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut total_middle_page_numbers = 0;

    for update in updates.iter() {
        let new_update = check_update(update, &page_ordering_rules);
        if new_update != *update {
            total_middle_page_numbers += new_update.get(new_update.len() / 2).unwrap();
        }
    }
    total_middle_page_numbers
}

fn check_update(update: &Vec<i32>, page_ordering_rules: &Vec<(i32, i32)>) -> Vec<i32> {
    if update
        .iter()
        .all(|&page| check_page(page, update, page_ordering_rules))
    {
        return update.clone();
    }

    let mut new_update = update.clone();
    new_update.sort_by(|a, b| {
        for &(rule_a, rule_b) in page_ordering_rules.iter() {
            if rule_a == *a && rule_b == *b {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Greater
    });
    new_update
}

fn check_page(page: i32, update: &Vec<i32>, page_ordering_rules: &Vec<(i32, i32)>) -> bool {
    page_ordering_rules
        .iter()
        .filter(|&&(a, _)| a == page)
        .all(|&(a, b)| match (update.contains(&a), update.contains(&b)) {
            (true, true) => {
                update.iter().position(|&x| x == a).unwrap()
                    < update.iter().position(|&x| x == b).unwrap()
            }
            (true, false) | (false, true) => true,
            (false, false) => panic!("impossible"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let result = part2(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 123);
    }
}
