use std::{collections::HashSet, ops::Sub};

fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;

    let symbols: HashSet<_> = file
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .filter(|(_, char)| {
                    // println!("{}", char);
                    return !char.is_numeric() && *char != '.';
                })
                .map(|(x, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let numbers = file
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut last_was_number = false;
            let mut line_numbers: Vec<(usize, usize, String)> = vec![];

            for (x, char) in line.char_indices() {
                if char.is_numeric() {
                    if last_was_number {
                        line_numbers.last_mut().unwrap().2.push(char);
                    } else {
                        line_numbers.push((x, y, char.to_string()));
                    }
                    last_was_number = true;
                } else {
                    last_was_number = false;
                }
            }

            let filtered_line_numbers = line_numbers
                .clone()
                .iter()
                .filter(|(x, y, number_str)| {
                    let x_range = x.checked_sub(1).unwrap_or(0)..x + number_str.len() + 1;
                    let y_range = y.checked_sub(1).unwrap_or(0)..y + 2;

                    for y in y_range {
                        for x in x_range.clone() {
                            if symbols.contains(&(x, y)) {
                                return true;
                            }
                        }
                    }

                    return false;
                })
                .filter_map(|(_, _, n)| n.parse::<usize>().ok())
                .collect::<Vec<_>>();

            return filtered_line_numbers;
        })
        .flatten()
        // .collect::<Vec<_>>();
        .sum::<usize>();

    println!("{:?}", numbers);

    return Ok(());
}
