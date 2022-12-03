use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

fn sack_intersection(contents: &str) -> HashSet<char> {
    let l = contents.len() / 2;
    let left_set: HashSet<char> = contents[..l].chars().collect();
    let right_set: HashSet<char> = contents[l..].chars().collect();
    left_set.intersection(&right_set).cloned().collect()
}

fn badge_intersection(sacks: &[&str]) -> HashSet<char> {
    let mut x: HashSet<char> = sacks[0].chars().collect();
    x = x
        .intersection(&sacks[1].chars().collect::<HashSet<char>>())
        .cloned()
        .collect();
    x.intersection(&sacks[2].chars().collect::<HashSet<char>>())
        .cloned()
        .collect()
}

fn score(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let lines = input.split('\n').collect::<Vec<_>>();

    let scores = lines
        .iter()
        .map(|&s| sack_intersection(s))
        .map(|mut s| score(s.drain().next().unwrap()));

    let badge_scores = lines
        .chunks(3)
        .map(badge_intersection)
        .map(|mut s| score(s.drain().next().unwrap()));

    println!("{:?}", scores.sum::<u32>());
    println!("{:?}", badge_scores.sum::<u32>());

    Ok(())
}
