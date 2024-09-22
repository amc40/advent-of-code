use std::collections::HashMap;

use crate::{input::Mapping, map_value};

pub fn get_locations_for_seeds(seeds: &Vec<u64>, mappings_by_source_name: &HashMap<String, Mapping>) -> Result<Vec<u64>, String> {

    let mut locations = Vec::new();

    for seed in seeds {
        let mut source_name: String = String::from("seed");
        let mut source_value = *seed;

        while source_name != "location" {
            let map_value::MappingResult {
                destination_name,
                destination_value,
            } = map_value::map_value(&source_name, source_value, mappings_by_source_name)?;
            source_name = destination_name;
            source_value = destination_value;
        }

        locations.push(source_value);
    }

    Ok(locations)
}