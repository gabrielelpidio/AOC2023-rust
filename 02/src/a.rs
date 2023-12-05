use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let limits: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)].into();
    let file = std::fs::read_to_string("input.txt")?;

    let games = file
        .lines()
        .map(|game| {
            return game
                .split_once(": ")
                .unwrap()
                .1
                .split(";")
                .map(|set| {
                    return set
                        .split(",")
                        .map(|color| {
                            let (number, color) = color.trim().split_once(" ").unwrap();

                            return (color, number.parse::<usize>().unwrap());
                        })
                        .collect::<Vec<_>>();
                })
                .collect::<Vec<_>>();
        })
        .enumerate()
        .filter(|(_, game)| {
            return game
                .iter()
                .find(|set| {
                    match set
                        .iter()
                        .find(|(color, number)| number > limits.get(color).unwrap())
                    {
                        Some(_) => return true,
                        None => return false,
                    }
                })
                .is_none();
        })
        .map(|(k, v)| {
            println!("{} {:?}", k, v);
            return k + 1;
        })
        .sum::<usize>();

    println!("{:?}", games);
    // for game in games {
    // }

    return Ok(());
}
