fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_lines = grid.len();
    let n_cols = grid[0].len();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut total = 0;

    for row in 0..n_lines {
        for col in 0..n_cols {
            if grid[row][col] != 'X' {
                continue;
            }
            total += directions.iter().filter(|&&(dx, dy)| {
                (1..=3).all(|step| {
                    let new_row = row as isize + dy * step;
                    let new_col = col as isize + dx * step;
                    if new_row < 0 || new_row >= n_lines as isize || new_col < 0 || new_col >= n_cols as isize {
                        return false;
                    }
                    let expected_char = match step {
                        1 => 'M',
                        2 => 'A',
                        3 => 'S',
                        _ => unreachable!(),
                    };
                    grid[new_row as usize][new_col as usize] == expected_char
                })
            }).count();
        }
    }
    total as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
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
        assert_eq!(result, 18);
    }
}
