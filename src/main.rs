// From https://adventofcode.com/2021/day/3

// TODO some generic stuff needed for binary to digit code

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
                acc[i] += *digit as u32;
            }
            acc
        })
}

fn binary_digit_list_to_number(input: &Vec<u32>) -> u32 {
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

// TODO make these two functions generic and work for any number type

fn binary_digit_list_to_number_u8(input: &Vec<u8>) -> u32 {
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
        .map(|s| {
            let bs = String::into_bytes(s.to_string());
            bs.iter().map(|n| *n - '0' as u8).collect()    
        })
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

    let one_winners_count = binary_digit_list_to_number(&one_winners);
    let zero_winners_count = binary_digit_list_to_number(&zero_winners);

    one_winners_count * zero_winners_count
}

// Filter words including only those with the given valur at the specified bit position
fn filter_words(words: Vec<Vec<u8>>, target: u8, bit: usize) -> Vec<Vec<u8>> {

    words.into_iter()
        .filter(|word| word[bit] == target)
        .collect::<Vec<Vec<u8>>>()
}

fn get_winner(words: &Vec<Vec<u8>>, index: usize, most_common: bool) -> u8 {
    let col: Vec<u8> = words
        .iter()
        .map(|word| word[index])
        .collect();
    let ones: usize = col.iter().fold(0,|a,n| a + *n as usize);
    let num_words = words.len();

    let zeros = num_words - ones;

    if most_common {
        if ones >= zeros {
            1
        } else {
            0
        }
    } else {
        if ones >= zeros {
            0
        } else {
            1
        }
    }
}

fn solve_part_2(input: &str) -> u32 {

    let words: Vec<Vec<u8>> = input
        .lines()
        .map(|s| {
            let bs = String::into_bytes(s.to_string());
            bs.iter().map(|n| *n - '0' as u8).collect()    
        })
        .collect();

    let word_size = words[0].len();

    // println!("words {:?}", words);
    println!("num words {:?} size {:?}", words.len(), word_size);

    // Take a mutable copy of the words
    // For each bit from word_size-1 to 0
    //   get winning bit for the bit index 
    //   filter the word list for matches 
    // convert to number 
    // profit 

    // TODO this could be more DRY

    let mut candidates = words.clone();

    for bit in 0..word_size {
        let winner = get_winner(&candidates, bit, true);
        // println!("{:?} bit {:?} winner {:?}", candidates, bit, winner);
        candidates = filter_words(candidates, winner, bit);
        if candidates.len() == 1 {
            break;
        }
    }

    let oxygen = binary_digit_list_to_number_u8(&candidates[0]);

    candidates = words.clone();

    for bit in 0..word_size {
        let winner = get_winner(&candidates, bit, false);
        // println!("{:?} bit {:?} winner {:?}", candidates, bit, winner);
        candidates = filter_words(candidates, winner, bit);
        if candidates.len() == 1 {
            break;
        }
    }

    let scrubber = binary_digit_list_to_number_u8(&candidates[0]);

    println!("{:?} x {:?}", oxygen, scrubber);


   oxygen * scrubber 
}

fn main() {

   println!("{}", solve_part_1(SAMPLE)); 
   println!("{}", solve_part_1(INPUT)); // 3320834


   println!("{}", solve_part_2(SAMPLE)); 
   println!("{}", solve_part_2(INPUT));  // 4481199
}

