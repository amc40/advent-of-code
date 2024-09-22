use input::Input;

mod input;
mod get_locations_for_seeds;
mod map_value;

fn main() {
    let Input { seeds, mappings_by_source_name } = match input::read_input_from_file("day-5-input.txt") {
        Ok(input) => input,
        Err(err) => {
            println!("{}", err);
            return
        }
    };
    
    let mut locations = match get_locations_for_seeds::get_locations_for_seeds(&seeds, &mappings_by_source_name) {
        Ok(locations) => locations,
        Err(err) => {
            println!("{}", err);
            return
        }
    };

    locations.sort();
    
    println!("{:?}", locations);
}


