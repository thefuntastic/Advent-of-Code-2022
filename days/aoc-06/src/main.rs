#![feature(iter_array_chunks)]

use iter_tools::Itertools;
use std::collections::HashSet;

fn main() {
    let _example = include_str!("../input/example.txt");
    let input = include_str!("../input/input.txt");

    let mut set = HashSet::<char>::new();

    //Part 1
    let input: Vec<char> = input.chars().collect();
    let index: usize = input
        .iter()
        .tuple_windows()
        .enumerate()
        .find_map(|(i, (a, b, c, d))| {
            set.clear();
            set.insert(*a);
            set.insert(*b);
            set.insert(*c);
            set.insert(*d);
            if set.len() == 4 {
                return Some(i + 4);
            } else {
                return None;
            }
        })
        .unwrap();

    println!("First index {}", index);

    //Pro tips
    //window.iter().unique().count() instead of the set, but does the same internally anyway, use a set

    //Part 2 - fine, make me do it properly
    let index: usize = input
        .windows(14)
        .position(|items| {
            set.clear();
            set.extend(items);
            return set.len() == 14;
        })
        .map(|i| i + 14)
        .unwrap();

    println!("Second index {}", index);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_parse_move() {
//         let raw = "move 13 from 9 to 1";
//         let mov = parse_move(raw);
//         assert_eq!(13, mov.count);
//         assert_eq!(9, mov.from);
//         assert_eq!(1, mov.to);
//     }
// }
