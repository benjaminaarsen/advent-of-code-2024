use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let guard_start_coords = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&c| c == '^').map(|col| (row, col)))
        .unwrap();
    let mut guard_direction = (-1, 0);
    let mut guard = guard_start_coords;
    let mut visited = HashSet::new();

    loop {
        let (row, col) = guard;

        visited.insert(guard);

        let check_coords = (
            row as isize + guard_direction.0,
            col as isize + guard_direction.1,
        );
        // the new coordinate would be out of bounds
        if check_coords.0 < 0
            || check_coords.0 >= grid.len() as isize
            || check_coords.1 < 0
            || check_coords.1 >= grid[0].len() as isize
        {
            break;
        }
        // if we hit an obstacle, rotate 90 degrees to the right
        if grid[check_coords.0 as usize][check_coords.1 as usize] == '#' {
            // rotate 90 degrees to the right
            guard_direction = match guard_direction {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };
            continue;
        }

        // move forward
        guard = (check_coords.0 as usize, check_coords.1 as usize);
    }
    visited.len() as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, 41);
    }
}
