use std::fs;

pub fn solve() {
    part1();
    part2();
}

fn get_seed_maps() -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    let file = fs::read_to_string("../input/day5.txt").unwrap();
    let mut file = file.split("\n\n");
    let seeds = match file.next() {
        Some(seeds) => seeds.split(" ").filter_map(|x| x.parse::<usize>().ok()),
        None => panic!("No seeds provided"),
    }.collect();

    let maps = file.map(parse_map).map(|mut v|{
        v.sort_by_key(|(_, b, _)| *b);
        v
    }).collect();

    (seeds, maps)
}

fn part1() {
    let (seeds, maps) = get_seed_maps();

    let mut min = usize::MAX;
    for &seed in seeds.iter() {
        let aux = calculate_range_seed((seed, seed), &maps, 0);
        if aux < min { min = aux; }
    }
    //println!("{}", min);
}

fn part2() {
    let (seeds, maps) = get_seed_maps();

    let mut min = usize::MAX;
    for slice in seeds.chunks(2) {
        if let &[start, size] = slice {
            let next = calculate_range_seed((start, start + size - 1), &maps, 0);
            if next < min { min = next; }
        }
    }
    //println!("{}", min);
}

fn calculate_range_seed((start, end): (usize, usize), remaining_maps: &[Vec<(usize, usize, usize)>], i: usize) -> usize {
    let next = remaining_maps.get(i);

    if let Some(map) = next {
        let intervals = partition_intervals(start, end, map);
        let mut min = usize::MAX;
        for &interval in intervals.iter() {
            let possible = calculate_range_seed(interval, remaining_maps, i + 1);
            if possible < min { min = possible; }
        }
        return min;
    }
    return start;
}

fn partition_intervals(start: usize, end: usize, ranges: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut s_current = start;
    for i in 0..ranges.len() {
        if i < ranges.len() -1 {
            let this_end = ranges[i].1 + ranges[i].2 - 1;
            let next_start = ranges[i+1].1;
            //there is a hole between intervals
            //so we fill it with identity map
            if this_end + 1 != next_start {
                let this_end = usize::max(start, this_end + 1);
                let this_end_end = usize::min(end, next_start - 1);
                result.push((this_end, this_end_end));
            }
        }

        let range = ranges[i];
        let (mut s, mut e) = (range.1, range.1 + range.2 - 1);

        if e < start { continue; }
        s = usize::max(s, s_current);
        if s > end { continue; }
        e = usize::min(e, end);

        let long = e - s + 1;
        let diff = s as i64 - range.1 as i64;
        let starting = (range.0 as i64 + diff) as usize;
        result.push((starting, starting + long - 1));

        if i < ranges.len() - 1 { s_current = ranges[i + 1].1; }
    }

    if start < ranges[0].1 { result.push((start, usize::min(end, ranges[0].1 - 1))); }

    let last = ranges.last().unwrap();
    if end > (last.1 + last.2 - 1) { result.push((usize::max(start, last.1 + last.2), end)); }

    let result = result.into_iter().filter(|(s, e)| s <= e).collect();

    return result;
}

fn parse_map(s: &str) -> Vec<(usize, usize, usize)> {
    s.lines().skip(1).map(|x|{
        match x.split(" ")
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>()[..] {
                [end, start, size] => (end, start, size),
                _ => panic!("not valid seed map"),
            }
    }).collect()
}
