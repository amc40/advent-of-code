use std::{fs::File, io::{BufRead, BufReader}};

pub fn read_lines(filename: &str) -> Result<Vec<String>, String>  {
    let file = File::open(filename)
        .map_err(|e| format!("Error opening file {}: {}", filename, e))?;
    let buffered_reader = BufReader::new(file);
    buffered_reader.lines()
        .enumerate()
        .try_fold(Vec::new(), |mut acc, (line_number, line_result)| {
            let line = line_result.map_err(|e| format!("Error reading line {}: {}", line_number, e))?;
            acc.push(line);
            Ok(acc)
        })
}