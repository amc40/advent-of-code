const NUMBER_BASE: u32 = 10;

#[derive(Debug)]
#[derive(Clone)]
pub struct NumberAndLocationInLine {
    pub value: u32,
    // inclusive
    pub start_index: usize,
    // exclusive
    pub end_index: usize, 
}

struct InProgressNumberAndStartLocationInLine {
    value: u32,
    start_index: usize,
}

pub fn get_numbers_in_each_line(lines: &Vec<String>) -> Vec<Vec<NumberAndLocationInLine>> {
    let n_lines = lines.len();

    if n_lines == 0 {
        return Vec::new();
    } ;

    let mut numbers_and_locations: Vec<Vec<NumberAndLocationInLine>> = vec![Vec::new(); n_lines];

    let mut in_progress_number_opt: Option<InProgressNumberAndStartLocationInLine> = None;

    for (line_index, line) in lines.iter().enumerate() {
        let numbers_and_locations_in_current_line = &mut numbers_and_locations[line_index];
        for (character_index, character) in line.chars().enumerate() {
            if let Some(digit) = character.to_digit(NUMBER_BASE) {
                add_digit_to_in_progress_or_start_new(digit, character_index, &mut in_progress_number_opt);
            } else {
                push_completed_number_if_present(numbers_and_locations_in_current_line, &mut in_progress_number_opt, character_index);
            }
        }
        push_completed_number_if_present(numbers_and_locations_in_current_line, &mut in_progress_number_opt, line.len());
    };

    numbers_and_locations
}

fn add_digit_to_in_progress_or_start_new(
    digit: u32,
    digit_index: usize,
    in_progress_number_opt:  &mut Option<InProgressNumberAndStartLocationInLine>,
) {
    if let Some(in_progress_number) = in_progress_number_opt {
        in_progress_number.value = (in_progress_number.value * NUMBER_BASE) + digit;
    } else {
        *in_progress_number_opt = Some(InProgressNumberAndStartLocationInLine {
            value: digit,
            start_index: digit_index,
        });
    }
}

fn push_completed_number_if_present(
    numbers_and_locations_in_current_line: &mut Vec<NumberAndLocationInLine>, 
    in_progress: &mut Option<InProgressNumberAndStartLocationInLine>,
    current_index: usize,
) {
    if let Some(in_progress_number) = in_progress.take() {
        numbers_and_locations_in_current_line.push(NumberAndLocationInLine {
            value: in_progress_number.value,
            start_index: in_progress_number.start_index,
            end_index: current_index,
        });
    }
}


