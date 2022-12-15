use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct Sensor {
    loc: (i64, i64),
    beacon_loc: (i64, i64),
    radius: i64,
}

impl Sensor {
    fn from_str(s: &str) -> Sensor {
        let entries: Vec<_> = s.split_whitespace().collect();
        let s_pos = (
            entries[2][2..entries[2].len() - 1].parse::<i64>().unwrap(),
            entries[3][2..entries[3].len() - 1].parse::<i64>().unwrap(),
        );
        let b_pos = (
            entries[8][2..entries[8].len() - 1].parse::<i64>().unwrap(),
            entries[9][2..entries[9].len()].parse::<i64>().unwrap(),
        );
        let d = (s_pos.0 - b_pos.0).abs() + (s_pos.1 - b_pos.1).abs();
        Sensor {
            loc: s_pos,
            beacon_loc: b_pos,
            radius: d,
        }
    }

    fn within(&self, pt: &(i64, i64)) -> bool {
        let d = (pt.0 - self.loc.0).abs() + (pt.1 - self.loc.1).abs();
        d <= self.radius
    }

    fn border_pts(&self, min: i64, max: i64) -> HashSet<(i64, i64)> {
        let inside = |pt: &(i64, i64)| !(pt.0 < min || pt.1 < min || pt.0 > max || pt.1 > max);
        let mut r = HashSet::new();
        let mut pt = (self.loc.0, self.loc.1 + self.radius + 1);
        for dir in &[(-1, 1), (-1, -1), (1, -1), (1, 1)] {
            for _ in 0..self.radius + 1 {
                pt.0 += dir.0;
                pt.1 += dir.1;
                if inside(&pt) {
                    r.insert(pt);
                }
            }
        }
        r
    }
}

fn covered(sensors: &[Sensor], pt: &(i64, i64), exclude: bool) -> bool {
    let mut r = false;
    for sensor in sensors {
        if sensor.within(pt) {
            r = true;
            break;
        }
    }
    if r && exclude {
        for sensor in sensors {
            if pt == &sensor.loc || pt == &sensor.beacon_loc {
                return false;
            }
        }
    }
    r
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let y_pos = match args[1].contains("test") {
        true => 10,
        false => 2000000,
    };
    let search_max = match args[1].contains("test") {
        true => 20,
        false => 4000000,
    };

    let sensors: Vec<_> = input.split('\n').map(Sensor::from_str).collect();
    println!("{:?}", sensors);
    let xmin: i64 = sensors.iter().map(|b| b.loc.0 - b.radius).min().unwrap();
    let xmax: i64 = sensors.iter().map(|b| b.loc.0 + b.radius).max().unwrap();

    let s = (xmin..xmax + 1)
        .into_iter()
        .map(|x_pos| {
            let pt = (x_pos, y_pos);
            match covered(&sensors, &pt, true) {
                true => 1,
                false => 0,
            }
        })
        .sum::<i64>();
    println!("{:?}", s);

    let border_pts: HashSet<_> = sensors
        .iter()
        .map(|s| s.border_pts(0, search_max))
        .reduce(|a, b| a.union(&b).copied().collect::<HashSet<_>>())
        .unwrap();

    let valid_pts: Vec<_> = border_pts
        .iter()
        .filter(|&pt| !covered(&sensors, pt, false))
        .collect();
    println!("{:?}", valid_pts[0].0 * 4000000 + valid_pts[0].1);

    Ok(())
}
