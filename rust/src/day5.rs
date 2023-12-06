use std::fs;

pub fn solve() {
    part1();
    part2();
}

fn part2() {
    //let file = fs::read_to_string("../input/test.txt").unwrap();
    let file = fs::read_to_string("../input/day5.txt").unwrap();
    let mut file = file.split("\n\n");
    let seeds = match file.next() {
        Some(seeds) => seeds.split(" ").filter_map(|x| x.parse::<usize>().ok()),
        None => panic!("No seeds provided"),
    }.collect::<Vec<_>>();

    let maps = file.map(parse_map).map(|mut v|{
        v.sort_by_key(|(_, b, _)| *b);
        v
    }).collect::<Vec<_>>();

    //let aux = maps.clone()
    //    .into_iter()
    //    .for_each(|v|{
    //        let v = v.into_iter()
    //         .flat_map(|(_a, b,c)| vec![b, b + c - 1])
    //         .collect::<Vec<_>>();
    //        let (mut i, mut j) = (1, 2);
    //        for _ in 0..((v.len() / 2) - 1) {
    //            let pred = v[i] +1 == v[j];
    //            println!("{}", pred);
    //            i += 2;
    //            j += 2;
    //        }
    //    });

    println!("empezamos");
    let mut min = usize::MAX;
    for slice in seeds.chunks(2) {
        if let &[start, size] = slice {
            let next = calculate_range_seed((start, start + size - 1), &maps, 0);
            if next < min { min = next; }
        }
    }
    println!("{}", min);
    //println!("{:?}", seeds.chunks(2).collect::<Vec<_>>());
}

fn calculate_range_seed((start, end): (usize, usize), remaining_maps: &[Vec<(usize, usize, usize)>], i: usize) -> usize {
    let next = remaining_maps.get(i);

    if let Some(map) = next {
        //println!("Dado el intervalo ({}, {}) y el mapa {:?}", start, end, map);
        let intervals = partition_intervals(start, end, map);
        //println!(">> {:?}", intervals);
        let mut min = usize::MAX;
        for &interval in intervals.iter() {
            let possible = calculate_range_seed(interval, remaining_maps, i + 1);
            if possible == 0 { println!("hola"); }
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

    let mut first_padd = None;
    let mut last_padd = None;

    if start < ranges[0].1 {
        first_padd = Some((start, usize::min(end, ranges[0].1 - 1)));
    }

    let last = ranges.last().unwrap();

    if end > (last.1 + last.2 - 1) {
        last_padd = Some((usize::max(start, last.1 + last.2), end));
    }

    if let Some(first_padd) = first_padd { result.push(first_padd); }
    if let Some(last_padd) = last_padd { result.push(last_padd); }
    //if first_padd != last_padd { result.push(last_padd); }
    let result = result.into_iter().filter(|(s, e)| s <= e).collect();
    //println!(">> {:?}", result);

    return result;
}

fn part1() {
    let file = fs::read_to_string("../input/day5.txt").unwrap();
    let mut file = file.split("\n\n");
    let seeds = match file.next() {
        Some(seeds) => seeds.split(" ").filter_map(|x| x.parse::<usize>().ok()),
        None => panic!("No seeds provided"),
    }.collect::<Vec<_>>();
    let maps = file.map(parse_map).collect::<Vec<_>>();

    let mut min = usize::MAX;
    for seed in seeds.iter() {
        let aux = calculate_seed(*seed, &maps, 0);
        if aux < min { min = aux; }
    }
    println!("{}", min);
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

fn calculate_seed(seed: usize, remaining_maps: &[Vec<(usize, usize, usize)>], i: usize) -> usize {
    let next = remaining_maps.get(i);

    if let Some(map) = next {
        for sentence in map {
            let (a, b, c) = *sentence;
            let valid = seed >= b && seed <= (b + c);
            if  valid { 
                let next = ( seed - b ) + a;
                return calculate_seed(next, remaining_maps, i + 1);
            }
        }
        return calculate_seed(seed, remaining_maps, i + 1);
    }

    return seed;
}
