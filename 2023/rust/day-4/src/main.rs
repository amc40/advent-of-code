mod input;

use std::collections::HashSet;

use input::read_scratch_cards;

#[derive(Debug)]
pub struct ScratchCard {
    id: u32,
    winning_numbers: HashSet<u32>,
    this_card_numbers: HashSet<u32>
}

fn main() {
   extension();
}

fn extension() {
    let scratch_cards = match read_scratch_cards("day-4-input.txt") {
        Ok(scratch_cards) => scratch_cards,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut card_copy_counts: Vec<u32> = vec![0; scratch_cards.len()];

    scratch_cards.iter()
        .for_each(|scratch_card| add_won_scratch_card_copies(&scratch_card, &mut card_copy_counts));

    let total_cards = get_total_cards(&card_copy_counts);

    println!("{}", total_cards);
}

fn get_total_cards(card_copy_counts: &Vec<u32>) -> u32 {
    let card_copy_count: u32 = card_copy_counts.iter().sum();
    let originals_count: u32 = card_copy_counts.iter().count().try_into().unwrap();
    card_copy_count + originals_count
}

fn add_won_scratch_card_copies(scratch_card: &ScratchCard, card_copy_counts: &mut Vec<u32>) {
    let current_card_index: usize = (scratch_card.id - 1).try_into().unwrap();
    let next_card_index = scratch_card.id;

    let number_of_winning_numbers = get_number_of_winning_numbers(scratch_card);
    let number_of_copies = card_copy_counts[current_card_index] + 1;

    for card_index_u32 in next_card_index..next_card_index + number_of_winning_numbers {
        let card_index: usize = card_index_u32.try_into().unwrap();
        card_copy_counts[card_index] += number_of_copies;
    }
}

fn base() {
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