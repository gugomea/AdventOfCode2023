use std::fs;
use std::cmp::Ordering;

pub fn solve() {
    part1();
    part2();
}

fn part1() {
    // the only card that is not in ascii order is J, so I replace it with R, with follows that
    // order, so I don't have to create an Enum.
    let file = fs::read_to_string("../input/day7.txt").unwrap().replace("J", "R");
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
    //Since the jocker is going to be the worst card, when sorting we want it to be "0" 
    //or less(I picked "0").
    let file = fs::read_to_string("../input/day7.txt").unwrap().replace("J", "0");
    let mut hands = file.lines()
        .map(|x| Hand::parse_hand(x, false))
        .collect::<Vec<_>>();

    hands.sort();

    let total = hands.into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i + 1) * x.bid);
    println!("{}", total);
}

#[derive(Debug)]
struct Hand {
    cards: Cards,
    bid: usize,
}

impl Hand {

    fn parse_hand(input: &str, part1: bool) -> Self {
        let mut split = input.split(" ");
        let (cards, bid) = (
            Cards::new(split.next().unwrap().chars().collect(), part1),
            split.next().unwrap().parse::<usize>().unwrap());

        Self { cards, bid }
    }
}

impl PartialEq for Hand {

    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cards.partial_cmp(&other.cards)
    }
}

impl Ord for Hand{

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Cards {
    values: Vec<char>,
    typ: usize,
}

impl Cards {

    fn new(values: Vec<char>, part1: bool) -> Self {
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
            let better = c.iter().find(|&&x| x != '0').copied().unwrap_or('A');

            for i in 0..values.len() {
                if c[i] == '0' { c[i] = better; }
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

impl PartialEq for Cards {

    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ && self.values == other.values
    }
}

impl Eq for Cards {}

impl PartialOrd for Cards {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // if the type is already different we don't event have 
        // to check for the cards
        if other.typ != self.typ {
            return Some(self.typ.cmp(&other.typ))
        } else {
            for (c1, c2) in std::iter::zip(&self.values, &other.values) {
                // if different cards:
                //    [ ] If both are letters we compare lexigographically
                //    [ ] If one of them is number, then the unicode value gives the propper ordering
                // else:
                //    [ ] Keep searching for different card
                match (c1 != c2, !c1.is_digit(10) && !c2.is_digit(10)) {
                    (true, true) => return c2.partial_cmp(&c1),
                    (true, _) => return c1.partial_cmp(&c2),
                    _ => continue,
                }
            }
            return None;
        }
    }
}

impl Ord for Cards {

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

