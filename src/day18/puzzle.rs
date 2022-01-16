use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

const FILESTRING: &str = &"src/day18/lines.txt";

// I blocked myself here because Rust doesn't allow cyclical graph. I need parents that hold their children, and vice-versa...

pub fn solve_puzzle1() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {
        let result: TreeNode = lines.map(|line| line.unwrap())
            .map(|line| TreeNode::from_str(Option::None, &line))
            .reduce(|a, b| {
                print!("adding new tree\n");
                add_two_trees(a, b)
            })
            .unwrap();
        println!("day18 puzzle 1: {}", result);
    }
}

pub fn solve_puzzle2() {
    let path: &Path = Path::new(FILESTRING);
    if let Ok(mut lines) = read_lines(path) {

        println!("day18 puzzle 2: {}", 0);
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug,Clone,Hash,Eq,PartialEq, Ord, PartialOrd)]
struct TreeNode {
    value: Option<u8>,
    parent: Option<Box<TreeNode>>,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

impl TreeNode {
    fn depth(&self) -> usize {
        if self.parent.is_none() {
            return 2;
        }
        return (*(self.parent.as_ref().unwrap())).depth() + 1;
    }

    fn can_be_split(&self) -> bool {
        return self.is_leaf() && self.value.unwrap() > 9
    }

    fn can_explode(&self) -> bool {
        if !self.is_leaf() {
        }
        return self.depth() > 4 && !self.is_leaf();
    }

    fn is_leaf(&self) -> bool {
        return self.value.is_some();
    }

    fn new_intermediate(parent: TreeNode, left: TreeNode, right: TreeNode) -> Self {
        Self {
            parent: Option::Some(Box::from(parent)),
            left: Option::Some(Box::from(left)),
            right: Option::Some(Box::from(right)),
            value: Option::None
        }
    }

    fn new_leaf(parent: TreeNode, value: u8) -> Self {
        Self {
            parent: Option::Some(Box::from(parent)),
            left: Option::None,
            right: Option::None,
            value: Option::Some(value)
        }
    }
    fn new_root(left: TreeNode, right: TreeNode) -> Self {
        Self {
            parent: Option::None,
            left: Option::Some(Box::from(left)),
            right: Option::Some(Box::from(right)),
            value: Option::None
        }
    }
    fn new_leaf_root(value: u8) -> Self {
        Self {
            parent: Option::None,
            left: Option::None,
            right: Option::None,
            value: Option::Some(value)
        }
    }

    fn find_next_exploding_node(&self) -> Option<&TreeNode> {
        // The leftmost pair explodes. Recursively find that pair.
        if self.left.is_some() {
            let exploding_left: Option<&TreeNode> = self.left.as_ref().unwrap().find_next_exploding_node();
            if exploding_left.is_some() {
                return exploding_left;
            }
        }
        print!("searching if {} can explode\n", self);
        if self.can_explode() {
            return Option::Some(self);
        }
        if self.right.is_some() {
            let exploding_right: Option<&TreeNode> = self.right.as_ref().unwrap().find_next_exploding_node();
            if exploding_right.is_some() {
                return exploding_right;
            }
        }
        return Option::None;
    }

    fn find_next_splitting_node(&self) -> Option<&TreeNode> {
        if self.left.is_some() {
            let splitting_left: Option<&TreeNode> = self.left.as_ref().unwrap().find_next_splitting_node();
            if splitting_left.is_some() {
                return splitting_left;
            }
        }
        if self.can_be_split() {
            return Option::Some(self);
        }
        if self.right.is_some() {
            let splitting_right: Option<&TreeNode> = self.right.as_ref().unwrap().find_next_splitting_node();
            if splitting_right.is_some() {
                return splitting_right;
            }
        }
        return Option::None;
    }

    // Completely reduce the given treeNode, applying operations one at a time,
    // respecting left_most priority
    fn reduce(&mut self) {
        let mut should_loop_again: bool = true;
        while should_loop_again {
            // explosions take priority over splits
            let exploding_node: Option<&TreeNode> = self.find_next_exploding_node();
            if exploding_node.is_some() {
                print!("Found exploding node");
                exploding_node.unwrap().to_owned().do_explode();
                continue;
            }
            let splitting_node: Option<&TreeNode> = self.find_next_splitting_node();
            if splitting_node.is_some() {
                print!("Found splitting node");
                splitting_node.unwrap().to_owned().do_split();
                continue;
            }
            should_loop_again = false;
        }
    }

    // To explode a pair, the pair's left value is added to the first regular number to the left
    // of the exploding pair (if any), and the pair's right value is added to the first regular
    // number to the right of the exploding pair (if any).
    // Exploding pairs will always consist of two regular numbers.
    // Then, the entire exploding pair is replaced with the regular number 0.
    fn do_explode(&mut self) {
        // When exploding to the left, we look for the rightmost TreeNode that is to the left
        // of this one.
        // We will prioritize right, then left, then parent. This is done by relying on Tuple ordering,
        // with the priority coming in first.
        let mut path_to_root: HashSet<TreeNode> = HashSet::new();

        let mut current_node: TreeNode = self.to_owned();
        while current_node.parent.is_some() {
            path_to_root.insert(current_node.clone());
            current_node = *current_node.parent.unwrap();
        }
        {
            // explode left
            let mut priority_queue: BinaryHeap<(u8, TreeNode)> = BinaryHeap::new();
            let mut visited_nodes: HashSet<TreeNode> = HashSet::new();
            priority_queue.push((3, *self.clone().parent.unwrap()));
            visited_nodes.insert(self.clone());
            while let Some((_priority, mut node)) = priority_queue.pop() {
                if visited_nodes.contains(&node) {
                    continue;
                }
                let clone: TreeNode = node.clone();
                visited_nodes.insert(clone);
                if node.is_leaf() {
                    let new_value: Option<u8> = Option::Some(node.value.unwrap() + self.left.clone().unwrap().value.unwrap());
                    node.value = new_value;
                    break;
                }
                if node.right.is_some() {
                    let right: TreeNode = (**node.right.as_ref().unwrap()).clone();
                    if !path_to_root.contains(&node.clone()) && !visited_nodes.contains(&right) {
                        priority_queue.push((1, right));
                    }
                }

                if node.left.is_some() {
                    let left: TreeNode = (*node.left.unwrap()).clone();
                    if !visited_nodes.contains(&left) {
                        priority_queue.push((2, left));
                    }
                }

                if node.parent.is_some() {
                    let parent: TreeNode = (*node.parent.unwrap()).clone();
                    if !visited_nodes.contains(&parent) {
                        priority_queue.push((3, parent));
                    }
                }
            }
        }
        {
            // explode right
            let mut priority_queue: BinaryHeap<(u8, TreeNode)> = BinaryHeap::new();
            let mut visited_nodes: HashSet<TreeNode> = HashSet::new();
            priority_queue.push((3, *self.clone().parent.unwrap()));
            visited_nodes.insert(self.clone());
            while let Some((_priority, mut node)) = priority_queue.pop() {
                if visited_nodes.contains(&node) {
                    continue;
                }
                let clone: TreeNode = node.clone();
                visited_nodes.insert(clone);
                if node.is_leaf() {
                    let new_value: Option<u8> = Option::Some(node.value.unwrap() + self.right.clone().unwrap().value.unwrap());
                    node.value = new_value;
                    break;
                }
                if node.left.is_some() {
                    let left: TreeNode = (**node.left.as_ref().unwrap()).clone();
                    if !path_to_root.contains(&node.clone()) && !visited_nodes.contains(&left) {
                        priority_queue.push((1, left));
                    }
                }

                if node.right.is_some() {
                    let right: TreeNode = (*node.right.unwrap()).clone();
                    if !visited_nodes.contains(&right) {
                        priority_queue.push((2, right));
                    }
                }

                if node.parent.is_some() {
                    let parent: TreeNode = (*node.parent.unwrap()).clone();
                    if !visited_nodes.contains(&parent) {
                        priority_queue.push((3, parent));
                    }
                }
            }
        }
        {
            self.value = Option::Some(0);
            self.left = Option::None;
            self.right = Option::None;
        }
    }

    fn do_split(&mut self) {
        let left: TreeNode = TreeNode::new_leaf(self.to_owned(), self.value.unwrap() / 2);
        let right_value: u8 = if self.value.unwrap() % 2 == 0 { self.value.unwrap() / 2 } else { (self.value.unwrap() / 2) + 1};
        let right: TreeNode = TreeNode::new_leaf(self.to_owned(), right_value);
        self.right = Option::Some(Box::from(right));
        self.left = Option::Some(Box::from(left));
        self.value = Option::None;
    }

    fn from_str(parent: Option<TreeNode>, s: &str) -> Self {
        let mut string: String = s.replace(" ", "");
        return if string.contains(',') {
            string = string.chars().skip(1).take(string.len() - 2).collect();
            let middle: usize = find_middle(&string);
            let left: TreeNode = TreeNode::from_str(parent.clone(), &string.chars().take(middle).collect::<String>());
            let right: TreeNode = TreeNode::from_str(parent.clone(), &string.chars().skip(middle + 1).collect::<String>());
            match parent {
                None => { TreeNode::new_root(left, right) }
                Some(p) => { TreeNode::new_intermediate(p, left, right) }
            }
        } else {
            let value: u8 = u8::from_str(&string).unwrap();
            match parent {
                None => { TreeNode::new_leaf_root(value) }
                Some(p) => { TreeNode::new_leaf(p, value) }
            }
        }
    }

}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.value.is_none() {
            write!(f, "[{},{}]", self.left.as_ref().unwrap(), self.right.as_ref().unwrap())
        } else {
            write!(f, "{}", self.value.unwrap())
        }
    }
}

fn find_middle(string: &str) -> usize {
    let mut lowest_idx: isize = -1;
    let mut lowest_depth = u32::MAX;
    let mut current_depth = 0;
    for (i, c) in string.chars().enumerate() {
        let string = String::from(c);
        if string == "[" {
            current_depth = current_depth + 1;
        } else if string == "]" {
            current_depth = current_depth - 1;
        } else if string == "," {
            if current_depth < lowest_depth {
                lowest_idx = i as isize;
                lowest_depth = current_depth;
            }
        }
    }
    return lowest_idx as usize;
}

fn add_two_trees(mut left: TreeNode, mut right: TreeNode) -> TreeNode {
    let mut result: TreeNode = TreeNode::new_root(left, right);
    left.parent = Option::Some(Box::from(result.clone()));
    // Here we need to modify left and right to have them know that result is their parent...
    result.reduce();
    return result;
}
