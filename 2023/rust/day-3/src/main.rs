mod input;
mod part_number_adjacency_mask;
mod numbers_in_each_line;
mod filter_part_numbers;

fn main() {
    let lines = match input::read_lines("day-3-input.txt") {
        Ok(lines) => lines,
        Err(e) => {
            println!("{}", e);
            return
        }
    };

    let part_number_adjacency_mask = part_number_adjacency_mask::get_part_number_adjacency_mask(&lines);

    let numbers_in_each_line = numbers_in_each_line::get_numbers_in_each_line(&lines);

    let part_numbers = filter_part_numbers::get_part_numbers(&part_number_adjacency_mask, &numbers_in_each_line);

    println!("{:?}", part_numbers);

    let part_number_total: u32 = part_numbers.iter().sum();

    println!("sum: {}", part_number_total);
}
