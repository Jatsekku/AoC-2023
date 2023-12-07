use core::fmt;
use inline_colorization::*;
use std::fs;

#[derive(Debug, Default)]
struct Schematic {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
    marking: Vec<Vec<bool>>,
}

impl Schematic {
    fn new(value: &str) -> Result<Self, &str> {
        let mut schematic: Schematic = Default::default();
        schematic.width = value.lines().next().ok_or("No data")?.chars().count();
        schematic.height = 0;
        for line in value.lines() {
            let line_width = line.chars().count();
            if line_width != schematic.width {
                return Err("Invalid data format");
            }
            schematic.grid.push(line.chars().collect::<Vec<char>>());
            schematic.marking.push(vec![false; line_width]);
            schematic.height = schematic.height + 1;
        }

        Ok(schematic)
    }

    fn is_symbol(ch: char) -> bool {
        !ch.is_digit(10) && ch != '.'
    }

    fn get_field(&self, x: i32, y: i32) -> Option<&char> {
        if x > self.width as i32 || y > self.height as i32 {
            return None;
        }

        let row = match self.grid.get(y as usize) {
            None => return None,
            Some(l) => l,
        };

        row.get(x as usize)
    }

    fn get_mark(&self, x: i32, y: i32) -> Option<&bool> {
        if x > self.width as i32 || y > self.height as i32 {
            return None;
        }

        let row = match self.marking.get(y as usize) {
            None => return None,
            Some(l) => l,
        };

        row.get(x as usize)
    }

    fn set_mark(&mut self, x: i32, y: i32, mark: bool) {
        if x > self.width as i32 || y > self.height as i32 {
            return;
        }

        self.marking[y as usize][x as usize] = mark;
    }


    #[rustfmt::skip]
    fn has_char_adjacent_symbol(&self, x: i32, y: i32) -> bool {
        let nb8_array = [(-1, -1), (0, -1), (1, -1),
                         (-1,  0),          (1,  0),
                         (-1,  1), (0,  1), (1,  1)];

        for nb in nb8_array {
            if let Some(&ch) = self.get_field(x+nb.0, y+nb.1) {
                if Self::is_symbol(ch) {
                    return true;
                }
            }
        }

        return false;
    }

    fn get_parts_numbers_sum(&mut self) -> Result<u32, &'static str> {
        let mut total_sum = 0;

        for y in 0..self.height {
            let mut current_number = 0;
            let mut was_symbol = false;

            for x in 0..self.width {
                let ch = self
                    .get_field(x as i32, y as i32)
                    .ok_or("Acces field failed")?;
                let is_digit = ch.is_digit(10);

                if is_digit {
                    let digit = ch.to_digit(10).ok_or("Char to digit failed")?;
                    current_number = current_number * 10;
                    current_number = current_number + digit;
                    was_symbol = was_symbol || self.has_char_adjacent_symbol(x as i32, y as i32);
                    self.set_mark(
                        x as i32,
                        y as i32,
                        self.has_char_adjacent_symbol(x as i32, y as i32),
                    );
                }

                if !is_digit || x == (self.width - 1) {
                    if was_symbol {
                        total_sum = total_sum + current_number;
                    }

                    current_number = 0;
                    was_symbol = false;
                }
            }
        }

        Ok(total_sum)
    }
}

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "width:{}, height: {}\n", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_mark(x as i32, y as i32) {
                    Some(m) => {
                        if *m {
                            write!(f, "{color_red}{}{color_reset}", self.grid[y][x])?;
                        } else {
                            write!(f, "{}", self.grid[y][x])?;
                        }
                    }
                    None => (),
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("inputs/d3_my_input").expect("Input file error");
    let mut schematic = Schematic::new(&input).unwrap();
    print!("{:?}\n\n", schematic.get_parts_numbers_sum().unwrap());
}
