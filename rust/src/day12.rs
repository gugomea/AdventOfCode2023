use std::{fs, collections::HashMap};

pub fn solve() {
    part1();
    part2();
}

fn parse(input: &str) -> (Vec<char>, Vec<usize>) {
    let (springs, groups) = input.split_once(' ').unwrap();

    (springs.chars().collect(),
    groups.split(',').filter_map(|x| x.parse().ok()).collect())
}

struct Line {
    springs: Vec<char>,
    groups: Vec<usize>,
    n: usize,
    m: usize,
    dp: HashMap<(usize, usize, Option<char>), usize>,
}

impl Line {
    fn new( springs: Vec<char>, groups: Vec<usize>) -> Self {
        let (n, m) = (springs.len(), groups.len());
        Self { springs, groups, n, m, dp: HashMap::new()}
    }

    fn solve(&mut self) -> usize{
        return self.arrangements(0, 0, Some(self.springs[0]));
    }

    fn arrangements(&mut self, spring_index: usize, group_depth: usize, current: Option<char>) -> usize {
        // >= because when we have to process # I increase the index by more than one
        if spring_index >= self.n  && group_depth == self.m {
            return 1;
        } else if spring_index >= self.n {
            return 0;
        }

        if let Some(n) = self.dp.get(&(spring_index, group_depth, current)) {
            return *n;
        }

        let value = match current.unwrap() {
            '.' => return self.arrangements(spring_index + 1, group_depth, self.springs.get(spring_index + 1).copied()),

            '#' => {
                if group_depth >= self.m { return 0; }
                let n = self.groups[group_depth];
                if n + spring_index > self.n { return 0; }
                if self.springs[spring_index..spring_index+n].contains(&'.') {
                    return 0;
                }
                if let Some(ch) = self.springs.get(spring_index + n) {
                    if *ch == '#' {
                        return 0;
                    }
                }
                return self.arrangements(spring_index + n + 1, group_depth + 1, self.springs.get(spring_index + n + 1).copied());
            }

            '?' => self.arrangements(spring_index, group_depth, Some('.')) + self.arrangements(spring_index, group_depth, Some('#')),

            ch => panic!("Unvalid Token: {}\nExpected one of these: . # ?", ch),
        };

        self.dp.insert((spring_index, group_depth, current), value);
        return value;
    }

}

fn part1() {

    let file = fs::read_to_string("../input/day12.txt").unwrap();

    let results = file.lines()
        .map(parse)
        .map(|(x, y)| Line::new(x, y).solve())
        .sum::<usize>();

    println!("{}", results);
}

fn part2() {

    let file = fs::read_to_string("../input/day12.txt").unwrap();

    let results = file.lines()
        .map(|line| {
            let (l1, l2) = line.split_once(' ').unwrap();
            let l = format!("{}?", l1).repeat(4);
            let l = format!("{}{}", l, l1);
            let ll = format!("{},", l2).repeat(4);
            let ll = format!("{}{},", ll, l2);
            format!("{} {}", l, ll)
        })
        .map(|x| parse(&x))
        .map(|(x, y)| Line::new(x, y).solve())
        .sum::<usize>();
    println!("{}", results);
}
