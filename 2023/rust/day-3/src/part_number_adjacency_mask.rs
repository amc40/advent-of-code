use std::cmp;

pub fn get_part_number_adjacency_mask(lines: &Vec<String>) -> Vec<Vec<bool>> {
    let n_lines = lines.len();

    if n_lines == 0 {
        return Vec::new();
    } ;

    let line_length = lines[0].len();

    let mut bitmask = vec![vec![false; line_length]; n_lines];

    lines.into_iter()
        .enumerate()
        .for_each(|(line_number, line)| {
           line.chars()
                .enumerate()
                .for_each(|(char_number, character)| {
                    if is_symbol(character) {
                        update_adjacency_matrix_for_symbol_at_location(
                            &mut bitmask,
                            n_lines,
                            line_length,
                            line_number,
                            char_number
                        )
                    }
                }) 
        });
    
    bitmask
}

fn is_symbol(character: char) -> bool {
    !character.is_numeric() && character != '.'
}

fn update_adjacency_matrix_for_symbol_at_location(
    bitmask: &mut Vec<Vec<bool>>,
    n_lines: usize,
    line_length: usize,
    symbol_line_number: usize,
    symbol_char_number: usize
) {
    for line_number in cmp::max(symbol_line_number - 1, 0)..cmp::min(symbol_line_number + 2, n_lines) {
        for char_number in cmp::max(symbol_char_number - 1, 0)..cmp::min(symbol_char_number + 2, line_length) {
            bitmask[line_number][char_number] = true;
        }
    }
}