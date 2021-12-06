//! We read each line of the file, corresponding to a value
//! Find how how many values are higher than the previous one by iterating over pairs of two and comparing

use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;

fn main() {
    let input_file_path = "input";

    let count = BufReader::new(                  // Open file and read input 
        File::open(input_file_path).expect("Couldn't open file.")
            )
            .lines()
            .map(|input| input.unwrap().parse::<i32>().expect("Input from file not a vaild number"))
            .collect::<Vec<i32>>()           // Collect into vector
            .windows(2)              // Iterate over slices of two
            .fold(0, |acc, window| // Accumulate how many x[i] > x [j] given i > j 
                acc + {if window[1] - window[0] > 0 {1} else {0}}
            );

    println!("{}", count);
}
