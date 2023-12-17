use std::{fs, iter::zip};

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    solve_for_differences(0);
}

fn part2() {
    solve_for_differences(1);
}

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (n, m) = (matrix[0].len(), matrix.len());
    let mut result = vec![vec!['0'; m]; n];
    for i in 0..n {
        for j in 0..m {
            result[i][j] = matrix[j][i];
        }
    }
    return result;
}

fn parse_block(block: &str) -> Vec<Vec<char>> {
    block.lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn by_rows(grid: &Vec<Vec<char>>, first: usize) -> usize {
    grid.iter().map(|row| {
        let left = row.iter().take(first).rev();
        let right = row.iter().skip(first).take(first);
        zip(left,right).filter(|(a, b)| a != b).count()
    }).sum()
}

fn solve_for_differences(different: usize) {
    let file = fs::read_to_string("../input/day13.txt").unwrap();
    let grids = file.split("\n\n").map(parse_block);
    let mut total = 0;
    for grid in grids {
        let horizontal = (1..grid[0].len())
            .filter(|x| by_rows(&grid, *x) == different)
            .sum::<usize>();

        let grid = transpose(grid);

        let vertical = (1..grid[0].len())
            .filter(|x| by_rows(&grid, *x) == different)
            .sum::<usize>();

        total += horizontal + vertical * 100;
    }
    println!("{}", total);
}
