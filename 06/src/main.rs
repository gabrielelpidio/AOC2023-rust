use std::time::Instant;

fn calculate_win(time: &usize, distance: &usize) -> usize {
    let rang = 1..time - 1;

    return rang.filter(|r| r * (time - r) > *distance).count();
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let file = std::fs::read_to_string("input.txt")?;
    let binding = file.replace(" ", "");
    let (raw_times, raw_distances) = binding.split_once("\n").unwrap();

    let distance = raw_distances
        .split("Distance:")
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let time = raw_times
        .split("Time:")
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    println!("{:?}", calculate_win(&time, &distance));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    return Ok(());
}
