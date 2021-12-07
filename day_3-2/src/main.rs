//! We read each line of the file, corresponding to a value
//! A vector is used to keep count of how many times the 0 binary digit is encountered in the specified position
//! If the count for a position in the vector is over half the total numbers read, the value is 0
//! Otherwise its 1
//! Convert the bit vector into decimal, do the same for a vector with inverted bits, and get result
//! 
#![feature(drain_filter)]
use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;

fn main() {
    let input_file_path = "../day_3/input";
    let values = BufReader::new(                     // Open file and read input 
        File::open(input_file_path).expect("Couldn't open file.")
            )
            .lines()
            .map(|input| input.expect("Input from file not vaild"))
            .collect::<Vec<String>>();

            
    let mut oxygen = values.clone();
    let mut final_binary_nr = calculate_most_frequent_bit(&values);
    
    // For each of the binary digits
    for i in 0..values.len() {    
        // Only keep the numbers with the most frequent bit at each positoin
        oxygen.drain_filter(|value| 
            value.chars().nth(i) != char::from_digit(final_binary_nr[i],10));

        if oxygen.len()<=1 { break }; // We are done when there's only one number left
        // Recalculate the most frequent bits
        final_binary_nr = calculate_most_frequent_bit(&oxygen);
    }
        
    // Same as previous codeblock, but this time we neet the LEAST FREQUENT bit instead
    let mut co2 = values.clone();
    final_binary_nr = calculate_most_frequent_bit(&values);
    for i in 0..values.len() {    
        co2.drain_filter(|value| 
            value.chars().nth(i) != char::from_digit((final_binary_nr[i]+1)%2,10));
            
        if co2.len()<=1 { break };
        final_binary_nr = calculate_most_frequent_bit(&co2);
    }
    
   // Convert original and inverted vector, multiply them to get the result
   println!("{}", u32::from_str_radix(&oxygen[0], 2).unwrap() * u32::from_str_radix(&co2[0], 2).unwrap());
}


// Calculates the frequency of each bit in a given position
fn calculate_most_frequent_bit(values: &Vec<String>) -> Vec<u32>{
    
    let binary_digits_count = 12;                                  // Length of our binary nr
    let mut count_of_zeros = vec![0;binary_digits_count];

    values  .iter()
            .for_each(|binary_nr|
                binary_nr.chars().enumerate().for_each(|(position,binary_digit)| 
                    if binary_digit == '0'{count_of_zeros[position] += 1}) // Store count of '0' digits
            );
    
    
    let mut final_binary_nr = Vec::with_capacity(binary_digits_count);
    for value in count_of_zeros.into_iter(){
        final_binary_nr.push(if value > values.len()/2 {0} else {1});   // If count of 0 digits is more than half,
                                                                        // the digit at that position should be 0
    }

    return final_binary_nr;
}