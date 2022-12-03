use std::collections::HashSet;

fn main() {
    let example = include_str!("../input/example.txt");
    let input = include_str!("../input/input.txt");

    //Part 1
    let total: u32 = input
        .lines()
        .map(parse_lines)
        .map(|(a, b)| compare(a, b))
        .map(score)
        .sum();

    println!("Total {total}");

    //From other solutions
    //input.lines().array_chunks::<3>()

    let lines: Vec<&str> = input.lines().collect();
    let lns = &lines[0..];
    let total: u32 = lns
        .chunks(3)
        .map(|group| parse_common(group))
        .map(|ch| score(ch))
        .sum();

    println!("Total {total}");
}

fn parse_lines(ln: &str) -> (&str, &str) {
    //From other solutions
    //ln.split_at(mid)

    let mid = ln.len() / 2;

    let a = &ln[0..mid];
    let b = &ln[mid..];
    assert_eq!(a.len(), b.len());
    (a, b)
}

fn compare(a: &str, b: &str) -> char {
    let set: HashSet<char> = a.chars().collect();

    let common = b.chars().filter(|ch| set.contains(ch)).next();

    common.unwrap()
}

fn score(n: char) -> u32 {
    if n.is_ascii_lowercase() {
        (n as u32) - ('a' as u32) + 1
    } else if n.is_ascii_uppercase() {
        (n as u32) - ('A' as u32) + 1 + 26
    } else {
        eprintln!("Something has gone wrong!");
        0
    }
}

fn parse_common(group: &[&str]) -> char {
    let set_b: HashSet<char> = group[1].chars().collect();
    let set_c: HashSet<char> = group[2].chars().collect();

    //From other solutions
    //set_b.intersection(set_c)

    let result = group[0]
        .chars()
        .filter(|ch| set_b.contains(ch))
        .filter(|ch| set_c.contains(ch))
        .next();

    if let Some(ch) = result {
        return ch.clone();
    } else {
        eprintln!("Something has gone wrong!");
        return '0';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score('a'), 1);
        assert_eq!(score('z'), 26);
        assert_eq!(score('A'), 27);
        assert_eq!(score('Z'), 52);
    }
}
