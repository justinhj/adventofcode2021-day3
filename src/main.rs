// From https://adventofcode.com/2021/day/3

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
fn totals(words: &Vec<Vec<u8>>) -> Vec<u32> {
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

fn solve_part_1(input: &str) -> u32 {

    let words: Vec<Vec<u8>> = input
        .lines()
        .map(|s| String::into_bytes(s.to_string()))
        .collect();
    let half_num_words = words.len() / 2;

    // println!("{} words with length {}", words.len(), word_length);

    let totals = totals(&words);

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

// Filter words including only those with the given valur at the specified bit position
fn filter_words(words: Vec<Vec<u8>>, target: &u8, bit: usize) -> Vec<Vec<u8>> {

    words.into_iter()
        .filter(|word| word[bit] == *target)
        .collect::<Vec<Vec<u8>>>()
}

fn solve_part_2(input: &str) -> u32 {

    let words: Vec<Vec<u8>> = input
        .lines()
        .map(|s| String::into_bytes(s.to_string()))
        .collect();
    let half_num_words = words.len() / 2;
    let word_length = words[0].len();

    println!("num words {:?}", words.len());

    let totals = totals(&words);
    println!("totals {:?}", totals);

    // Generate the winners (1,0) for each bit position
    // If it is a draw then 1 wins

    let winners: Vec<u8> = totals.iter().map (
        |total| {
            if *total as usize >= half_num_words {
                '1' as u8
            } else {
                '0' as u8
            }
        }
    ).collect();

    println!("winners {:?}", winners);

    // probably can negate winners to get losers
    
    // now eliminate non winners from each bit position until only word is left
    let mut candidates = words.clone();

    for (bit, winner) in winners.iter().rev().enumerate() {
        candidates = filter_words(candidates, &winner, bit);
        println!("{:?}", candidates);
        if candidates.len() == 1 {
            break;
        }
    }

    println!("winrar {:?}", candidates[0]);
    let winners: Vec<u32> = candidates[0].iter().map(|n| *n as u32 - '0' as u32).collect(); 
    let winners_number = binary_digit_list_to_number(winners);

    winners_number
}

fn main() {

   println!("{}", solve_part_1(SAMPLE)); 
   println!("{}", solve_part_1(INPUT)); 


   println!("{}", solve_part_2(SAMPLE)); 
   //println!("{}", solve_part_2(INPUT)); 
}

