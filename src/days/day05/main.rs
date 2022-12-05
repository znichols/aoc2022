use std::env;
use std::error::Error;
use std::fs;

fn parse_crates(s: &str) -> Vec<Box<Vec<char>>> {
    let mut lines: Vec<&str> = s.split('\n').collect();
    let num_stacks = lines[lines.len() - 1]
        .split_whitespace()
        .flat_map(|s| s.parse::<u32>())
        .max()
        .unwrap();

    let mut crate_stacks: Vec<Box<Vec<char>>> =
        (0..num_stacks).map(|_| Box::new(Vec::new())).collect();
    for i in (0..lines.len() - 1).rev() {
        let this_line = lines[i];
        for j in (0..this_line.len()).step_by(4) {
            let this_block = &this_line[j..j + 3];
            let block_char = this_block.chars().nth(1).unwrap();
            if block_char != ' ' {
                let v = &mut *crate_stacks[j / 4];
                v.push(block_char);
            }
        }
    }

    crate_stacks
}

fn move_single_crate(i: usize, j: usize, crates: &mut [Box<Vec<char>>]) {
    let ch: char;
    {
        let from_crate = &mut crates[i];
        ch = from_crate.pop().unwrap();
    }
    let to_crate = &mut crates[j];
    to_crate.push(ch);
}

fn move_mutiple_crates(i: usize, j: usize, n: usize, crates: &mut [Box<Vec<char>>]) {
    let mut tail: Vec<char>;
    {
        let from_crate = &mut crates[i];
        tail = from_crate.split_off(from_crate.len() - n);
    }
    let to_crate = &mut crates[j];
    to_crate.append(&mut tail);
}

fn move_crates(s: &str, crates: &mut Vec<Box<Vec<char>>>, grouped: bool) {
    for line in s.split('\n') {
        if line.is_empty() {
            continue;
        }
        let entries: Vec<_> = line.split_whitespace().collect();
        let times = entries[1].parse::<usize>().unwrap();
        let from = entries[3].parse::<usize>().unwrap() - 1;
        let to = entries[5].parse::<usize>().unwrap() - 1;
        if !grouped {
            for _ in 0..times {
                move_single_crate(from, to, crates);
            }
        } else {
            move_mutiple_crates(from, to, times, crates);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let input_split: Vec<_> = input.split("\n\n").collect();

    let mut crate_stacks = parse_crates(input_split[0]);
    let mut crate_stacks2 = crate_stacks.clone();
    move_crates(input_split[1], &mut crate_stacks, false);
    move_crates(input_split[1], &mut crate_stacks2, true);
    let top_crates: String = crate_stacks.iter().map(|v| v[v.len() - 1]).collect();
    let top_crates2: String = crate_stacks2.iter().map(|v| v[v.len() - 1]).collect();

    println!("{:?}", top_crates);
    println!("{:?}", top_crates2);

    Ok(())
}
