#![feature(slice_group_by)]

#[derive(Copy, Clone)]
pub enum Choice {
    RockAX,
    PaperBY,
    ScissorsCY,
}

#[derive(Copy, Clone)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Copy, Clone)]
pub struct Round {
    a: Choice,
    b: Choice,
}

#[derive(Copy, Clone)]
pub struct RoundTarget {
    a: Choice,
    b: Outcome,
}

fn score_choice(choice: Choice) -> u32 {
    match choice {
        Choice::RockAX => 1,
        Choice::PaperBY => 2,
        Choice::ScissorsCY => 3,
    }
}

fn score_outcome(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3,
    }
}

impl From<char> for Choice {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Choice::RockAX,
            'B' | 'Y' => Choice::PaperBY,
            'C' | 'Z' => Choice::ScissorsCY,
            _ => {
                eprintln!("somehtings gone very wrong");
                Choice::RockAX
            }
        }
    }
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => {
                eprintln!("somehtings gone very wrong");
                Outcome::Lose
            }
        }
    }
}

fn get_outcome(a: Choice, b: Choice) -> Outcome {
    match (a, b) {
        (Choice::RockAX, Choice::PaperBY) => Outcome::Win,
        (Choice::RockAX, Choice::RockAX) => Outcome::Draw,
        (Choice::RockAX, Choice::ScissorsCY) => Outcome::Lose,

        (Choice::PaperBY, Choice::RockAX) => Outcome::Lose,
        (Choice::PaperBY, Choice::PaperBY) => Outcome::Draw,
        (Choice::PaperBY, Choice::ScissorsCY) => Outcome::Win,

        (Choice::ScissorsCY, Choice::RockAX) => Outcome::Win,
        (Choice::ScissorsCY, Choice::ScissorsCY) => Outcome::Draw,
        (Choice::ScissorsCY, Choice::PaperBY) => Outcome::Lose,
    }
}

fn parse_line(ln: &str) -> (char, char) {
    let mut result = ('M', 'M');

    for (i, char) in ln.chars().enumerate() {
        if i == 0 {
            result.0 = char;
        } else if i == 2 {
            result.1 = char;
        }
    }

    result
}

fn choose_strategy(desired: RoundTarget) -> Round {
    let choice = match desired.b {
        Outcome::Win => match desired.a {
            Choice::RockAX => Choice::PaperBY,
            Choice::PaperBY => Choice::ScissorsCY,
            Choice::ScissorsCY => Choice::RockAX,
        },
        Outcome::Lose => match desired.a {
            Choice::RockAX => Choice::ScissorsCY,
            Choice::PaperBY => Choice::RockAX,
            Choice::ScissorsCY => Choice::PaperBY,
        },
        Outcome::Draw => desired.a,
    };

    Round {
        a: desired.a,
        b: choice,
    }
}

fn score(round: Round) -> u32 {
    score_outcome(get_outcome(round.a, round.b)) + score_choice(round.b)
}

pub struct Play {}

fn main() {
    let example = include_str!("../input/example.txt");
    let input = include_str!("../input/input.txt");

    let lines: Vec<&str> = input.lines().collect();

    // Part 1
    let x = lines
        .iter()
        .map(|ln| parse_line(ln))
        .map(|(a, b)| Round {
            a: Choice::from(a),
            b: Choice::from(b),
        })
        .map(|round| score(round))
        .sum::<u32>();

    println!("Total {}", x);

    // Part 2
    let x = lines
        .iter()
        .map(|ln| parse_line(ln))
        .map(|(a, b)| RoundTarget {
            a: Choice::from(a),
            b: Outcome::from(b),
        })
        .map(|desired| choose_strategy(desired))
        .map(|round| score(round))
        .sum::<u32>();

    println!("Total strat {}", x);
}
