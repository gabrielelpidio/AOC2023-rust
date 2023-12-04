use std::{collections::HashSet, time::Instant};
fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let file = std::fs::read_to_string("input.txt")?;

    let mut instances: Vec<_> = file.lines().map(|_| 1).collect();

    file.lines().enumerate().for_each(|(card_idx, card)| {
        let (rest, raw_winning_numbers) = card.split_once("|").unwrap();
        let winning_numbers: Vec<_> = raw_winning_numbers
            .split(" ")
            .filter_map(|val| val.parse::<usize>().ok())
            .collect();

        let w_num_count = rest
            .split(" ")
            .filter_map(|val| val.parse::<usize>().ok())
            .filter(|value| winning_numbers.contains(&value))
            .count();

        let current_card_instances = *instances.get(card_idx).unwrap();

        for idx in card_idx + 1..card_idx + 1 + w_num_count {
            instances[idx] += current_card_instances;
        }
    });

    println!("{}", instances.iter().sum::<usize>());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    return Ok(());
}
