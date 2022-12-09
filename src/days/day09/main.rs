use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

fn stick_together(start: &[i32], end: &mut [i32]) {
    let hdiff: i32 = start[0] - end[0];
    let vdiff: i32 = start[1] - end[1];
    if hdiff.abs() > 1 || vdiff.abs() > 1 {
        end[0] += hdiff.signum();
        end[1] += vdiff.signum();
    }
}

fn tail_wag(move_seq: &[&str], tail_len: usize) -> HashSet<(i32, i32)> {
    let mut head_pos = vec![0, 0];
    let mut tail_pos_seq = vec![vec![0, 0]; tail_len];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    for &dir in move_seq.iter() {
        match dir {
            "R" => head_pos[0] += 1,
            "L" => head_pos[0] -= 1,
            "U" => head_pos[1] += 1,
            _ => head_pos[1] -= 1,
        };
        stick_together(&head_pos, tail_pos_seq.get_mut(0).unwrap());
        for i in 1..tail_pos_seq.len() {
            let start = tail_pos_seq[i - 1].clone();
            let end = &mut tail_pos_seq[i];
            stick_together(&start, end);
        }
        visited.insert((
            tail_pos_seq[tail_pos_seq.len() - 1][0],
            tail_pos_seq[tail_pos_seq.len() - 1][1],
        ));
    }
    visited
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let move_seq: Vec<_> = input
        .split('\n')
        .flat_map(|line| {
            let mut s = line.split_whitespace();
            let dir = s.next().unwrap();
            let times = s.next().unwrap().parse::<usize>().unwrap();
            vec![dir; times]
        })
        .collect();

    let visited_tail_1 = tail_wag(&move_seq, 1);
    let visited_tail_9 = tail_wag(&move_seq, 9);
    println!("{:?}", visited_tail_1.len());
    println!("{:?}", visited_tail_9.len());

    Ok(())
}
