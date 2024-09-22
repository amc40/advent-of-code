use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Lines}};

use itertools::Itertools;

#[derive(Debug)]
pub struct RangeMapEntry {
    pub source_start: u64,
    pub destination_start: u64,
    pub length: u64,
}

#[derive(Debug)]
pub struct Mapping {
    pub source_name: String,
    pub destination_name: String,
    pub mapping_entries: Vec<RangeMapEntry>,
}

#[derive(Debug)]
pub struct Input {
    pub seeds: Vec<u64>,
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
    lines.next()
        .ok_or("Expected at least one mapping")?
        .map_err(|e| format!("Error reading line to skip. {}", e))?;

    while let Some(mapping) = read_mapping(&mut lines)? {
        mappings_by_source_name.insert(mapping.source_name.clone(), mapping);
    }

    Ok(Input {
        seeds,
        mappings_by_source_name,
    })
}

fn read_seeds_from_first_line(line_iterator: &mut Lines<BufReader<File>>) -> Result<Vec<u64>, String> {
    let first_line = line_iterator.next()
        .ok_or("There are no lines in the provided: {}")?
        .map_err(|e| format!("Error reading line:\n{}", e))?;

    read_seeds_from_first_line_str(&first_line)
}

fn read_seeds_from_first_line_str(first_line: &str) -> Result<Vec<u64>, String> {
    let (_, seed_str) = first_line.splitn(2, ": ")
        .collect_tuple()
        .ok_or(format!("Could not parse seeds line:\n{}", first_line))?;
    parse_whitespace_separated_numbers(seed_str)
}

fn read_mapping(line_iterator: &mut Lines<BufReader<File>>) -> Result<Option<Mapping>, String> {
    if let Some(line) = line_iterator.next().transpose().map_err(|_| String::from("Error reading mapping header"))? {
        let (source_name, destination_name_with_suffix) = line.splitn(2, "-to-")
            .collect_tuple()
            .ok_or_else(|| format!("Error splitting mapping start line:\n{}", line))?;
        let destination_name = destination_name_with_suffix.strip_suffix(" map:")
            .ok_or_else(|| format!("Mapping start line does not end with ' map:'\n{}", line))?;
        
        let mapping_entries = read_mapping_entries(line_iterator)?;
        Ok(Some(Mapping {
            source_name: source_name.to_string(),
            destination_name: destination_name.to_string(),
            mapping_entries
        }))
    } else {  
        Ok(None)
    }
}

fn read_mapping_entries(lines: &mut Lines<BufReader<File>>) -> Result<Vec<RangeMapEntry>, String> {
    let mut mapping_entries = Vec::new();
    while let Some(line) = lines.next().transpose().map_err(|e| format!("Error reading mapping line: {}", e))? {
        if line.is_empty() {
            break;
        }
        let numbers_in_line = parse_whitespace_separated_numbers(&line)?;

        if numbers_in_line.len() != 3 {
            return Err("Expected exactly 3 numbers in mapping entry".to_owned());
        }
         mapping_entries.push(RangeMapEntry {
            source_start: numbers_in_line[1],
            destination_start: numbers_in_line[0],
            length: numbers_in_line[2],
        });
    }
    Ok(mapping_entries)
}

fn parse_whitespace_separated_numbers(whitespace_separated_numbers: &str) -> Result<Vec<u64>, String> {
    whitespace_separated_numbers.split_whitespace()
        .map(|num_str| num_str.parse()
            .map_err(|_| format!("Not a valid number: {}", num_str)))
        .collect()
}