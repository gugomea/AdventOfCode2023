use std::{fs, collections::HashMap};

pub fn solve() {
    part1();
    part2();
}

fn parse_node(input: &str) -> (&str, (&str, &str)) {
    let (key, locations) = input.split_once(" = ").unwrap();
    let n = locations.len();
    let (left, right) = locations[1..n-1].split_once(", ").unwrap();
    (key, (left, right))
}

fn parse_file(file: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (instructions, network) = file.split_once("\n\n").unwrap();
    let network = network
        .lines()
        .map(parse_node)
        .collect();

    (instructions, network)
}

fn part1() {
    let file = fs::read_to_string("../input/day8.txt").unwrap();
    let (instructions, network) = parse_file(&file);

    let (mut key, mut total) = ("AAA", 0);
    for instruction in instructions.chars().cycle() {
        if key == "ZZZ" { break; }

        key = match instruction {
            'R' => network.get(key).unwrap().1,
            'L' => network.get(key).unwrap().0,
            ic => panic!("Invalid instruction: {}", ic),
        };

        total += 1;
    }
    println!("total: {}", total);
}

fn part2() {
    let file = fs::read_to_string("../input/day8.txt").unwrap();
    let (instructions, network) = parse_file(&file);

    let mut keys = network
        .iter()
        .filter(|(key, _)| key.chars().last().unwrap() == 'A')
        .map(|(&key, _)| key)
        .collect::<Vec<_>>();

    let mut totals = vec![];
    for key in keys.iter_mut() {
        let mut total = 0;

        for instruction in instructions.chars().cycle() {
            if let Some('Z') = key.chars().last() { break; }

            *key = match instruction {
                'R' => network.get(key).unwrap().1,
                'L' => network.get(key).unwrap().0,
                ic => panic!("Invalid instruction: {}", ic),
            };
            total += 1;
        };

        //If this is 0 fo all(which is) we can just calculate lcm for the `totals` vector.
        //println!("{}", total % instructions.len());

        totals.push(total);
    }

    let total = totals.into_iter().fold(1, |acc, x| lcm(acc, x));

    println!("total: {}", total);
}

fn lcm(x: usize, y: usize) -> usize {
    return (x * y) / gcd(x, y);
}

fn gcd(x: usize, y: usize) -> usize {
    if y > x { return gcd(y, x); }

    match x % y == 0 {
        true => y,
        false => gcd(x, x % y),
    }
}
