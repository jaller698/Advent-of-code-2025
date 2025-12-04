use std::io::{self, Read};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Position {
    Empty = 0,
    Occupied = 1,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    // Construct a two dimensional vector of digits from the input
    // if the position is '@' it is occupied, otherwise it is empty
    let grid = parse_input(&input);
    let sum = multiple_turns_until_stable(grid);
    println!("Total movements needed: {}", sum);
}

fn multiple_turns_until_stable(grid: Vec<Vec<Position>>) -> i32 {
    let mut current_grid = grid.clone();
    let mut total_rolls = 0;

    loop {
        let rolls_needed = determine_rolls_needed(&current_grid);
        if rolls_needed == 0 {
            break;
        }
        total_rolls += rolls_needed;
        current_grid = update_grid(&current_grid);
    }

    total_rolls
}

fn determine_rolls_needed(grid: &Vec<Vec<Position>>) -> i32 {
    let mut rolls_needed = 0;
    let mut position_index = (0, 0);
    for row in grid {
        for pos in row {
            if determine_forklift_movement(pos, grid, position_index) {
                rolls_needed += 1;
            }
            position_index.1 += 1;
        }
        position_index.0 += 1;
        position_index.1 = 0;
    }
    rolls_needed
}

fn parse_input(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Position::Occupied,
                    _ => Position::Empty,
                })
                .collect()
        })
        .collect()
}

fn determine_forklift_movement(
    pos: &Position,
    grid: &Vec<Vec<Position>>,
    position_index: (i32, i32),
) -> bool {
    match pos {
        Position::Occupied => {
            // check all 8 surrounding positions
            // if less than 4 are occupied, return 1
            let mut occupied_count = 0;
            let directions = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            for dir in &directions {
                let new_x = position_index.0 + dir.0;
                let new_y = position_index.1 + dir.1;
                if new_x >= 0
                    && (new_x as usize) < grid.len()
                    && new_y >= 0
                    && (new_y as usize) < grid[0].len()
                {
                    if grid[new_x as usize][new_y as usize] == Position::Occupied {
                        occupied_count += 1;
                    }
                }
            }

            if occupied_count < 4 {
                return true;
            } else {
                return false;
            }
        }
        Position::Empty => false,
    }
}

fn update_grid(grid: &Vec<Vec<Position>>) -> Vec<Vec<Position>> {
    let mut new_grid: Vec<Vec<Position>> = grid.clone();
    for (i, row) in grid.iter().enumerate() {
        for (j, pos) in row.iter().enumerate() {
            if determine_forklift_movement(pos, grid, (i as i32, j as i32)) {
                new_grid[i][j] = Position::Empty;
            }
        }
    }
    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_paper_one_turn() {
        let example_lines = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];
        let grid = parse_input(&example_lines.join("\n"));

        let sum = determine_rolls_needed(&grid);

        assert_eq!(sum, 13);
    }

    #[test]
    fn test_keep_finding_paper() {
        let example_lines = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];
        let grid = parse_input(&example_lines.join("\n"));
        let sum = multiple_turns_until_stable(grid);

        assert_eq!(sum, 43);
    }
}

