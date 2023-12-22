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
    GREEN,
    RED,
    BLUE
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

fn parse_game_id(game_str: &str) -> u32 {
    game_str[5..].parse().unwrap()
}

fn parse_cube_count(cube_count_str: &str) -> CubeCount {
    let cube_count_and_color: Vec<&str> = cube_count_str.split(" ")
        .collect();
    let cube_count: u32 = cube_count_and_color[0].parse().unwrap();
    let color: Color = cube_count_and_color[1].parse().unwrap();
    CubeCount {
        count: cube_count,
        color
    }
}

fn line_to_game(line: &str) -> Game {
    let game_and_reveals_str: Vec<&str> = line.split(": ")
        .collect();
    let game_id = parse_game_id(game_and_reveals_str[0]);

    let reveal_strs: Vec<&str> =  game_and_reveals_str[1]
        .split("; ")
        .collect();

    let reveal_cube_counts: Vec<Vec<CubeCount>> = reveal_strs.into_iter()
        .map(|reveal| reveal.split(", ")
            .map(|cube_count_str| parse_cube_count(cube_count_str))
            .collect()
        )
        .collect();

    Game {
        id: game_id,
        reveal_cube_counts
    }
}

fn read_games(filename: &str) {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let result: Vec<Game> = buf.lines()
        .map(|l| line_to_game(&l.expect("Could not parse line")))
        .collect();
    println!("{:?}", result);
}