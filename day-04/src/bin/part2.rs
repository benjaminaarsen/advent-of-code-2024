fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for r in 0..grid.len() - 2 {
        for c in 0..grid[0].len() - 2 {
            let block: [[char; 3]; 3] = [
                [grid[r][c], grid[r][c + 1], grid[r][c + 2]],
                [grid[r + 1][c], grid[r + 1][c + 1], grid[r + 1][c + 2]],
                [grid[r + 2][c], grid[r + 2][c + 1], grid[r + 2][c + 2]],
            ];
            let diag1 = [block[0][0], block[1][1], block[2][2]];
            let diag2 = [block[0][2], block[1][1], block[2][0]];
            if [diag1, diag2]
                .iter()
                .all(|diag| matches!(diag, ['M', 'A', 'S'] | ['S', 'A', 'M']))
            {
                count += 1;
            }
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let result = part2(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 9);
    }
}
