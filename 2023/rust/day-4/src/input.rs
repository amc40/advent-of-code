use std::{collections::HashSet, fs::File, io::{BufReader, BufRead}};

use crate::ScratchCard;

pub fn read_scratch_cards(filename: &str) -> Result<Vec<ScratchCard>, String> {
    let file = File::open(filename)
        .map_err(|e| format!("Could not open file: {}. {}", filename, e))?;
    let buffered_reader = BufReader::new(file);
    buffered_reader.lines()
        .enumerate()
        .map(|(line_number, line_result)| 
        line_result.map_err(|e| format!("Error reading line {}: {}", line_number, e))
                .and_then(|line| parse_scratch_card(&line))
                .map_err(|e| format!("Error parsing scratch card on line {}: {}", line_number, e))
            
        )
        .collect()
}

fn parse_scratch_card(line: &str) -> Result<ScratchCard, String> {
    let mut card_description_and_numbers_strs = line.split(": ");

    let card_description_str = card_description_and_numbers_strs.next()
        .ok_or("Expected card description before ': ' separator")?;

    let all_card_numbers_str = card_description_and_numbers_strs.next()
        .ok_or("Expected card numbers after ': ' separator")?;
    
    let card_id = parse_card_id(card_description_str)?;

    let mut winning_and_this_card_numbers_strs = all_card_numbers_str.split(" | ");

    let winning_card_numbers_str = winning_and_this_card_numbers_strs.next()
        .ok_or("Expected winning card numbers before ' | '")?;

    let this_card_numbers_str = winning_and_this_card_numbers_strs.next()
        .ok_or("Expected this card's numbers after ' | '")?;

    let winning_numbers = parse_whitespace_separated_numbers(winning_card_numbers_str)?;

    let this_card_numbers = parse_whitespace_separated_numbers(this_card_numbers_str)?;   
    
    Ok(ScratchCard {
        id: card_id,
        winning_numbers,
        this_card_numbers
    })
}

fn parse_card_id(card_description_str: &str) -> Result<u32, String> {
    let mut card_text_and_card_id = card_description_str.split_whitespace();

    card_text_and_card_id.next();

    let card_id_str = card_text_and_card_id.next()
        .ok_or("Expected card id after 'Card '")?;
    card_id_str.parse()
        .map_err(|_: std::num::ParseIntError| format!("Invalid Card Id: {}", card_id_str))
}

fn parse_whitespace_separated_numbers(whitespace_separated_numbers_str: &str) -> Result<HashSet<u32>, String> {
    whitespace_separated_numbers_str.split_whitespace()
        .map(|number| 
            number.parse()
                .map_err(|_| format!("Not a valid number: {}", number))
        )
        .collect()
}