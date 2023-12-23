mod input;

use std::{collections::HashSet};

use input::{read_scratch_cards};

#[derive(Debug)]
pub struct ScratchCard {
    id: u32,
    winning_numbers: HashSet<u32>,
    this_card_numbers: HashSet<u32>
}

fn main() {
    let scratch_cards = match read_scratch_cards("day-4-input.txt") {
        Ok(scratch_cards) => scratch_cards,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let total_points = get_total_points(&scratch_cards);
    println!("{}", total_points);    
}

fn get_total_points(scratch_cards: &Vec<ScratchCard>) -> u32 {
    scratch_cards.iter()
        .map(|scratch_card| get_points(scratch_card))
        .sum()
}

fn get_points(scratch_card: &ScratchCard) -> u32 {
    match get_number_of_winning_numbers(scratch_card) {
        0 => return 0,
        greater_than_zero => 2_u32.checked_pow(greater_than_zero - 1)
            .ok_or(format!("2^{} exceeded max of u32", greater_than_zero))
            .unwrap()
    }
}

fn get_number_of_winning_numbers(scratch_card: &ScratchCard) -> u32 {
    scratch_card.this_card_numbers.iter()
        .filter(|this_card_number| scratch_card.winning_numbers.contains(this_card_number))
        .count()
        .try_into()
        .unwrap()
}