use std::fs;

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    let file = fs::read_to_string("../input/day9.txt").unwrap();
    let result = file
        .lines()
        .map(|x| x.split(" ")
             .filter_map(|x| x.parse::<i64>().ok())
             .collect())
        .map(calculate_next)
        .sum::<i64>();

    println!("resultado: {}", result);
}

fn calculate_next(numbers: Vec<i64>) -> i64 {

    let mut current_numbers = numbers;
    let mut values = vec![];
    while current_numbers.iter().any(|&x| x != 0) {
        let n = current_numbers.len();
        values.push(current_numbers[n - 1]);
        current_numbers = current_numbers.windows(2).map(|x| x[1] - x[0]).collect();
    }
    values.iter().sum()
}

fn part2() {
    let file = fs::read_to_string("../input/day9.txt").unwrap();
    let result = file
        .lines()
        .map(|x| {
            let mut v = x.split(" ")
             .filter_map(|x| x.parse().ok())
             .collect::<Vec<_>>();
            v.reverse();
            return v;
        })
        .map(calculate_next)
        .sum::<i64>();

    println!("resultado: {}", result);
}
