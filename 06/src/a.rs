use std::ops::Mul;

fn calculate_win(time: &usize, distance: &usize) -> usize {
    let rang = 1..time - 1;

    return rang.filter(|r| r * (time - r) > *distance).count();
}

fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;
    let (raw_times, raw_distances) = file.split_once("\n").unwrap();

    let td = raw_times
        .split(" ")
        .filter_map(|x| x.parse::<usize>().ok())
        .zip(
            raw_distances
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok()),
        )
        .map(|(t, d)| calculate_win(&t, &d))
        .fold(1, |acc, v| acc * v);

    println!("{:?}", td);

    return Ok(());
}
