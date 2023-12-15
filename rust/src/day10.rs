use std::{fs, collections::VecDeque};

pub fn solve() {
    part1();
}

fn matches(u1: &[(i64, i64); 2], u2: &[(i64, i64); 2]) -> bool {
    for u in u1 {
        for uu in u2 {
            if u.1 == -uu.1 && u.0 == -uu.0 { return true; }
        }
    }
    return false;
}

fn bfs(given: [(i64, i64); 2], i: i64, j: i64, matrix: &mut Vec<Vec<usize>>, source: &mut Vec<Vec<[(i64, i64); 2]>>, visited: &mut Vec<Vec<bool>>) {
    let mut queue = VecDeque::from(
        [([i, j, 0], given)]
    );

    let (n, m) = (matrix.len() as i64, matrix[0].len() as i64);

    while let Some(([a, b, cont], previus)) = queue.pop_front() {
        if a < 0 || b < 0 || a >= n || b>= m { continue; }

        let (i, j) = (a as usize, b as usize);
        if visited[i][j] { continue; }
        if !matches(&previus, &source[i][j]) { continue; }
        visited[i][j] = true;
        matrix[i][j] = cont as usize;

        for (c1, c2) in source[i][j] {
            queue.push_back(([a + c1, b + c2, cont + 1], source[i][j]));
        }
    }
}

fn dfs(i: i64, j: i64, v: &mut Vec<Vec<usize>>) {
    let (n, m) = (v.len() as i64, v[0].len() as i64);
    if i < 0 || j < 0 || i >= n || j >= m { return; }

    if v[i as usize][j as usize] != 0 { return ; }
    v[i as usize][j as usize] = usize::MAX;
    dfs(i + 1, j, v);
    dfs(i - 1, j, v);
    dfs(i, j + 1, v);
    dfs(i, j - 1, v);
}

fn follow_path(matrix: &mut Vec<Vec<[(i64, i64); 2]>>, grid: &mut Vec<Vec<usize>>, i: i64, j: i64, value: usize, flag: bool) {
    let [d1, d2] = matrix[i as usize][j as usize];
    let [d1, d2] = [(i + d1.0, j + d1.1), (i + d2.0, j + d2.1)];
    let expected = value + 1;
    if grid[d1.0 as usize][d1.1 as usize] == expected {

        let mut direccion = (d1.1 - j, d1.0 - i);
        if direccion.1 == 0 { direccion.0 *= -1; }

        if flag {
            direccion.0 *= -1;
            direccion.1 *= -1;
        }

        let (i, j) = (i + direccion.0, j + direccion.1);
        dfs(i, j, grid);
        let (i, j) = (d1.0 + direccion.0, d1.1 + direccion.1);
        dfs(i, j, grid);
        follow_path(matrix, grid, d1.0, d1.1, expected, flag);
    } else if grid[d2.0 as usize][d2.1 as usize] == expected {
        let mut direccion = (d2.1 - j, d2.0 - i);
        if direccion.1 == 0 { direccion.0 *= -1; }

        if flag {
            direccion.0 *= -1;
            direccion.1 *= -1;
        }

        let (i, j) = (i + direccion.0, j + direccion.1);
        dfs(i, j, grid);
        let (i, j) = (d2.0 + direccion.0, d2.1 + direccion.1);
        dfs(i, j, grid);
        follow_path(matrix, grid, d2.0, d2.1, expected, flag);

    } else {}
}

fn part1() {

    let file = fs::read_to_string("../input/day10.txt").unwrap();
    let (mut s_i, mut s_j) = (0, 0);
    let mut grid = file.lines().enumerate().map(|(i, line)| {
        line.chars().enumerate().map(|(j, ch)| {
            if ch == 'S' { (s_i, s_j) = (i as i64, j as i64); }
            match ch {
                '|' => [(1, 0), (-1, 0)],
                '-' => [(0, 1), (0, -1)],
                'L' => [(-1, 0), (0, 1)],
                'J' => [(-1, 0), (0, -1)],
                '7' => [(0, -1), (1, 0)],
                'F' => [(1, 0), (0, 1)],
                '.' | 'S' => [(0, 0), (0, 0)],
                _ => panic!(),
            }
        }).collect()
    }).collect::<Vec<Vec<_>>>();

    let (n, m) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; m]; n];
    let mut matrix = vec![vec![0; m]; n];

    let initial = [(1, 0), (0, 1)];
    let given = [(-1, 0), (0, -1)];
    grid[s_i as usize][s_j as usize] = initial;
    bfs(given, s_i, s_j, &mut matrix, &mut grid, &mut visited);

    follow_path(&mut grid, &mut matrix, s_i + initial[0].0, s_j + initial[0].1, 1, false);
    follow_path(&mut grid, &mut matrix, s_i + initial[1].0, s_j + initial[1].1, 1, true);

    matrix[s_i as usize][s_j as usize] = 0;

    println!("{}", matrix.into_iter().map(|line| line
            .into_iter()
            .filter(|&x| x == usize::MAX)
            .count())
        .sum::<usize>());

}
