//! First we read the numbers that will be drawn
//! After that we read each matrix, corresponding to a 'Bingo card'
//! We simulate the drawing of lucky numbers, and for each draw we mark out the corresponding value in each bingo card (if it exists)
//! Once all the values in a row/column are marked, we have a strike, the bingo card is a winner
//! Calculate the result of unmarked values in the winning card

use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;
#[derive(Default, Debug)]
struct BingoCard(Vec<Vec<(i32, bool)>>);

fn main() {
    let input_file_path = "../day_4/input";
    let mut bingo = Vec::<BingoCard>::new();

    let mut lines = BufReader::new(                     // Open file and read input 
        File::open(input_file_path).expect("Couldn't open file.")
            )
            .lines();

    let drawn_numbers = lines.next()      // Numbers that we will draw from
                                     .unwrap()
                                     .unwrap()
                                     .split(',')
                                     .map(|x| x.parse::<i32>().expect("Bad input, not a number"))
                                     .collect::<Vec<_>>();

    lines.for_each(|line|{                              // All the bingo cards
        let line = line.unwrap();
        if line.is_empty() {                                               // Start of new BingoCard
            bingo.push(BingoCard::default())
        }
        else {                                                             // Continue building same BingoCard
            bingo.last_mut().unwrap().0.push(line.split_whitespace()
                                                 .map(|nr| (nr.parse::<i32>().expect("Bad input, not a number"), false))
                                                 .collect()
                                             )
        }
    });
    
    let mut finished_value = 0;                                               //  Final answer

    'outer: for drawn_number in &drawn_numbers {                             // Draw each of the numbers until we a bingo card wins
         for bingocard in bingo.iter_mut() {                        // After drawing check each card
             
            let mut changed_value_at = None;                 // The drawn number is chosen by this bingo card
            let mut striked = false;

             // Cycle through the entire matrix, until we find a hit
            'bingo: for (row_index, row) in bingocard.0.iter_mut().enumerate() {        // For each row

                for (column_index, (value, is_marked)) in row.iter_mut().enumerate() {    // For each value in each row
                    if value == drawn_number {                                   // If we find a hit, set the index where we found it and break out of this bingo
                        *is_marked = true;
                        changed_value_at = Some((row_index, column_index));
                        break 'bingo;
                    }
                }
            }

                // If we did have a hit in this bingo card, check the row and column of hit to see if we have for a strike (all values in row or column marked)
                if let Some(changed_value_at) = changed_value_at { 

                for (_, is_drawn) in  &(bingocard.0[changed_value_at.0]) {  // Check each row for a strike (all values in row marked)
                    striked = true;
                    if !is_drawn {            // Found unmarked value in row, break out
                        striked = false;
                        break;
                    }
                }

                if striked {                 // Finished, calculate answer
                    finished_value = calculate_sum(bingocard, *drawn_number);
                    break 'outer;
                }

                for row_index  in 0..bingocard.0.len() {                    // Check each column for a strike (all values in column marked)
                    striked = true;

                    if !bingocard.0[row_index][changed_value_at.1].1 {
                        striked = false;
                        break;
                    }
                }

                if striked {                // Finished, calculate answer
                    finished_value = calculate_sum(bingocard, *drawn_number);
                    break 'outer;
                }
            }
        }
    }
  
    // Print solution
   println!("{}",finished_value);
}


// Calculate the sum according to the given criteria
// (Sum of all unmarked values in card) * last drawn value
fn calculate_sum(card: &BingoCard, value: i32) -> i32{
    card.0.iter().flatten().fold(0, |acc, value| if value.1 == false {acc + value.0} else {acc}) * value
}