use std::fs;

pub fn solve() {
    part1();
    part2();
}

fn calculate_column(grid: &Vec<Vec<char>>, column: usize) -> usize {
    let rows = grid.len();
    let mut multiplier = grid.len();
    let mut total = 0;
    for i in 0..rows {
        if grid[i][column] == 'O' {
            total += multiplier;
            multiplier -= 1;
        } else if grid[i][column] == '#' {
            multiplier = rows - (i + 1);
        }
    }
    return total;
}

fn part1() {
    let file = fs::read_to_string("../input/day14.txt").unwrap();
    let grid = file.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<_>>>();
    let mut total = 0;
    for column in 0..grid[0].len() {
        let result = calculate_column(&grid, column);
        total += result;
    }
    println!("total: {total}");
}

fn part2() {
}
