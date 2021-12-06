//! We read each line of the file, corresponding to a value
//! For each line we add/substract to current coordinates

use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;

fn main() {
    let input_file_path = "../day_2/input";

    let count = BufReader::new(                     // Open file and read input 
        File::open(input_file_path).expect("Couldn't open file.")
            )
            .lines()
            .map(|input| {                  // Tuple of (Command, Parameter)
                let values = input.unwrap();
                let mut values = values.split(' ');
                (values.next().expect("Input from file not vaild").to_owned(),
                 values.last().unwrap().parse::<i32>().expect("Input from file not a vaild number"))
                }
            )
            .fold((0, 0),|acc, x|{// Update current coordinates
                    let mut temp = acc;
                    match (x.0).as_str() {
                        "forward"  => temp.0 += x.1,
                        "down"     => temp.1 -= x.1,
                        "up"       => temp.1 += x.1,
                        _ => (),
                    };
                    temp
                }
            );

    println!("{}", (count.0*count.1).abs());
}
