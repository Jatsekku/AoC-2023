use std::cmp;
use std::fs;

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_string(word: &str) -> Result<Color, &str> {
        match word {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("Unable to create variant"),
        }
    }
}

#[derive(Debug, Default)]
struct RoundData {
    red_cubes_quantity: u32,
    green_cubes_quantity: u32,
    blue_cubes_quantity: u32,
}

impl RoundData {
    fn new() -> Self {
        Default::default()
    }

    fn process_cube(s: &str) -> Result<(Color, u32), &str> {
        let (quantity, color) = s.trim().split_once(" ").ok_or("Invalid cube format")?;

        let color = Color::from_string(color)?;
        let quantity = quantity.parse().map_err(|_| "Quantity parsing failed")?;

        Ok((color, quantity))
    }

    fn from_string(line: &str) -> Result<Self, &str> {
        let mut round = RoundData::new();

        for cube_str in line.split(",") {
            match RoundData::process_cube(cube_str)? {
                (Color::Red, quantity) => round.red_cubes_quantity = quantity,
                (Color::Green, quantity) => round.green_cubes_quantity = quantity,
                (Color::Blue, quantity) => round.blue_cubes_quantity = quantity,
            }
        }

        Ok(round)
    }

    fn was_possible(&self) -> bool {
        self.red_cubes_quantity <= 12
            && self.green_cubes_quantity <= 13
            && self.blue_cubes_quantity <= 14
    }
}

struct GameData {
    id: u32,
    rounds: Vec<RoundData>,
}

impl GameData {
    fn from_string(line: &str) -> Result<Self, &str> {
        let (head, tail) = line.split_once(":").ok_or("Spliting failed")?;
        let (_, game_id) = head.split_once(" ").ok_or("Spliting failed")?;

        let rounds: Result<Vec<RoundData>, &str> =
            tail.split(";").map(|r| RoundData::from_string(r)).collect();

        Ok(Self {
            id: game_id.parse().map_err(|_| "Game ID Parsing failed")?,
            rounds: rounds?,
        })
    }

    fn was_possible(&self) -> bool {
        for round in &self.rounds {
            if !round.was_possible() {
                return false;
            }
        }

        true
    }

    fn calculate_min_set_power(self) -> u32 {
        let mut min_set = (0, 0, 0);
        for round in &self.rounds {
            min_set = (
                cmp::max(min_set.0, round.red_cubes_quantity),
                cmp::max(min_set.1, round.green_cubes_quantity),
                cmp::max(min_set.2, round.blue_cubes_quantity),
            );
        }

        min_set.0 * min_set.1 * min_set.2
    }
}

fn main() {
    let input = fs::read_to_string("inputs/d2p1_my_input").expect("Input file error");
    let possible_ids_sum: u32 = input
        .lines()
        .map(|l| GameData::from_string(l).unwrap().calculate_min_set_power())
        .sum();
    println!("Sum of the sets power {:?}", possible_ids_sum);
}
