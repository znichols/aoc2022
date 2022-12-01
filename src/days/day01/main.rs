use std::env;
use std::error::Error;
use std::fs;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let elf_bag = input.split("\n\n")
        .map(|l| {
            l.split('\n').flat_map(|n| n.parse::<i32>())
                .collect::<Vec<_>>()
        });

    let mut elf_sum: Vec<i32> = elf_bag.map(|b| {
        b.iter().sum()
    }).collect();

    elf_sum.sort_by(|a, b| b.cmp(a));

    println!("{:?}", elf_sum[0]);
    println!("{:?}", &elf_sum[..3].iter().sum::<i32>());

    Ok(())
}