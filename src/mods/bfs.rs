/*
// Characters in matrix:
. = walkable path
X = stone (mineable)
A = cobblestone (placeable)
B = bedrock roadblock (avoid this)
C = iron (mineable material)
D = osmium (mineable material)
E = spawn point
F = acid (deals damage)
? = unknown
*/
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use crate::mods::matrix::read_starting_position;

pub fn get_matrix(server_file_contents: &str, id: char, round: usize) {
    // Specify path to obtain starting position
    let file_path = format!("game/s{}_{}.txt", id, round);
    
    match read_starting_position(&file_path) {
        Ok((x, y)) => {
            println!("Starting position: ({}, {})", x, y);
            
        }
        Err(e) => eprintln!("Error reading starting position: {}", e),
    }
    
    let matrix: Vec<Vec<char>> = server_file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect()
        })
        .collect();
    
    // Print matrix
    println!("Matrix:");
    for row in &matrix {
        println!("{:?}", row);
    }
    
    // Characters of interest with priority
    let chars_of_interest = vec![('D', 1), ('C', 2), ('.', 3), ('X', 4), ('B', 8)];
    
    // BFS from node ID
    let result = bfs(&matrix, id, &chars_of_interest);
    
    // Extract chars of interest & priorities from result
    let updated_chars_of_interest: Vec<(char, usize)> = result.chars_of_interest.clone();
    
    println!("Updated chars of interest: {:?}", chars_of_interest);
}

pub struct BfsResult {
    visited_nodes: HashMap<char, usize>,
    chars_of_interest: Vec<(char, usize)>,
}

impl BfsResult {
    // Print visited nodes
    fn print_visited_nodes(&self) {
        println!("Visited nodes:");
        for (node, priority) in &self.visited_nodes {
            println!("Node: {}, Priority: {}", node, priority);
        }
    }
}

pub fn bfs(matrix: &Vec<Vec<char>>, start: char, chars_of_interest: &Vec<(char, usize)>) -> BfsResult {
    let mut visited_nodes = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut chars_of_interest_found = Vec::new();

    visited_nodes.insert(start, 0);
    queue.push(Reverse((start, 0)));

    while let Some(Reverse((node, priority))) = queue.pop() {
        if let Some(node_index) = matrix.iter().position(|row| row[0] == node) {
            for &neighbor in &matrix[node_index][1..] {
                if !visited_nodes.contains_key(&neighbor) {
                    let new_priority = priority + 1;
                    visited_nodes.insert(neighbor, new_priority);
                    queue.push(Reverse((neighbor, new_priority)));

                    // Check if the character of interest is found
                    if let Some(&(_, interest_priority)) = chars_of_interest.iter().find(|&&(c, _)| c == neighbor) {
                        println!("Found character of interest: {} with priority: {}", neighbor, interest_priority);
                        chars_of_interest_found.push((neighbor, interest_priority))
                        
                        // Additional actions or return early if needed
                    }
                }
            }
        }
    }

    BfsResult {
        visited_nodes,
        chars_of_interest: chars_of_interest_found,
    }
}