use std::{
    fs::File,
    io::{prelude::*, BufReader}, collections::HashMap
};

use strum::IntoEnumIterator;
use strum_macros::{EnumString, EnumIter};

#[derive(EnumString, EnumIter, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
enum Color {
    Green,
    Red,
    Blue
}

struct CubeCount {
    count: u32,
    color: Color
}

struct Game {
    id: u32,
    reveal_cube_counts: Vec<HashMap<Color, u32>>
}

fn main() {
    extension();
}

fn base() {
    let games = match read_games("day-2-input.txt") {
        Ok(games) => games,
        Err(err_msg) => {
            println!("{}", err_msg);
            return; 
        }
    };
    let max_cube_counts_for_colors: HashMap<Color, u32> = HashMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14)
    ]);
    let sum_of_of_possible_game_ids = sum_of_possible_game_ids(&games, &max_cube_counts_for_colors);
    println!("{}", sum_of_of_possible_game_ids);
}

fn extension() {
    let games = match read_games("day-2-input.txt") {
        Ok(games) => games,
        Err(err_msg) => {
            println!("{}", err_msg);
            return; 
        }
    };
    let sum_of_minimum_powers: u32 = games.iter()
        .map(|game| minimum_power_for_game(game))
        .sum();
    println!("{}", sum_of_minimum_powers);
}

fn minimum_power_for_game(game: &Game) -> u32 {
    Color::iter()
        .map(|color| max_cube_count_for_color(&game.reveal_cube_counts, &color))
        .fold(1, |acc, max_cube_count_for_color| acc * max_cube_count_for_color)
}

fn sum_of_possible_game_ids(games: &Vec<Game>, max_cube_counts_for_colors: &HashMap<Color, u32>) -> u32 {
    games.iter()
        .filter(|game| is_game_possible_with_cubes(game, max_cube_counts_for_colors))
        .map(|game| game.id)
        .sum()
}

fn is_game_possible_with_cubes(
    game: &Game,
    max_cube_counts_for_colors: &HashMap<Color, u32>
) -> bool {
    Color::iter().all(|color| -> bool {
        let max_cube_count_for_color = max_cube_count_for_color(&game.reveal_cube_counts, &color);
        max_cube_count_for_color <= max_cube_counts_for_colors[&color]
    })
}

fn max_cube_count_for_color(all_cube_counts: &Vec<HashMap<Color, u32>>, color: &Color) -> u32 {
    *all_cube_counts.into_iter()
        .map(|cube_counts| cube_counts.get(color).unwrap())
        .max().unwrap()
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

fn sum_cube_counts(cube_counts: Vec<CubeCount>) -> HashMap<Color, u32> {
    let mut reveal_cube_count_for_color: HashMap<Color, u32> = Color::iter()
    .map(|color| (color, 0))
    .collect();
    
    cube_counts.into_iter()
        .for_each(|reveal_cube_count|
            *reveal_cube_count_for_color.get_mut(&reveal_cube_count.color).unwrap() += reveal_cube_count.count
        );
    
    reveal_cube_count_for_color
}

fn parse_cube_counts_for_reveal(reveal: &str) -> Result<Vec<CubeCount>, String> {
    reveal.split(", ")
        .map(|cube_count_str| parse_cube_count(cube_count_str))
        .collect()
}

fn parse_reveals(reveals_str: &str) -> Result<Vec<HashMap<Color, u32>>, String> {
    let reveal_strs: Vec<&str> =  reveals_str
        .split("; ")
        .collect();

    reveal_strs.into_iter()
        .map(|reveal| 
            parse_cube_counts_for_reveal(reveal)
                .map(|cube_counts| sum_cube_counts(cube_counts))
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

