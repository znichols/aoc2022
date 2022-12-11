use std::env;
use std::error::Error;
use std::fs;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Item {
    val: u64,
    inspection_count: usize,
}

type ItemRef = Rc<RefCell<Item>>;

#[derive(Debug)]
struct Monkey {
    t_mod: u64,
    op: String,
    operand: String,
    items: Vec<ItemRef>,
    inspection_count: usize,
    to_monkeys: (usize, usize),
    item_mod: u64,
}

impl Monkey {
    fn new(block: &str) -> Rc<RefCell<Self>> {
        let lines: Vec<_> = block.split('\n').collect();
        let items: Vec<_> = lines[1]
            .split(':')
            .last()
            .unwrap()
            .split(',')
            .map(|s| {
                Rc::new(RefCell::new(Item {
                    val: s.trim().parse().unwrap(),
                    inspection_count: 0,
                }))
            })
            .collect();

        Rc::new(RefCell::new(Monkey {
            t_mod: lines[3]
                .split_whitespace()
                .nth(3)
                .map(|s| s.parse::<u64>().unwrap())
                .unwrap(),
            op: lines[2].split_whitespace().nth(4).unwrap().to_string(),
            operand: lines[2].split_whitespace().nth(5).unwrap().to_string(),
            items,
            inspection_count: 0,
            to_monkeys: (
                lines[4]
                    .split_whitespace()
                    .last()
                    .map(|s| s.parse().unwrap())
                    .unwrap(),
                lines[5]
                    .split_whitespace()
                    .last()
                    .map(|s| s.parse().unwrap())
                    .unwrap(),
            ),
            item_mod: 0,
        }))
    }

    fn accept_item(&mut self, item: ItemRef) {
        self.items.push(item);
    }

    fn inspect_items(&mut self) -> Vec<(usize, ItemRef)> {
        self.inspection_count += self.items.len();

        let r = self
            .items
            .iter()
            .map(|item| {
                let mut item_mut = item.borrow_mut();
                item_mut.inspection_count += 1;
                let operand = self.operand.parse::<u64>().unwrap_or(item_mut.val);
                match self.op.as_str() {
                    "+" => item_mut.val += operand,
                    _ => item_mut.val *= operand,
                };
                if self.item_mod == 0 {
                    item_mut.val /= 3;
                } else {
                    item_mut.val %= self.item_mod;
                }
                match item_mut.val % self.t_mod {
                    0 => (self.to_monkeys.0, Rc::clone(item)),
                    _ => (self.to_monkeys.1, Rc::clone(item)),
                }
            })
            .collect();
        self.items.clear();
        r
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let monkeys: Vec<_> = input.split("\n\n").map(Monkey::new).collect();
    for _ in 0..20 {
        for monkey in &monkeys {
            let new_items = monkey.borrow_mut().inspect_items();
            for (monkey_ind, item) in new_items {
                let mut mref = monkeys[monkey_ind].borrow_mut();
                mref.accept_item(item);
            }
        }
    }
    let mut inspection_counts: Vec<_> = monkeys
        .iter()
        .map(|m| m.borrow().inspection_count)
        .collect();
    inspection_counts.sort();
    println!(
        "{:?}",
        inspection_counts[inspection_counts.len() - 2..]
            .iter()
            .product::<usize>()
    );

    let monkey_prod = monkeys.iter().map(|m| m.borrow().t_mod).product::<u64>();

    let monkeys: Vec<_> = input.split("\n\n").map(Monkey::new).collect();
    monkeys
        .iter()
        .for_each(|m| m.borrow_mut().item_mod = monkey_prod);
    for _ in 0..10000 {
        for monkey in &monkeys {
            let new_items = monkey.borrow_mut().inspect_items();
            for (monkey_ind, item) in new_items {
                let mut mref = monkeys[monkey_ind].borrow_mut();
                mref.accept_item(item);
            }
        }
    }
    inspection_counts = monkeys
        .iter()
        .map(|m| m.borrow().inspection_count)
        .collect();
    inspection_counts.sort();
    println!(
        "{:?}",
        inspection_counts[inspection_counts.len() - 2..]
            .iter()
            .product::<usize>()
    );

    Ok(())
}
