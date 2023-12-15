use std::fs;

pub fn solve() {
    part1();
    part2();
}

fn transpose(v: &Vec<Vec<char>>) ->  Vec<Vec<char>> {
    let (n, m) = (v.len(), v[0].len());
    let mut v2 = vec![vec!['0'; n]; m];

    for i in 0..n {
        for j in 0..m {
            v2[i][j] = v[j][i];
        }
    }
    return v2;
}

fn calculate(multiplier: usize) {
    let file = fs::read_to_string("../input/day11.txt").unwrap();
    let matrix = file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let m = matrix;
    let m_t = transpose(&m);
    let is = m.iter()
        .map(|x| x.iter().all(|&ch| ch == '.'))
        .collect::<Vec<_>>();
    let js = m_t.iter()
        .map(|x| x.iter().all(|&ch| ch == '.'))
        .collect::<Vec<_>>();
    let (n, k) = (m.len(), m[0].len());

    let mut galaxy = vec![];
    for i in 0..n {
        let i_padding = is.iter()
            .enumerate()
            .filter(|&(idx, &x)| idx <= i && x)
            .count() * multiplier;
        for j in 0..k {
            let j_padding = js.iter()
                .enumerate()
                .filter(|&(idx, &x)| idx <= j && x)
                .count() * multiplier;
            if m[i][j] == '#' {
                galaxy.push(((i_padding + i) as i64, (j_padding + j) as i64));
            }
        }
    }

    let mut total = 0;
    for dot in &galaxy {
        for other in &galaxy {
            total += (dot.0 - other.0).abs() + (dot.1 - other.1).abs();
        }
    }

    println!("{:?}", total / 2);
}

fn part1() {
    calculate(1);
}

fn part2() {
    calculate(999_999);
}

