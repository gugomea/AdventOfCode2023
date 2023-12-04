#![allow(non_snake_case)]
use std::fs;

pub fn solve() {
    part1();
    part2();
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<(usize, usize, usize)>,
}

impl Game {
    fn possible(&self, (m1, m2, m3): (usize, usize, usize)) -> usize {
        if self.sets.iter().any(|&(r, g, b)| r > m1 || g > m2 || b > m3) {
            return 0
        }
        return self.id;
    }
    fn max(&self) -> usize{
        let R = self.sets.iter().map(|&(r, _, _)| r).max().unwrap();
        let G = self.sets.iter().map(|&(_, g, _)| g).max().unwrap();
        let B = self.sets.iter().map(|&(_, _, b)| b).max().unwrap();
        return R * G * B;
    }
}

impl From<(usize, &str)> for Game {
    fn from((id, value): (usize, &str)) -> Self {
        let sets = value[6..].split(";")
            .map(|x| x.split(" ").collect::<Vec<_>>())
            .map(|game| (parse("red", &game), parse("green", &game), parse("blue", &game)))
            .collect();
        Self {
            id: id + 1,
            sets,
        }
    }
}

fn parse(color: &str, origin: &[&str]) -> usize {
    match origin.iter().position(|x| x.contains(color)) {
        Some(i) => origin[i - 1].parse().unwrap(),
        None => 0
    }
}

fn part1() {
    let file = fs::read_to_string("../input/day2.txt").unwrap();
    let maximum = (12, 13, 14);
    let solution = file.lines()
        .enumerate()
        .map(|x| Game::from(x).possible(maximum))
        .sum::<usize>();
    println!("{:#?}", solution);
}

fn part2() {
    let file = fs::read_to_string("../input/day2.txt").unwrap();
    let solution  = file.lines()
        .enumerate()
        .map(|x| Game::from(x).max())
        .sum::<usize>();
    println!("{:#?}", solution);
}
