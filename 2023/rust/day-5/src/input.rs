use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Lines}};

use itertools::Itertools;

#[derive(Debug)]
pub struct RangeMapEntry {
    pub source_start: u32,
    pub destination_start: u32,
    pub length: u32,
}

#[derive(Debug)]
pub struct Mapping {
    pub source_name: String,
    pub destination_name: String,
    pub mapping_entries: Vec<RangeMapEntry>,
}

#[derive(Debug)]
pub struct Input {
    pub seeds: Vec<u32>,
    pub mappings_by_source_name: HashMap<String, Mapping>,
}

pub fn read_input_from_file(filename: &str) -> Result<Input, String> {
    let file = File::open(filename)
        .map_err(|e| format!("Could not open file: {}. {}", filename, e))?;
    let bufferred_reader = BufReader::new(file);

    let mut lines = bufferred_reader.lines();

    let seeds = read_seeds_from_first_line(&mut lines)?;

    let mut mappings_by_source_name = HashMap::new();

    // skip over blank line before mappings
    let skip_result = lines.next()
        .ok_or("Expected at least one mapping")?;
    skip_result.map_err(|e| format!("Error reading line to skip. {}", e))?;

    while let Some(mapping) = read_mapping(&mut lines)? {
        mappings_by_source_name.insert(mapping.source_name.clone(), mapping);
    }

    Ok(Input {
        seeds,
        mappings_by_source_name,
    })
}

fn read_seeds_from_first_line(line_iterator: &mut Lines<BufReader<File>>) -> Result<Vec<u32>, String> {
    let first_line_result = line_iterator.next()
        .ok_or("There are no lines in the provided: {}")?;
    let first_line = first_line_result
        .map_err(|e| format!("Error reading line:\n{}", e))?;

    read_seeds_from_first_line_str(&first_line)
}

fn read_seeds_from_first_line_str(first_line: &str) -> Result<Vec<u32>, String> {
    let (_, seed_str) = first_line.splitn(2, ": ")
        .collect_tuple()
        .ok_or(format!("Could not parse seeds line:\n{}", first_line))?;
    parse_whitespace_separated_numbers(seed_str)
}

fn read_mapping(line_iterator: &mut Lines<BufReader<File>>) -> Result<Option<Mapping>, String> {
    if let Some(line_result) = line_iterator.next() {
        let line = line_result
            .map_err(|e| format!("Error parsing line. {}", e))?;
        let (source_name, destination_name_with_suffix) = line.splitn(2, "-to-")
            .collect_tuple()
            .ok_or(format!("Error splitting mapping start line:\n{}", line))?;
        let destination_name = destination_name_with_suffix.strip_suffix(" map:")
            .ok_or(format!("Mapping start line does not end with ' map:'\n{}", line))?;
        
        let mapping_entries = read_mapping_entries(line_iterator)?;
        return Ok(Some(Mapping {
            source_name: source_name.to_string(),
            destination_name: destination_name.to_string(),
            mapping_entries
        }))
    } else {  
        return Ok(None)
    }
}

fn read_mapping_entries(line_iterator: &mut Lines<BufReader<File>>) -> Result<Vec<RangeMapEntry>, String> {
    let line_result = line_iterator.next()
        .ok_or(format!("There should be at least one range mapping"))?;
    let mut line = line_result
        .map_err(|e| format!("Error reading mapping line: {}", e))?;
    let mut mapping_entries = Vec::new();
    while !line.is_empty() {
        let numbers_in_line = parse_whitespace_separated_numbers(&line)?;
        if numbers_in_line.len() != 3 {
            return Err("Expected exactly 3 numbers in mapping entry".to_owned());
        }
        let source_range_start = numbers_in_line[0];
        let dest_range_start = numbers_in_line[1];
        let length = numbers_in_line[2];
        mapping_entries.push(RangeMapEntry {
            source_start: source_range_start,
            destination_start: dest_range_start,
            length: length,
        });
        if let Some(line_result) = line_iterator.next() {
            line = line_result
                .map_err(|e| format!("Error reading mapping line: {}", e))?
        } else {
            return Ok(mapping_entries)
        }
    }
    Ok(mapping_entries)
}

fn parse_whitespace_separated_numbers(whitespace_separated_numbers: &str) -> Result<Vec<u32>, String> {
    whitespace_separated_numbers.split_whitespace()
        .try_fold(Vec::new(), |mut acc_vec, number_str| {
            let number = number_str.parse()
                .map_err(|_| format!("Not a valid number: {}", number_str))?;
            acc_vec.push(number);
            Ok(acc_vec)
        })
}