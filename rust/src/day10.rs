use std::fs;

pub fn solve() {
    part1();
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Tile {
    fn from_char(ch: char) -> Self {
        match ch {
            '|' => Self { up: true, down: true, left: false, right: false },
            '-' => Self { up: false, down: false, left: true, right: true },
            'L' => Self { up: true, down: true, left: false, right: false },
            'J' => Self { up: true, down: true, left: false, right: false },
            '7' => Self { up: true, down: true, left: false, right: false },
            'F' => Self { up: true, down: true, left: false, right: false },
            '.' => Self { up: true, down: true, left: false, right: false },
            'S' => panic!("idk"),
        }
    }
}

fn part1() {

    let file = fs::read_to_string("../input/test.txt").unwrap();
    let grid = file.lines().map(|line| {
        line.chars().map(|ch| match ch {
            '|' => Some((Flow::Up, Flow::Down)),
            '-' => Some((Flow::Left, Flow::Right)),
            'L' => Some((Flow::Up, Flow::Right)),
            'J' => Some((Flow::Left, Flow::Up)),
            '7' => Some((Flow::Down, Flow::Left)),
            'F' => Some((Flow::Down, Flow::Right)),
            '.' => None,
            'S' => panic!("idk"),
            c => panic!("invalid character: {}", c),
        }).collect()
    }).collect::<Vec<Vec<_>>>();

    println!("grid: {:?}", grid);

}
