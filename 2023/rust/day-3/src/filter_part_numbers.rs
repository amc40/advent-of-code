use crate::numbers_in_each_line::NumberAndLocationInLine;

pub fn get_part_numbers(
    part_number_adjacency_mask: &Vec<Vec<bool>>,
    numbers_in_each_line: &Vec<Vec<NumberAndLocationInLine>>
) -> Vec<u32> {
    numbers_in_each_line.iter().enumerate()
        .flat_map(|(line_index, numbers_in_line)| {
            let part_number_adjacency_mask_line = &part_number_adjacency_mask[line_index];
            numbers_in_line.iter()
                .filter(|number_and_location| is_number_part_number(number_and_location, part_number_adjacency_mask_line))
        })
        .map(|NumberAndLocationInLine { value, .. }| *value)
        .collect::<Vec<_>>()
}

fn is_number_part_number(
    NumberAndLocationInLine { start_index, end_index, .. }: &NumberAndLocationInLine,
    part_number_adjacency_mask_line: &Vec<bool>
) -> bool {
    (*start_index..*end_index)
        .any(|character_index| part_number_adjacency_mask_line[character_index])
}