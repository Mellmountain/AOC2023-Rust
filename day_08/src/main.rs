use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}
fn main() {
    let mut input = include_str!("./input.txt").lines();
    let instructions: Vec<char> = input.next().unwrap().chars().collect();
    let mut start_nodes: Vec<Node> = Vec::new();
    let mut end_nodes: Vec<Node> = Vec::new();
    let mut network: HashMap<&str, Node> = HashMap::new();
    
    for line in input {
        if line.len() == 0 {continue;}
        let (node_id, paths) = &line.split_once(" = ").unwrap();
        
        let (left, right) = paths.split_once(", ").unwrap();
        network.insert(node_id, Node {id: node_id.to_string(), left: left[1..].to_string(), right: right[0..3].to_string()});
        if node_id.ends_with("A") {
            start_nodes.push(Node {id: node_id.to_string(), left: left[1..].to_string(), right: right[0..3].to_string()});
        }
    }

    let mut current_node = network.get("AAA").unwrap();
    let mut part1 = 0;
    let mut ip = 0;
    while current_node.id != "ZZZ" {
        if instructions[ip] == 'R' {
            current_node = network.get(current_node.right.as_str()).unwrap();
        } else {
            current_node = network.get(current_node.left.as_str()).unwrap();
        }
        part1 += 1;
        ip = (ip + 1) % instructions.len();
    } 
    
    let mut LCM: Vec<usize> = Vec::new(); 
    for node in start_nodes {
        let mut steps = 0;
        let mut ip = 0;
        let mut current = node.clone();
        
        while !current.id.ends_with("Z") {
            if instructions[ip] == 'R' {
                current = network.get(current.right.as_str()).unwrap().clone();
            } else {
                current = network.get(current.left.as_str()).unwrap().clone();
            }
            
            steps += 1;
            ip = (ip + 1) % instructions.len();
        }
        LCM.push(steps);
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", lcm_of(&LCM));

}

fn gcd(x: usize, y:usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: usize, y:usize) -> usize {
    x * y / gcd(x, y)
}

fn lcm_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();

    let mut ans = lcm(first, second);
    while let Some(&next) = iter.next() {
        ans = lcm(ans, next)
    }
    ans
}

