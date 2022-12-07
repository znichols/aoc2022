use std::env;
use std::error::Error;
use std::fs;

use trees::{RcNode, Tree};

#[derive(Debug, Clone)]
struct FileNode {
    name: String,
    size: usize,
    isdir: bool,
}

fn calc_dir_size(file_tree: RcNode<FileNode>) -> RcNode<FileNode> {
    let mut sum = 0;
    for child in file_tree.iter_rc() {
        if child.data().isdir {
            sum += calc_dir_size(child).data().size;
        } else {
            sum += child.data().size;
        }
    }
    file_tree.data_mut().size = sum;
    file_tree
}

fn get_dir_sizes(file_tree: &RcNode<FileNode>) -> Vec<FileNode> {
    let mut v = vec![file_tree.data().clone()];
    for child in file_tree.iter_rc() {
        if child.data().isdir {
            v.append(&mut get_dir_sizes(&child));
        }
    }
    v
}

fn add_files(ls_output: &[&str], dir_node: &RcNode<FileNode>) {
    for &line in ls_output.iter() {
        let entries: Vec<_> = line.split_whitespace().collect();
        if entries[0] == "dir" {
            continue;
        }
        dir_node.push_back(Tree::new(FileNode {
            name: entries[1].to_string(),
            size: entries[0].parse().unwrap(),
            isdir: false,
        }));
    }
}

fn read_term(term: &[&str], file_tree: RcNode<FileNode>) -> RcNode<FileNode> {
    let mut i = 1;
    let mut dir_node_stack: Vec<RcNode<FileNode>> = vec![file_tree];

    while i < term.len() {
        let cmd: Vec<_> = term[i].split_whitespace().collect();
        match cmd[1] {
            "ls" => {
                let mut j = i + 1;
                while j < term.len() && !term[j].starts_with('$') {
                    j += 1;
                }
                add_files(&term[i + 1..j], dir_node_stack.last().unwrap());
                i = j;
            }
            _ => {
                match cmd[2] {
                    ".." => {
                        dir_node_stack.pop();
                    }
                    _ => {
                        let curr_node = dir_node_stack.last().unwrap();
                        curr_node.push_back(Tree::new(FileNode {
                            name: cmd[2].to_string(),
                            size: 0,
                            isdir: true,
                        }));
                        let new_node = curr_node.back().unwrap();
                        dir_node_stack.push(new_node);
                    }
                };
                i += 1;
            }
        }
    }
    dir_node_stack.truncate(1);
    dir_node_stack.pop().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let term: Vec<_> = input.split('\n').collect();

    let file_tree = Tree::new(FileNode {
        name: "/".to_string(),
        size: 0,
        isdir: true,
    });

    let mut root_node = RcNode::from(file_tree);
    root_node = read_term(&term, root_node);
    root_node = calc_dir_size(root_node);
    let dir_sizes = get_dir_sizes(&root_node);
    let used_space = dir_sizes[0].size;
    println!(
        "{}",
        dir_sizes
            .iter()
            .filter(|d| d.size <= 100000)
            .map(|d| d.size)
            .sum::<usize>()
    );
    let needed_space = used_space - 40000000;
    println!(
        "{}",
        dir_sizes
            .iter()
            .filter(|d| d.size >= needed_space)
            .map(|d| d.size)
            .min()
            .unwrap()
    );

    Ok(())
}
