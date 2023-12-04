use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;

    let mut sum: usize = 0;

    let lines = file.lines().for_each(|card| {
        let (rest, raw_winning_numbers) = card.split_once("|").unwrap();
        let winning_numbers: Vec<_> = raw_winning_numbers
            .split(" ")
            .filter_map(|val| val.parse::<usize>().ok())
            .collect();

        let my_numbers: Vec<_> = rest
            .split_once(":")
            .unwrap()
            .1
            .split(" ")
            .filter_map(|val| val.parse::<usize>().ok())
            .collect();

        let w_num_count = my_numbers
            .iter()
            .filter(|value| winning_numbers.contains(&value))
            .count();

        if w_num_count > 0 {
            if w_num_count == 1 {
                sum += 1
            } else {
                let base: usize = 2;
                sum += base.pow((w_num_count - 1).try_into().unwrap())
            }
        }

        println!("{:?}, {:?}", winning_numbers, w_num_count)
    });

    println!("{}", sum);

    return Ok(());
}
