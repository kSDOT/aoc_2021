//! First we read the numbers that will be drawn
//! After that we read each matrix, corresponding to a 'Bingo card'
//! We simulate the drawing of lucky numbers, and for each draw we mark out the corresponding value in each bingo card (if it exists)
//! Once all the values in a row/column are marked, we have a strike, the bingo card is a winner
//! Continue until the all the cards, including the last one wins
//! Calculate the result of unmarked values in the card that finishes last

use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;
#[derive(Default, Debug)]
struct BingoCard(Vec<Vec<(i32, bool)>>);

fn main() {
    let input_file_path = "../day_4/input";
    let mut bingo = Vec::<(BingoCard, bool)>::new();

    let mut lines = BufReader::new(                     // Open file and read input 
        File::open(input_file_path).expect("Couldn't open file.")
            )
            .lines();

    let mut drawn_numbers = lines.next()                // Numbers that we will draw from
                             .unwrap()
                             .unwrap()
                             .split(',')
                             .map(|x| x.parse::<i32>().expect("Bad input, not a number"))
                             .collect::<Vec<_>>();

    lines.for_each(|line|{                              // All the bingo cards
        let line = line.unwrap();
        if line.is_empty() {                            // Start of new BingoCard
            bingo.push((BingoCard::default(), bool::default()))
        }
        else {                                          // Continue building same BingoCard
            bingo.last_mut().unwrap().0.0.push(line.split_whitespace()
                                                 .map(|nr| (nr.parse::<i32>().expect("Bad input, not a number"), false))
                                                 .collect()
                                             )
        }
    });
    
    let mut finished_card_index = 0;                      // Index of card that finished last
    let mut finished_value = 0;                       // Last drawn_number that causes the card to win 

    let mut nr_not_won_yet = bingo.len();           // Keep count how many cards havent won yet

    for drawn_number in &mut drawn_numbers {                                // Draw each of the numbers
        if nr_not_won_yet == 1 {                                                    // Only have 1 card left
            for (index, (_, is_winner)) in bingo.iter().enumerate() {   // Find and store its index
                if !is_winner{finished_card_index = index;}
            }        
        }
        for (bingocard, is_winner) in bingo.iter_mut() {     // After drawing check each card
            
            if !*is_winner {                                                       // Only take in consideration cards that haven't won yet
                // Find out if the drawn number was chosen by this bingo card, and store row_column where the number is located in the card
                let mut changed_value_at = None;
                // Flag that indicates whether we have already found a winning row/column           
                let mut striked = false;

                'bingo: for (row_index, row) in bingocard.0.iter_mut().enumerate() {   //  Cycle through the entire matrix, until we find a hit
                    for (column_index, (value, is_marked)) in row.iter_mut().enumerate() {
                        // If we find a hit, set the index where we found it and break out of the search
                        if value == drawn_number {                                               
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
                        if !is_drawn {          // Found unmarked value in row, break out
                            striked = false;
                            break;
                        }
                    }

                    if striked {               // Finished
                        *is_winner = true;     // This card has a winner ROW, change the flag and continue to next card
                        nr_not_won_yet-=1;
                        if nr_not_won_yet == 0 {
                            finished_value = *drawn_number; // If this is the last card to get striked, store the value that caused it
                        }
                        continue;              // Prevent from checking for column strike 
                    }

                    for row_index  in 0..bingocard.0.len() {  // Check each column for a strike (all values in column marked)
                        striked = true;

                        if !bingocard.0[row_index][changed_value_at.1].1 {
                            striked = false;
                            break;
                        }
                    }

                    if striked {                // Finished
                        *is_winner = true;      // This card has a winner COLUMN, change the flag and continue to next card
                        nr_not_won_yet-=1;      
                        if nr_not_won_yet == 0 {
                            finished_value = *drawn_number;
                        }
                    }
                }
            }
        }
    }

    // Print solution
    println!("{}", calculate_sum(&bingo[finished_card_index].0, finished_value));
}


// Calculate the sum according to the given criteria
// (Sum of all unmarked values in card) * last drawn value
fn calculate_sum(card: &BingoCard, value: i32) -> i32{ 
    card.0.iter().flatten().fold(0, |acc, value| if value.1 == false {acc + value.0} else {acc}) * value
}