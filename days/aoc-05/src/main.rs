#![feature(iter_array_chunks)]
fn main() {
    let _example = include_str!("../input/example.txt");
    let input = include_str!("../input/input.txt");

    let raw: Vec<&str> = input.split("\r\n\r\n").collect();

    let moves: Vec<Move> = raw[1].lines().map(parse_move).collect();
    println!("{moves:?}");

    let mut map = parse_map(raw[0]);
    let mut map_2 = map.clone();

    for mv in moves.iter() {
        // println!("Performing move \n{map:#?} \n\n {mv:?}");
        map.perform_move(mv);
    }
    println!("{}", map.print_top());

    for mv in moves.iter() {
        map_2.perform_move_2(mv);
    }

    println!("{}", map_2.print_top());
}

fn parse_map(raw: &str) -> Map {
    let mut lines = raw.lines().rev();

    // Get number of columns
    let count = lines
        .next()
        .map(|ln| ln.matches(char::is_numeric).count())
        .unwrap();
    println! {"{count:?}"}

    let mut map = Map::new(count);

    for ln in lines {
        let row: Vec<Position> = ln.as_bytes().chunks(4).map(parse_create).collect();

        for (i, item) in row.iter().enumerate() {
            if let Position::Crate(val) = item {
                map.add_crate(i, val.clone())
            }
        }
    }

    map
}

fn parse_create(raw: &[u8]) -> Position {
    let empty = ' ' as u8;
    if raw[0] != empty {
        Position::Crate(char::from(raw[1]))
    } else {
        Position::Empty
    }
}

fn parse_move(ln: &str) -> Move {
    let results: Vec<usize> = ln
        .split(" ")
        .filter_map(|item| item.parse::<usize>().ok())
        .collect();

    assert_eq!(3, results.len());

    Move {
        count: results[0],
        from: results[1],
        to: results[2],
    }
}

#[derive(Debug)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
pub struct Map {
    stacks: Vec<Vec<char>>,
}

impl Map {
    fn new(n_stacks: usize) -> Map {
        let mut stacks: Vec<Vec<char>> = vec![];
        for i in 0..n_stacks {
            stacks.push(vec![]);
        }
        Map { stacks }
    }

    fn add_crate(&mut self, index: usize, val: char) {
        self.stacks[index].push(val);
    }

    pub fn perform_move(&mut self, mov: &Move) {
        let items: Vec<char> = {
            let from = &mut self.stacks[mov.from - 1];
            let start: usize = from.len() - mov.count;
            // println!(
            //     "From {} len {} start {} count {}",
            //     mov.from - 1,
            //     from.len(),
            //     from.len() - mov.count,
            //     mov.count
            // );
            from.drain(start..).collect()
        };

        let to = &mut self.stacks[mov.to - 1];
        to.extend(items.iter().rev());
    }

    pub fn perform_move_2(&mut self, mov: &Move) {
        let items: Vec<char> = {
            let from = &mut self.stacks[mov.from - 1];
            let start: usize = from.len() - mov.count;
            // println!(
            //     "From {} len {} start {} count {}",
            //     mov.from - 1,
            //     from.len(),
            //     from.len() - mov.count,
            //     mov.count
            // );
            from.drain(start..).collect()
        };

        let to = &mut self.stacks[mov.to - 1];
        to.extend(items.iter());
    }

    pub fn print_top(&self) -> String {
        let mut output = "".to_string();

        self.stacks
            .iter()
            .for_each(|item| output.push(*item.last().unwrap()));

        output
    }
}

enum Position {
    Crate(char),
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move() {
        let raw = "move 13 from 9 to 1";
        let mov = parse_move(raw);
        assert_eq!(13, mov.count);
        assert_eq!(9, mov.from);
        assert_eq!(1, mov.to);
    }
}
