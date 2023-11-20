mod mods;

use std::{
    fs::File,
    io::prelude::*,
    io::{self, BufWriter},
    time::Instant,
    time::Duration,
    thread
};
use mods::matrix::matrix_txt;

fn main() {
    // Initialize rover ID
    println!("ID:");
    let mut str_id = String::new();
    io::stdin().read_line(&mut str_id).expect("Failed to read line (type a number)");
    let str_id = str_id.trim();
    
    // Convert string to char
    let Some(id) = str_id.chars().next() else { todo!() };
    println!("Rover ID is: {}", id);
    
    // TODO: Tie in with BFS
    let mut movement = "U";
    let mut action = "M U";
    let mut buying = "B ";
    
    // Initialize round
    let mut round = 0;
    
    // Initialize directory & create if not present
    let base_directory = "game";
    std::fs::create_dir_all(base_directory).unwrap_or_else(|err| {
        eprintln!("Error creating directory: {}", err);
        std::process::exit(1);
    });
    
    loop {
        // Read the server file & store matrix elements
        matrix_txt(id, round);
        
        // Generate file name to be read by server
        let file_name = format!("{}/c{}_{}.txt", base_directory, id, round);
        
        // Create file to write to
        let file = File::create(&file_name).unwrap_or_else(|err| {
            eprintln!("Error creating file: {}", err);
            std::process::exit(1);
        });
        
        // TODO: Change parameter to rover's movement, action, and buying
        let mut buf_writer = BufWriter::new(file);
        if round < 10 {
            writeln!(buf_writer, "U M U"/*"This is file: {}.", round*/).unwrap_or_else(|err| {
                eprintln!("Error writing to file: {}", err);
                std::process::exit(1);
            });
        }
        else {
            writeln!(buf_writer, "P D"/*"This is file: {}.", round*/).unwrap_or_else(|err| {
                eprintln!("Error writing to file: {}", err);
                std::process::exit(1);
            });
        }
        // Trying to sleep for 5 ms after calling thread::sleep
        // Goal: Don't overwhelm server
        let start = Instant::now();
        thread::sleep(Duration::from_millis(5));
        println!("Slept {} ms", start.elapsed().as_millis());
        
        // Round should increment at end
        round += 1;
        println!("{}", round);
    }
}
/*
fn movement() {
    struct Directions {
        U: Vec<i8>,
        D: Vec<i8>,
        L: Vec<i8>,
        R: Vec<i8>,
    let mut action = 1
    let mut buying = 1
}
*/