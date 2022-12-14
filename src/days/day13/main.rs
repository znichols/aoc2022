use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct PacketList {
    packet: String,
    pos: usize,
}

#[derive(Debug, Clone)]
enum PacketElement {
    Num(i32),
    Packet(PacketList),
    None,
}

impl PacketList {
    fn from_str(s: &str) -> Self {
        PacketList {
            packet: s.to_string(),
            pos: 0,
        }
    }

    fn next_elem(&mut self) -> PacketElement {
        while self.packet.chars().nth(self.pos) == Some(',') {
            self.pos += 1;
        }
        match self.packet.chars().nth(self.pos) {
            Some('[') => {
                let mut parens = 1;
                for i in self.pos + 1..self.packet.len() {
                    match self.packet.chars().nth(i) {
                        Some('[') => parens += 1,
                        Some(']') => parens -= 1,
                        _ => {}
                    };
                    if parens == 0 {
                        let r =
                            PacketElement::Packet(Self::from_str(&self.packet[self.pos + 1..i]));
                        self.pos = i + 1;
                        return r;
                    }
                }
                PacketElement::None
            }
            Some(']') => PacketElement::None,
            Some(_) => {
                let mut i = self.pos;
                let mut ch = self.packet.chars().nth(i);
                while ch.is_some() && ch != Some(',') {
                    i += 1;
                    ch = self.packet.chars().nth(i);
                }
                let r = PacketElement::Num(self.packet[self.pos..i].parse::<i32>().unwrap());
                self.pos = i + 1;
                r
            }
            _ => PacketElement::None,
        }
    }

    fn cmp(&mut self, right: &mut Self) -> Option<bool> {
        let left_next = self.next_elem();
        let right_next = right.next_elem();
        match left_next {
            PacketElement::None => match right_next {
                PacketElement::None => None,
                _ => Some(true),
            },
            PacketElement::Num(left_n) => match right_next {
                PacketElement::None => Some(false),
                PacketElement::Num(right_n) => match left_n.cmp(&right_n) {
                    Ordering::Less => Some(true),
                    Ordering::Greater => Some(false),
                    _ => self.cmp(right),
                },
                PacketElement::Packet(mut right_p) => {
                    let mut new_packet = PacketList {
                        packet: left_n.to_string(),
                        pos: 0,
                    };
                    match new_packet.cmp(&mut right_p) {
                        Some(r) => Some(r),
                        None => self.cmp(right),
                    }
                }
            },
            PacketElement::Packet(mut left_p) => match right_next {
                PacketElement::None => Some(false),
                PacketElement::Num(right_n) => {
                    let mut new_packet = PacketList {
                        packet: right_n.to_string(),
                        pos: 0,
                    };
                    match left_p.cmp(&mut new_packet) {
                        Some(r) => Some(r),
                        _ => self.cmp(right),
                    }
                }
                PacketElement::Packet(mut right_p) => match left_p.cmp(&mut right_p) {
                    Some(b) => Some(b),
                    _ => self.cmp(right),
                },
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let packet_couples: Vec<_> = input
        .split("\n\n")
        .map(|s| {
            let grp: Vec<_> = s.split('\n').collect();
            (PacketList::from_str(grp[0]), PacketList::from_str(grp[1]))
        })
        .map(|(mut p1, mut p2)| p1.cmp(&mut p2))
        .collect();

    println!(
        "{:?}",
        packet_couples
            .iter()
            .enumerate()
            .map(|(i, b)| if b.unwrap() { i + 1 } else { 0 })
            .sum::<usize>()
    );

    let mut packet_strings: Vec<_> = input.split("\n\n").flat_map(|s| s.split('\n')).collect();
    packet_strings.push("[[2]]");
    packet_strings.push("[[6]]");
    packet_strings.sort_by(|&a, &b| {
        match PacketList::from_str(a).cmp(&mut PacketList::from_str(b)) {
            Some(false) => Ordering::Greater,
            Some(true) => Ordering::Less,
            _ => Ordering::Equal,
        }
    });
    println!(
        "{}",
        (packet_strings.iter().position(|s| s.eq(&"[[2]]")).unwrap() + 1)
            * (packet_strings.iter().position(|s| s.eq(&"[[6]]")).unwrap() + 1)
    );

    Ok(())
}
