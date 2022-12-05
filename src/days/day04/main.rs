use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let assignment_pairs = input
        .split('\n')
        .map(|l| {
            l.split(',')
                .map(|s| {
                    s.split('-')
                        .flat_map(|t| t.parse::<u32>())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let overlap_sum = assignment_pairs
        .iter()
        .map(|p| {
            if p[0][0] <= p[1][0] && p[0][1] >= p[1][1] {
                1
            } else if p[0][0] >= p[1][0] && p[0][1] <= p[1][1] {
                1
            } else {
                0
            }
        })
        .sum::<u32>();

    let partial_overlap_sum = assignment_pairs
        .iter()
        .map(|p| {
            if p[0][1] < p[1][0] || p[1][1] < p[0][0] {
                0
            } else {
                1
            }
        })
        .sum::<u32>();

    println!("{:?}", overlap_sum);
    println!("{:?}", partial_overlap_sum);

    Ok(())
}
