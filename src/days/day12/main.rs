use std::env;
use std::error::Error;
use std::fs;
use std::{cell::RefCell, rc::Rc};

type DistMap = Rc<RefCell<Vec<Vec<usize>>>>;

fn parse_input(input: &str) -> ((usize, usize), (usize, usize), Vec<Vec<usize>>) {
    let mut pos = (0, 0);
    let mut goal = (0, 0);
    let mut r = vec![];

    for (i, line) in input.split('\n').enumerate() {
        let mut this_row = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    pos = (i, j);
                    this_row.push(0);
                }
                'E' => {
                    goal = (i, j);
                    this_row.push(25);
                }
                _ => this_row.push(c as usize - 'a' as usize),
            }
        }
        r.push(this_row);
    }
    (pos, goal, r)
}

fn neighbors(map: &[Vec<usize>], pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut r = vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)];
    if pos.0 > 0 {
        r.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        r.push((pos.0, pos.1 - 1));
    }
    let xmax = map.len();
    let ymax = map[0].len();
    r.into_iter()
        .filter(|new_p| {
            new_p.0 < xmax && new_p.1 < ymax && map[pos.0][pos.1] <= (map[new_p.0][new_p.1] + 1)
        })
        .collect()
}

fn path_out(
    map: &[Vec<usize>],
    pos: (usize, usize),
    new_pos: (usize, usize),
    dist_from_goal: DistMap,
) {
    let new_dist = dist_from_goal.borrow()[pos.0][pos.1] + 1;
    if new_dist < dist_from_goal.borrow()[new_pos.0][new_pos.1] {
        dist_from_goal.borrow_mut()[new_pos.0][new_pos.1] = new_dist;
        for p in neighbors(map, new_pos) {
            path_out(map, new_pos, p, dist_from_goal.clone());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let (pos, goal, mut map) = parse_input(&input);

    let dist_map: DistMap = Rc::new(RefCell::new(vec![
        vec![usize::MAX; map[0].len()];
        map.len()
    ]));
    dist_map.borrow_mut()[goal.0][goal.1] = 0;
    for p in neighbors(&map, goal) {
        path_out(&map, goal, p, dist_map.clone());
    }

    println!("{:?}", dist_map.borrow()[pos.0][pos.1]);

    let mut ground_locs: Vec<(usize, usize)> = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                ground_locs.push((i, j));
            }
        }
    }
    println!(
        "{:?}",
        ground_locs
            .iter()
            .map(|p| dist_map.borrow()[p.0][p.1])
            .min()
            .unwrap()
    );

    Ok(())
}
