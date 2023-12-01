use std::fs;
pub fn solve() {
    part1();
    part2()
}

fn part1() {
    let fich = fs::read_to_string("./src/day1/input.txt").unwrap();
    let firsts = fich.lines().map(|l| l.chars().find_map(|c| c.to_digit(10)).unwrap());
    let lasts = fich.lines().map(|l| l.chars().rev().find_map(|c| c.to_digit(10)).unwrap());
    let f_sum = firsts.map(|x| x * 10).sum::<u32>();
    let l_sum = lasts.sum::<u32>();
    println!("{:?}", f_sum + l_sum);
}

fn part2() {
    let fich = fs::read_to_string("./src/day1/input.txt").unwrap();
    let fich = fich
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    let firsts = fich.lines().map(|l| l.chars().find_map(|c| c.to_digit(10)).unwrap());
    let lasts = fich.lines().map(|l| l.chars().rev().find_map(|c| c.to_digit(10)).unwrap());
    let f_sum = firsts.map(|x| x * 10).sum::<u32>();
    let l_sum = lasts.sum::<u32>();
    println!("{:?}", f_sum + l_sum);
}
