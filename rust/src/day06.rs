use std::{fs, iter::zip};
pub fn solve() {
    part1();
    part2();
}

fn part1() {
    let file = fs::read_to_string("../input/day6.txt").unwrap();
    let mut aux = file.lines().map(|x| x.split_whitespace().skip(1)
                                   .filter_map(|x| x.parse::<usize>().ok())
                                   .collect::<Vec<_>>());
    let (times, races) = (aux.next().unwrap(), aux.next().unwrap());
    let total = zip(times, races)
        .map(calculate_records)
        .product::<usize>();
    println!("total: {total}");
}

fn calculate_records((time, race): (usize, usize)) -> usize {
    (0..time + 1)
        .map(|i| (time - i) * i)
        .filter(|&x| x > race)
        .count()
}

fn part2() {
    let file = fs::read_to_string("../input/day6.txt").unwrap();
    let mut aux = file.lines()
        .filter_map(|x| x.split_whitespace().skip(1)
                         .collect::<String>()
                         .parse::<usize>().ok());
    let (time, distance) = (aux.next().unwrap(), aux.next().unwrap());
    let total = calculate_records((time, distance));
    println!("total: {}", total);
}
