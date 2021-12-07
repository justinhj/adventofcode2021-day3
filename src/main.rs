// for r

const INPUT: &str = include_str!("../inputs/day3.txt");

const SAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

// Given the vector of words calculate the total ones in each column
fn totals(words: Vec<Vec<u8>>) -> Vec<u32> {
    let len = words[0].len();
    let counts = vec![0u32; len];

    words.iter().fold(counts,
        |mut acc,word| {
            for (i, digit) in word.iter().enumerate() {
                acc[i] += (*digit - '0' as u8) as u32;
            }
            acc
        })
}

fn binary_digit_list_to_number(input: Vec<u32>) -> u32 {
    let mut acc: u32 = 0;
    for (i,n) in input.iter().rev().enumerate() {
        acc += if *n == 1 {
            2u32.pow(i as u32)
        }
        else {
            0
        }
    }
    acc
}

fn solve(input: &str) -> u32 {

    let words: Vec<Vec<u8>> = input
        .lines()
        .map(|s| String::into_bytes(s.to_string()))
        .collect();
    let word_length = words[0].len();
    let half_num_words = words.len() / 2;

    // println!("{} words with length {}", words.len(), word_length);

    let totals = totals(words);

    // println!("{:?}", totals);

    // Generate the numbers from 1 winners and 0 winners
    
    let one_winners: Vec<u32> = totals.iter().map (
        |total| {
            if *total as usize >= half_num_words {
                1
            } else {
                0
            }
        }
    ).collect();
  
    let zero_winners: Vec<u32> = one_winners.iter().map(|n| if *n == 0 {1} else {0}).collect();

    let one_winners_count = binary_digit_list_to_number(one_winners);
    let zero_winners_count = binary_digit_list_to_number(zero_winners);

    one_winners_count * zero_winners_count
}


fn main() {

   println!("{}", solve(SAMPLE)); 
   println!("{}", solve(INPUT)); 


}

