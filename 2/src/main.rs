use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_file_lines<T>(f: T) -> io::Lines<io::BufReader<File>>
where
    T: AsRef<Path>,
{
    let file = File::open(f).unwrap();
    io::BufReader::new(file).lines()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}
impl Moves {
    fn beats(&self) -> Self {
        use Moves::*;
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        }
    }

    fn loses_to(&self) -> Self {
        use Moves::*;
        match *self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }

    fn from_result(&self, wld: &WLD) -> Self {
        match wld {
            WLD::Win => self.loses_to(),
            WLD::Lose => self.beats(),
            _ => *self
        }
    }

    fn to_wld(&self, other: &Moves) -> WLD {
        match (self, other) {
            (a, b) if a == b => WLD::Draw,
            (a, b) if &a.beats() == b => WLD::Win,
            _ => WLD::Lose
        }
    }
}

enum WLD {
    Win,
    Lose,
    Draw,
}


impl WLD {
    fn add_num(&self, m: &Moves) -> i32 {
        match self {
            WLD::Win => 6 + get_move_val(m),
            WLD::Draw => 3 + get_move_val(m),
            WLD::Lose => get_move_val(m),
        }
    }

}

fn get_move_val(m: &Moves) -> i32 {
    match m {
        Moves::Rock => 1,
        Moves::Paper => 2,
        Moves::Scissors => 3,
    }
}


fn get_my_move(s: &String) -> Moves {
    match s {
        x if x.contains("X") => Moves::Rock,
        x if x.contains("Y") => Moves::Paper,
        x if x.contains("Z") => Moves::Scissors,
        _ => panic!("Invalid move"),
    }
}

fn get_desired_outcome(s: &String) -> WLD {
    match s {
        x if x.contains("X") => WLD::Lose,
        x if x.contains("Y") => WLD::Draw,
        x if x.contains("Z") => WLD::Win,
        _ => panic!("Invalid input")
    }
}

fn get_your_move(s: &String) -> Moves {
    match s {
        x if x.contains("A") => Moves::Rock,
        x if x.contains("B") => Moves::Paper,
        x if x.contains("C") => Moves::Scissors,
        _ => panic!("Invalid move"),
    }
}

fn main() {
    let mut score = 0;
    let mut scorept2 = 0;
    for line in read_file_lines("./input.txt") {
        if let Ok(l) = line {
            let my_move = get_my_move(&l);
            let your_move = get_your_move(&l);
            let wld = my_move.to_wld(&your_move);
            score += wld.add_num(&my_move);

            // pt2
            let desired = get_desired_outcome(&l);
            let my_pt2_move = your_move.from_result(&desired);
            scorept2 += desired.add_num(&my_pt2_move);
        }
    }
    println!("{:?}", score);
    println!("{:?}", scorept2);
}
