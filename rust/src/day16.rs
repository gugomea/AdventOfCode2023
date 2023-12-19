use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Dot,
    Slash,
    BackSlash,
    Vertical,
    Horizontal,
}

impl Element {

    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Element::Dot,
            '/' => Element::Slash,
            '\\' => Element::BackSlash,
            '|' => Element::Vertical,
            '-' => Element::Horizontal,
            ch => panic!("Invalid Token: {ch}"),
        }
    }

    fn transform(&self, dx: i64, dy: i64) -> impl Iterator<Item = (i64, i64)> {
        match self {
            Element::Dot => vec![(dx, dy)],
            Element::Vertical => match dx == 0 {
                true => vec![(1, 0), (-1, 0)],
                false => vec![(dx, dy)],
            }
            Element::Horizontal => match dx == 0 {
                true => vec![(dx, dy)],
                false => vec![(0, 1), (0, -1)],
            }
            Element::Slash => vec![(-dy, -dx)],
            Element::BackSlash => vec![(dy, dx)],
        }.into_iter()
    }

}

fn get_grid() -> Vec<Vec<Element>> {
    let file = fs::read_to_string("../input/day16.txt").unwrap();
    file.lines()
        .map(|l| l.chars()
             .map(Element::from_char)
             .collect())
        .collect()
}

fn part1() {
    let grid = get_grid();
    let startings = [((0, 0), (0, 1))].into_iter();
    solve_starting(grid, startings);
}

fn part2() {
    let grid = get_grid();
    let (n, m) = (grid.len() as i64, grid[0].len() as i64);

    let up = (0..m).map(|i| ((0, i), (1, 0)));
    let down = (0..m).map(|i| ((n - 1, i), (-1, 0)));
    let left = (0..n).map(|i| ((i, 0), (0, 1)));
    let right = (0..n).map(|i| ((i, m - 1), (0, -1)));
    let startings = up.chain(down).chain(left).chain(right);

    solve_starting(grid, startings);
}

fn update(direction: (i64, i64)) -> char{
    match direction  {
        (0, 1)  => '>',
        (0, -1) => '<',
        (1, 0)  => 'v',
        (-1, 0) => '^',
        ch => panic!("Invalid Token: {ch:?}"),
    }
}

fn solve_starting(grid: Vec<Vec<Element>>, startings: impl Iterator<Item = ((i64, i64), (i64, i64))>) {

    let (n, m) = (grid.len(), grid[0].len());

    let mut max = 0;
    for starting in startings {
        let mut stack = vec![starting];

        let mut result = vec![vec!['0'; m]; n];
        result[starting.0.0 as usize][starting.0.1 as usize] = update(starting.1);

        while let Some(((i, j), (di, dj))) = stack.pop() {
            let next = grid[i as usize][j as usize].transform(di, dj);
            for (ni, nj) in next {
                let (vi, vj) = (i + ni, j + nj);
                let valid_index = vi >= 0 && vj >= 0 && vi < n as i64 && vj < m as i64;

                if valid_index {
                    let r = result[vi as usize][vj as usize];
                    result[vi as usize][vj as usize] = update((ni, nj));
                    if r != result[vi as usize][vj as usize] {
                        stack.push(((vi, vj), (ni, nj)));
                    }
                }
            }
        }

        let cont = result.into_iter()
            .map(|row| row.into_iter()
                 .filter(|&ch| ch != '0')
                 .count())
            .sum::<usize>();
        if cont > max { max = cont; }
    }
    println!("{}", max);
}

pub fn solve() {
    part1();
    part2();
}
