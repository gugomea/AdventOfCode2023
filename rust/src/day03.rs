use std::fs;

#[derive(Debug)]
struct Grid {
    chars: Vec<Vec<char>>,
}

impl Grid {

    fn from_file(file: &str) -> Self {
        let file = fs::read_to_string(file).unwrap();
        let chars = file.lines()
            .map(|l| l.chars().collect())
            .collect();
        Self { chars }
    }

    fn parse_number(&self, i: i32, j: i32) -> Option<usize> {
        let (n, m) = (self.chars.len() as i32, self.chars[0].len() as i32);
        if i >= n || i <0 || j >= m || j <0 { return None};

        let (i, j) = (i as usize, j as usize);
        if !self.chars[i][j].is_digit(10) { return None; }

        let m = self.chars[0].len();

        let izda = self.chars[i][0..j]
            .iter().rev()
            .take_while(|x| x.is_digit(10));
        let dcha = self.chars[i][j..m]
            .iter()
            .take_while(|x| x.is_digit(10));
        format!("{}{}",
                izda.collect::<String>().chars().rev().collect::<String>(),
                dcha.collect::<String>(),
        ).parse().ok()
    }

    fn horizontal(&self, i: i32, j: i32) -> Vec<Option<usize>> {
        let parsed = self.parse_number(i, j); 
        match parsed {
            Some(_) => vec![parsed],
            None => vec![self.parse_number(i, j - 1),self.parse_number(i, j + 1)],
        }
    }

    fn check_all(&self, i: i32, j: i32) -> Option<Vec<usize>> {
        let ch = self.chars[i as usize][j as usize];
        if ch == '.' || ch >= '0' && ch <= '9' { return None; }
        let mut total = vec![];
        total.extend(self.horizontal(i - 1, j));
        total.extend(self.horizontal(i + 1, j));
        total.push(self.parse_number(i, j + 1));
        total.push(self.parse_number(i, j - 1));
        return Some(total.into_iter().filter_map(|x| x).collect());
    }

}

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    let file = "../input/day3.txt";
    let grid = Grid::from_file(file);
    let sum = grid.chars.iter().enumerate().map(|(i, x)|{
        x.into_iter().enumerate()
            .map(|(j, _ch)| grid.check_all(i as i32, j as i32))
            .filter_map(|x| x).map(|x| x.into_iter().sum::<usize>())
            .sum::<usize>()
    }).sum::<usize>();
    println!("{}", sum);
}

fn part2() {
    let file = "../input/day3.txt";
    let grid = Grid::from_file(file);
    let sum = grid.chars.iter().enumerate().map(|(i, x)|{
        x.into_iter().enumerate()
            .map(|(j, _ch)| grid.check_all(i as i32, j as i32))
            .filter_map(|x| x)
            .filter(|x| x.len() > 1)
            .map(|x| x.into_iter().product::<usize>())
            .sum::<usize>()
    }).sum::<usize>();
    println!("{}", sum);

}
