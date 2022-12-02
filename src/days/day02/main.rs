use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

#[derive(PartialEq, Eq, Hash)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

fn score_move(
    move_line: &str,
    left_map: &HashMap<char, Throw>,
    right_map: &HashMap<char, Throw>,
    throw_scores: &HashMap<Throw, i32>,
) -> i32 {
    let left_throw = left_map.get(&move_line.chars().next().unwrap()).unwrap();
    let right_throw = right_map.get(&move_line.chars().nth(2).unwrap()).unwrap();

    let right_score = *throw_scores.get(right_throw).unwrap();
    let left_score = *throw_scores.get(left_throw).unwrap();
    let mut score = right_score;

    if right_score == left_score {
        score += 3;
    } else if right_score - left_score == 1 || left_score - right_score == 2 {
        score += 6;
    }
    score
}

fn score_move2(
    move_line: &str,
    left_map: &HashMap<char, Throw>,
    throw_scores: &HashMap<Throw, i32>,
) -> i32 {
    let left_throw = left_map.get(&move_line.chars().next().unwrap()).unwrap();
    let left_score = *throw_scores.get(left_throw).unwrap();
    let mut right_score = match move_line.chars().nth(2).unwrap() {
        'X' => left_score - 1,
        'Y' => left_score,
        _ => (left_score + 1) % 3,
    };
    if right_score == 0 {
        right_score = 3;
    }

    let mut score = right_score;

    if right_score == left_score {
        score += 3;
    } else if right_score - left_score == 1 || left_score - right_score == 2 {
        score += 6;
    }
    score
}

fn main() -> Result<(), Box<dyn Error>> {
    let throw_scores = HashMap::from([(Throw::Rock, 1), (Throw::Paper, 2), (Throw::Scissors, 3)]);
    let left_map = HashMap::from([
        ('A', Throw::Rock),
        ('B', Throw::Paper),
        ('C', Throw::Scissors),
    ]);
    let right_map = HashMap::from([
        ('X', Throw::Rock),
        ('Y', Throw::Paper),
        ('Z', Throw::Scissors),
    ]);

    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let scores = input
        .split('\n')
        .map(|s| score_move(s, &left_map, &right_map, &throw_scores));
    let scores2 = input
        .split('\n')
        .map(|s| score_move2(s, &left_map, &throw_scores));

    println!("{:?}", scores.sum::<i32>());
    println!("{:?}", scores2.sum::<i32>());

    Ok(())
}
