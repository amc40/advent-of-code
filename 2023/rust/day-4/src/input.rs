use std::{collections::HashSet, fs::File, io::{BufReader, BufRead}};

use itertools::Itertools;

use crate::ScratchCard;

pub fn read_scratch_cards(filename: &str) -> Result<Vec<ScratchCard>, String> {
    let file = File::open(filename)
        .map_err(|e| format!("Could not open file: {}. {}", filename, e))?;
    let buffered_reader = BufReader::new(file);
    buffered_reader.lines()
        .enumerate()
        .try_fold(Vec::new(), |mut acc, (line_number, line_result)| {
            let line = line_result.map_err(|e| format!("Error reading line {}: {}", line_number, e))?;
            let scratch_card = parse_scratch_card(&line)
                .map_err(|e| format!("Error parsing scratch card on line {}: {}", line_number, e))?;
            acc.push(scratch_card);
            Ok(acc)
        })
}

fn parse_scratch_card(line: &str) -> Result<ScratchCard, String> {
    let (card_description_str, all_card_numbers_str) = line.splitn(2, ": ")
        .collect_tuple()
        .ok_or("Expected ': ' separator")?;
    
    let card_id = parse_card_id(card_description_str)?;

    let (winning_card_numbers_str, this_card_numbers_str) = all_card_numbers_str.splitn(2, " | ")
        .collect_tuple()
        .ok_or("Expected card numbers to be separated by ' | '")?;

    let winning_numbers = parse_whitespace_separated_numbers(winning_card_numbers_str)?;

    let this_card_numbers = parse_whitespace_separated_numbers(this_card_numbers_str)?;   
    
    Ok(ScratchCard {
        id: card_id,
        winning_numbers,
        this_card_numbers
    })
}

fn parse_card_id(card_description_str: &str) -> Result<u32, String> {
    let (_, card_id_str) = card_description_str.split_whitespace()
        .collect_tuple()
        .ok_or("Expected 'Card {cardId}")?;
    card_id_str.parse()
        .map_err(|_: std::num::ParseIntError| format!("Invalid Card Id: {}", card_id_str))
}

fn parse_whitespace_separated_numbers(whitespace_separated_numbers_str: &str) -> Result<HashSet<u32>, String> {
    whitespace_separated_numbers_str.split_whitespace()
        .try_fold(HashSet::new(), |mut acc, number_str| {
            let number = number_str.parse()
                .map_err(|_| format!("Not a valid number: {}", number_str))?;
            acc.insert(number);
            Ok(acc)
        })
}