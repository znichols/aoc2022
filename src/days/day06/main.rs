use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let mut i = 0;
    while true {
        let chars: HashSet<char> = input[i..i + 4].chars().collect();
        if chars.len() == 4 {
            println!("{:?}", &input[i..i + 4]);
            break;
        }
        i += 1;
    }
    println!("{:?}", i + 4);

    i = 0;
    while true {
        let chars: HashSet<char> = input[i..i + 14].chars().collect();
        if chars.len() == 14 {
            println!("{:?}", &input[i..i + 14]);
            break;
        }
        i += 1;
    }
    println!("{:?}", i + 14);

    Ok(())
}
