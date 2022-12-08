use std::env;
use std::error::Error;
use std::fs;

fn check_visibility(tree_heights: &[Vec<usize>]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let xmax = tree_heights.len();
    let ymax = tree_heights[0].len();
    let mut vis = vec![vec![0; ymax]; xmax];
    let mut scenic_scores = vec![vec![0; ymax]; xmax];

    for i in 0..xmax {
        for j in 0..ymax {
            if i == 0 || j == 0 || i == xmax - 1 || j == ymax - 1 {
                vis[i][j] = 1;
                continue;
            }
            let h = tree_heights[i][j];
            let mut dirs = vec![true; 4];
            let mut v_counts = vec![0; 4];
            for ii in (0..i).rev() {
                v_counts[0] += 1;
                if tree_heights[ii][j] >= h {
                    dirs[0] = false;
                    break;
                }
            }
            for ii in i + 1..xmax {
                v_counts[1] += 1;
                if tree_heights[ii][j] >= h {
                    dirs[1] = false;
                    break;
                }
            }
            for jj in (0..j).rev() {
                v_counts[2] += 1;
                if tree_heights[i][jj] >= h {
                    dirs[2] = false;
                    break;
                }
            }
            for jj in j + 1..ymax {
                v_counts[3] += 1;
                if tree_heights[i][jj] >= h {
                    dirs[3] = false;
                    break;
                }
            }
            if dirs.into_iter().reduce(|a, b| a || b).unwrap() {
                vis[i][j] = 1;
            }
            scenic_scores[i][j] = v_counts.iter().product();
        }
    }

    (vis, scenic_scores)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let tree_heights: Vec<Vec<usize>> = input
        .split('\n')
        .map(|l| l.split("").flat_map(|x| x.parse()).collect())
        .collect();

    let (visibility, scenic_scores) = check_visibility(&tree_heights);

    println!(
        "{:?}",
        visibility
            .iter()
            .map(|v| v.iter().sum::<usize>())
            .sum::<usize>()
    );
    println!(
        "{:?}",
        scenic_scores
            .iter()
            .flat_map(|v| v.iter().max())
            .max()
            .unwrap()
    );

    Ok(())
}
