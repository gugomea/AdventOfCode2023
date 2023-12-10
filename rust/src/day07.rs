use std::fs;
use std::cmp::Ordering;

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    // the only card that is not in ascii order is J, so I replace it with R, with follows that
    // order, so I don't have to create an Enum.
    let file = fs::read_to_string("../input/day7.txt").unwrap();
    let mut hands = file.lines()
        .map(|x| Hand::parse_hand(x, true))
        .collect::<Vec<_>>();

    //sort them by rank
    hands.sort();

    let total = hands.into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i + 1) * x.bid);
    println!("{}", total);
}

fn part2() {
    let file = fs::read_to_string("../input/day7.txt").unwrap().replace("J", "Ñ");
    let mut hands = file.lines()
        .map(|x| Hand::parse_hand(x, false))
        .collect::<Vec<_>>();

    hands.sort();

    let total = hands.into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i + 1) * x.bid);
    println!("{}", total);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Cards,
    bid: usize,
}

impl Hand {

    fn parse_hand(input: &str, part1: bool) -> Self {
        let (c, b)  = input.split_once(' ').unwrap();
        let (cards, bid) = (
            Cards::new(c.chars().map(Card::from_ch).collect(), part1),
            b.parse().unwrap());

        Self { cards, bid }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cards {
    typ: usize,
    values: Vec<Card>,
}

impl Cards {

    fn new(values: Vec<Card>, part1: bool) -> Self {
        let mut c = values.clone();
        //sort by the number of occurences
        c.sort_by(|a, b| {
            let n = values.iter()
                .filter(|&x| x == b)
                .count()
                .cmp(&values.iter().filter(|&x|x == a).count());
            if n == Ordering::Equal { return b.cmp(a); }
            return n;
        });


        if !part1 {
            let better = c.iter().find(|&&x| x != Card::Joker).copied().unwrap_or(Card::A);

            for i in 0..values.len() {
                if c[i] == Card::Joker { c[i] = better; }
            }


            //sort, having replaced jokers with the most repeated value.
            c.sort_by(|a, b| {
                let n = values.iter()
                    .filter(|&x| x == b)
                    .count()
                    .cmp(&values.iter().filter(|&x|x == a).count());
                if n == Ordering::Equal { return b.cmp(a); }
                return n
            });
        }

        //the number of the type is the index in this array.
        let typ = [
            c[0] != c[1], // High card
            c[0] == c[1], // One pair
            c[0] == c[1] && (c[2] == c[3] || c[3] == c[4]), // Two pair
            c[0] == c[2], // Three of a kind
            c[0] == c[2] && c[3] == c[4], //Full house
            c[0] == c[3], // Four of a kind
            c[0] == c[4] // Five of a kind
        ].into_iter().rposition(|x| x).unwrap();

        Self { values, typ }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Two, Three, Four, Five, Six, Seven, Eight, Nine,
    T, J, Q, K, A,
}

impl Card {
    fn from_ch(ch: char) -> Self {
        match ch {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'K' => Card::K,
            'Q' => Card::Q,
            'A' => Card::A,
            'Ñ' => Card::Joker,
            ch => panic!("{}", ch),
        }
    }
}
