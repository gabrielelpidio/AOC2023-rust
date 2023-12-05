use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;

    let games = file
        .lines()
        .map(|game| {
            let mut game_limits: HashMap<_, _> = [("red", 0), ("green", 0), ("blue", 0)].into();

            game.split_once(": ").unwrap().1.split(";").for_each(|set| {
                return set.split(",").for_each(|color| {
                    let (number, color) = color.trim().split_once(" ").unwrap();

                    let current_limit = *game_limits.get(color).unwrap();

                    if number.parse::<usize>().unwrap() > current_limit {
                        game_limits.insert(color, number.parse().unwrap());
                    }
                });
            });

            return game_limits
                .iter()
                .map(|(_, v)| v)
                .fold(1, |acc, curr| curr * acc);
        })
        .sum::<usize>();

    println!("{:?}", games);
    // for game in games {
    // }

    return Ok(());
}
