use std::fs;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>>{
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>> {
    let fichero = fs::read_to_string("../input/day4.txt")?;
    let wining_numbers = fichero.lines()
        .enumerate()
        .map(|(i, x)| Card::parse_card(i, x).calculate())
        .sum::<usize>();
    println!("{}", wining_numbers);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let fichero = fs::read_to_string("../input/day4.txt")?;
    let cards = fichero.lines().enumerate()
        .map(|(i, x)| Card::parse_card(i, x))
        .collect::<Vec<_>>();
    let n = cards.len();
    let mut counts = vec![0; n];

    for i in 1..=n {
        proceso_recursivo(&cards, i, &mut counts);
        counts[i - 1] += 1;
    }

    let n = counts.iter().sum::<usize>();
    println!("{}", n);
    Ok(())
}

fn proceso_recursivo(cards: &[Card], card_id: usize, counts: &mut [usize]){
    let aux = cards[card_id - 1].winning_numbers();
    aux.for_each(|x|{
        counts[x - 1] += 1;
        proceso_recursivo(cards, x, counts);
    });
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn parse_card(id: usize, s: &str) -> Self {
        let mut t = s.split(" | ").map(|x| x.split_whitespace().filter_map(|x| x.parse().ok()).collect());
        if let (Some(winning), Some(numbers)) = (t.next(), t.next()) {
            return Self { id: id + 1, winning, numbers };
        }
        panic!("NO");
    }

    fn winning_numbers(&self) -> impl Iterator<Item = usize> {
        let n = self.id;
        (1..=self.matches()).map(move |x| x + n)
    }

    fn matches(&self) -> usize {
        self.winning.iter()
            .filter(|x| self.numbers.contains(x))
            .count()
    }

    fn calculate(&self) -> usize {
        match self.matches() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}
