fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;

    let sum: usize = file
        .lines()
        .filter_map(|line| {
            let values: Vec<_> = line
                .split("")
                .filter(|char| char.parse::<usize>().ok().is_some())
                .collect();

            let first = values.first().unwrap().to_owned();
            let last = values.last().unwrap().to_owned();

            let concat = "".to_owned() + first + last;

            return concat.parse::<usize>().ok();
        })
        .sum();

    println!("{}", sum);
    return Ok(());
}
