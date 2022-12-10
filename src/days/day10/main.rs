use std::env;
use std::error::Error;
use std::fs;

fn process(instructions: &[&str]) -> Vec<i64> {
    let mut signal: Vec<i64> = vec![];
    let mut register: i64 = 1;
    for &line in instructions {
        let words: Vec<_> = line.split_whitespace().collect();
        match words[0] {
            "noop" => signal.push(register),
            _ => {
                let x = words[1].parse::<i64>().unwrap();
                signal.push(register);
                signal.push(register);
                register += x;
            }
        }
    }
    signal
}

fn draw_signal(signal: &[i64]) -> String {
    let hlines = 40;
    let vlines = 6;
    let mut line_vals = vec![vec![' '; hlines]; vlines];
    for i in 0..vlines {
        for j in 0..hlines {
            let signal_ind = i * hlines + j;
            if (signal[signal_ind] - j as i64).abs() < 2 {
                line_vals[i][j] = '#';
            }
        }
    }

    line_vals
        .iter()
        .map(|chars| chars.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let instructions: Vec<_> = input.split('\n').collect();

    let signal = process(&instructions);

    let poll = vec![20, 60, 100, 140, 180, 220];
    println!(
        "{:?}",
        poll.iter().map(|&i| i as i64 * signal[i - 1]).sum::<i64>()
    );

    println!("{}", draw_signal(&signal));

    Ok(())
}
