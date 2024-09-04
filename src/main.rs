use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::error::Error;
use rotate_left::rotate_left_in_place; // Import from lib.rs

fn main() -> Result<(), Box<dyn Error>> {
    // Read from stdin and write output to a file
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Provide a default output path if OUTPUT_PATH is not set
    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| {
        println!("OUTPUT_PATH not set. Defaulting to 'output.txt'.");
        "output.txt".to_string()
    });

    let mut fptr = File::create(output_path)?;

    // Read the first input line and split into integers directly
    let first_multiple_input: Vec<usize> = stdin_iterator
        .next()
        .ok_or("Missing input")??
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let n = first_multiple_input[0]; // Number of elements
    let d = first_multiple_input[1]; // Number of rotations

    // Read the second input line, which contains the array elements
    let mut arr: Vec<i32> = stdin_iterator
        .next()
        .ok_or("Missing input")??
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    // Ensure we have the correct number of elements
    if arr.len() != n {
        return Err("Array length does not match the expected input size.".into());
    }

    // Rotate the array in-place
    rotate_left_in_place(d % n, &mut arr);

    // Write the result to the output file efficiently
    for (i, val) in arr.iter().enumerate() {
        if i > 0 {
            write!(&mut fptr, " ")?;
        }
        write!(&mut fptr, "{}", val)?;
    }
    writeln!(&mut fptr)?;

    Ok(())
}
