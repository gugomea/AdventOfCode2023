use std::fs;

fn hash(input: &str) -> usize {
    input.as_bytes()
        .into_iter()
        .fold(0_usize, |acc, &x| ((x as usize + acc) * 17) % 256)
}

fn part1() {
    let file = fs::read_to_string("../input/day15.txt").unwrap().replace("\n", "");

    let result: usize = file.split(',').map(hash).sum();

    println!("{}", result);
}

fn part2() {
    let file = fs::read_to_string("../input/day15.txt").unwrap().replace("\n", "");
    let mut v: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for instruction in file.split(',') {
        if let Some((name, number)) = instruction.split_once('=') {
            let number = number.parse().unwrap();
            let hash = hash(name);
            match v[hash].iter().position(|x| x.0 == name) {
                Some(position) => v[hash][position] = (name, number),
                None => v[hash].push((name, number)),
            }
        } else {
            let n = instruction.len();
            let instruction = &instruction[..n-1];
            let hash = hash(instruction);
            if let Some(position) = v[hash].iter().position(|x| x.0 == instruction) {
                v[hash].remove(position);
            }
        }
    }

    let mut total = 0;
    for (idx, row) in v.into_iter().enumerate() {
        total += row.into_iter()
            .enumerate().map(|(k, (_, v))| (idx + 1) * (k + 1) * v)
            .sum::<usize>();
    }

    println!("{}", total);
}

pub fn solve() {
    part1();
    part2();
}
