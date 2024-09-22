use std::collections::HashMap;

use crate::input::{Mapping, RangeMapEntry};

pub struct MappingResult {
    pub destination_name: String,
    pub destination_value: u64,
}

pub fn map_value(
    source_name: &String,
    source_value: u64,
    mappings_by_source_name: &HashMap<String, Mapping>
) -> Result<MappingResult, String> {
    let mapping = mappings_by_source_name.get(source_name)
        .ok_or(format!("Could not find mapping for source: {}", source_name))?;
    let destination_value = get_relevant_mapping_entry(source_value, &mapping)
        .map_or(
            source_value, 
            |RangeMapEntry { source_start, destination_start, .. }| {
                destination_start + (source_value - source_start)
            }
        );
    Ok(MappingResult {
        destination_name: mapping.destination_name.clone(),
        destination_value,
    })
}

fn get_relevant_mapping_entry(source_value: u64, mapping: &Mapping) -> Option<&RangeMapEntry> {
    mapping.mapping_entries.iter()
        .find(|RangeMapEntry { source_start, length, ..} |  
            *source_start <= source_value && source_value < *source_start + *length )
}