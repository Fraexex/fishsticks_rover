/*
Characters in matrix:
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
use std::io::{self, BufRead, prelude::*};
use std::fs::File;
use crate::mods::bfs::get_matrix;

pub fn read_starting_position(file_path: &str) -> io::Result<(usize, usize)> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffer reader
    let reader = io::BufReader::new(file);

    // Read the first line (assuming it contains the starting position)
    if let Some(Ok(line)) = reader.lines().next() {
        // Parse the line to extract the x and y coordinates
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
                return Ok((x, y));
            }
        }
    }

    // If there's an issue with reading or parsing, return an error
    Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid starting position format"))
}

pub fn matrix_txt(id: char, round: usize) {
    // Determine name of server created file
    let server_file = format!("game/s{}_{}.txt", id, round);
    
    // Open server file for reading
    match File::open(server_file.clone()) {
        Ok(mut file) => {
            // Read contents
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            
            // Display content (delete when finished)
            println!("Content of {}: \n{}", server_file, content);
            
            // Save content into variable
            let server_file_contents = content;
            
            // Send to GetMatrix function
            get_matrix(&server_file_contents, id, round);
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}