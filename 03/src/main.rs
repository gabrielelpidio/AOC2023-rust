use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;

    let mut symbols: HashMap<_, _> = file
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .filter(|(_, char)| {
                    // println!("{}", char);
                    return *char == '*';
                })
                .map(|(x, _)| ((x, y), vec![]))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    file.lines().enumerate().for_each(|(y, line)| {
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

        line_numbers.clone().iter().for_each(|(x, y, number_str)| {
            let x_range = x.checked_sub(1).unwrap_or(0)..x + number_str.len() + 1;
            let y_range = y.checked_sub(1).unwrap_or(0)..y + 2;

            for y in y_range {
                for x in x_range.clone() {
                    if let Some(symbol) = symbols.get_mut(&(x, y)) {
                        symbol.push(number_str.parse::<usize>().unwrap());
                    }
                }
            }
        });
    });

    println!(
        "{:?}",
        symbols
            .iter()
            .filter(|x| x.1.len() == 2)
            .map(|x| x.1.iter().fold(1, |acc, curr| curr * acc))
            .sum::<usize>()
    );

    println!("{:?}", symbols);

    return Ok(());
}
