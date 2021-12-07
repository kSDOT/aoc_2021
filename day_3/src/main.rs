//! We read each line of the file, corresponding to a value
//! A vector is used to keep count of how many times the 0 binary digit is encountered in the specified position
//! If the count for a position in the vector is over half the total numbers read, the value is 0
//! Otherwise its 1
//! Convert the bit vector into decimal, do the same for a vector with inverted bits, and get result

use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;

fn main() {
    let input_file_path = "../day_3/input";
    let binary_digits_count = 12;
    let mut count_of_zeros = vec![0;binary_digits_count];
    let mut input_length = 0;
    let _ = BufReader::new(                     // Open file and read input 
        File::open(input_file_path).expect("Couldn't open file.")
            )
            .lines()
            .map(|input| input.expect("Input from file not vaild"))
            .for_each(|binary_nr|{
                binary_nr.chars().enumerate().for_each(|(position,binary_digit)| 
                    if binary_digit == '0'{count_of_zeros[position] += 1}); // Store count of '0' digits
                input_length +=1        // How many input values we have 
            }
            );
    
    let mut final_binary_nr = Vec::with_capacity(binary_digits_count);
    for value in count_of_zeros.into_iter(){
        final_binary_nr.push(if value > input_length/2 {0} else {1});   // If count of 0 digits is more than half,
                                                                        // the digit at that position should be 0
    }
    
   // Convert original and inverted vector, multiply them to get the result
   println!("{}", to_u32(&final_binary_nr) * to_u32_inverted(&final_binary_nr));
}


fn to_u32(slice: &[u8]) -> u32 {            // convert vector of bits into decimal number
    slice.iter().fold(0, |acc, &b| acc*2 + b as u32)
}

fn to_u32_inverted(slice: &[u8]) -> u32 {   // invert each bit in  vector of bits and convert into decimal number
    slice.iter().fold(0, |acc, &b| acc*2 + ((b+1)%2) as u32)
}