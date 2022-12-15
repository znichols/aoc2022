use ndarray::prelude::*;
use std::env;
use std::error::Error;
use std::fs;

fn drop(a: &mut Array2<i32>) -> bool {
    let mut pos: (usize, usize) = (500, 0);
    while pos.1 < 499 {
        if a[[pos.0, pos.1 + 1]] == 0 {
            pos.1 += 1;
        } else if a[[pos.0 - 1, pos.1 + 1]] == 0 {
            pos.0 -= 1;
            pos.1 += 1;
        } else if a[[pos.0 + 1, pos.1 + 1]] == 0 {
            pos.0 += 1;
            pos.1 += 1;
        } else {
            a[[pos.0, pos.1]] = 2;
            if pos == (500, 0) {
                return false;
            }
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let mut a = Array2::zeros((700, 500));
    for line in input.split('\n') {
        let mut p_iter = line.split(" -> ");
        let mut pt: Vec<_> = p_iter
            .next()
            .map(|s| s.split(',').flat_map(|e| e.parse::<usize>()).collect())
            .unwrap();
        for new_pt_str in p_iter {
            let new_pt: Vec<_> = new_pt_str
                .split(',')
                .flat_map(|e| e.parse::<usize>())
                .collect();
            if new_pt[0] > pt[0] || new_pt[1] > pt[1] {
                a.slice_mut(s![pt[0]..new_pt[0] + 1, pt[1]..new_pt[1] + 1])
                    .fill(1);
            } else {
                a.slice_mut(s![new_pt[0]..pt[0] + 1, new_pt[1]..pt[1] + 1])
                    .fill(1);
            }
            pt = new_pt;
        }
    }

    let mut i = 0;
    while drop(&mut a) {
        i += 1;
    }
    println!("{}", i);

    let ymax = a
        .sum_axis(Axis(0))
        .indexed_iter()
        .filter(|x| *x.1 > 0)
        .map(|x| x.0)
        .max()
        .unwrap();
    a.slice_mut(s![.., ymax + 2]).fill(1);

    i += 1;
    while drop(&mut a) {
        i += 1;
    }
    println!("{}", i);

    Ok(())
}
