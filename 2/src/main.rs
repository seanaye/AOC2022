use std::{
    fs::{self, File},
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

enum WLD {
    Win(Moves),
    Lose(Moves),
    Draw(Moves),
}
impl WLD {
    fn to_num(&self) -> i32 {
        match self {
            WLD::Win(x) => 6 + get_move_val(x),
            WLD::Draw(x) => 3 + get_move_val(x),
            WLD::Lose(x) => get_move_val(x),
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

fn get_score(me: &Moves, you: &Moves) -> WLD {
    if me == you {
        return WLD::Draw(me.clone());
    }
    match (me, you) {
        (Moves::Paper, Moves::Rock) => WLD::Win(me.clone()),
        (Moves::Rock, Moves::Scissors) => WLD::Win(me.clone()),
        (Moves::Scissors, Moves::Paper) => WLD::Win(me.clone()),
        _ => WLD::Lose(me.clone()),
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
    for line in read_file_lines("./input.txt") {
        if let Ok(l) = line {
            let my_move = get_my_move(&l);
            let your_move = get_your_move(&l);
            let wld = get_score(&my_move, &your_move);
            score += wld.to_num()
        }
    }
    println!("{:?}", score)
}
