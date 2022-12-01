use std::env;
use std::error::Error;
use std::fs;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let mut elf_sums: Vec<i32> = input.split("\n\n")
        .map(|l| {
            l.split('\n').flat_map(|n| n.parse::<i32>()).sum()
        }).collect();

    elf_sums.sort_by(|a, b| b.cmp(a));

    println!("{:?}", elf_sums[0]);
    println!("{:?}", &elf_sums[..3].iter().sum::<i32>());

    Ok(())
}