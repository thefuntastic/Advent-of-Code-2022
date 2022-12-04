use std::ops::RangeInclusive;

fn main() {
    let _example = include_str!("../input/example.txt");
    let input = include_str!("../input/input.txt");

    //Part 1
    let total: usize = input
        .lines()
        .map(parse_lines)
        .filter_map(|vals| compare(vals))
        //.for_each(|opt| println!("{:?}", opt))
        .count();

    println!("Total {total}");

    //Part 2
    let total: usize = input
        .lines()
        .map(parse_lines)
        .filter(|vals| compare_any(vals))
        //.for_each(|opt| println!("{:?}", opt))
        .count();

    println!("Total {total}");
}

fn parse_lines(ln: &str) -> Values {
    let mut pairs = ln.split(",");
    Values {
        a: parse_pair(pairs.next().unwrap()),
        b: parse_pair(pairs.next().unwrap()),
    }
}

fn parse_pair(raw: &str) -> RangeInclusive<u32> {
    let mut range = raw.split("-");
    let (min, max) = (
        range.next().unwrap().parse::<u32>().unwrap(),
        range.next().unwrap().parse::<u32>().unwrap(),
    );
    min..=max
}

fn compare(vals: Values) -> Option<RangeInclusive<u32>> {
    if vals.a.contains(vals.b.start()) && vals.a.contains(vals.b.end()) {
        Some(vals.a)
    } else if vals.b.contains(vals.a.start()) && vals.b.contains(vals.a.end()) {
        Some(vals.b)
    } else {
        None
    }
}

fn compare_any(vals: &Values) -> bool {
    if vals.a.contains(vals.b.start())
        || vals.a.contains(vals.b.end())
        || vals.b.contains(vals.a.start())
        || vals.b.contains(vals.a.end())
    {
        return true;
    }
    false
}

struct Values {
    a: RangeInclusive<u32>,
    b: RangeInclusive<u32>,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_score() {
//
//     }
// }
