use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use strum_macros::EnumString;

fn main() {
    read_games("day-2-input.txt");
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Color {
    Green,
    Red,
    Blue
}

#[derive(Debug)]
struct CubeCount {
    count: u32,
    color: Color
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveal_cube_counts: Vec<Vec<CubeCount>>
}

const GAME_ID_START_INDEX: usize = 5;

fn parse_game_id(game_str: &str) -> Result<u32, String> {
    game_str[GAME_ID_START_INDEX..].parse()
        .map_err(|_| format!("Invalid game id: {}", &game_str[GAME_ID_START_INDEX..]))
}

fn parse_cube_count(cube_count_str: &str) -> Result<CubeCount, String> {
    let mut cube_count_and_color = cube_count_str.split(" ");
    let cube_count_str: &str = cube_count_and_color.next().ok_or("Cube count should be present")?;
    let color_str: &str = cube_count_and_color.next().ok_or("Cube color should be present")?;
    
    let cube_count: u32 = cube_count_str.parse()
        .map_err(|_| format!("Invalid number for cube count: {}",  {cube_count_str}))?;
    
    let color: Color = color_str.parse()
        .map_err(|_| format!("Invalid color value: {}",  {color_str}))?;

    Ok(CubeCount {
        count: cube_count,
        color
    })
}

fn parse_reveals(reveals_str: &str) -> Result<Vec<Vec<CubeCount>>, String> {
    let reveal_strs: Vec<&str> =  reveals_str
        .split("; ")
        .collect();

    reveal_strs.into_iter()
        .map(|reveal| reveal.split(", ")
            .map(|cube_count_str| parse_cube_count(cube_count_str))
            .collect()
        )
        .collect()
}

fn line_to_game(line: &str) -> Result<Game, String> {
    let game_and_reveals_str: Vec<&str> = line.split(": ")
        .collect();
    let game_id = parse_game_id(game_and_reveals_str[0])?;

    let reveal_cube_counts = parse_reveals(game_and_reveals_str[1])?;

    Ok(Game {
        id: game_id,
        reveal_cube_counts
    })
}

fn read_games(filename: &str) -> Result<Vec<Game>, String> {
    let file = File::open(filename)
        .map_err(|e| format!("Failed to open file '{}': {}", filename, e))?;
    let buf = BufReader::new(file);
    buf.lines()
        .enumerate()
        .map(|(line_number, line)| 
            line.map_err(|e| format!("Error reading line {}: {}", line_number, e))
                .and_then(|l| line_to_game(&l).map_err(|e| format!("Error in line {}: {}", line_number, e)))
        )
        .collect()
}