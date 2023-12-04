fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;

    let sum: usize = file
        .lines()
        .filter_map(|line| {
            let binding: Vec<_> = line.split("").collect();
            let mut first: String = "".to_string();
            let mut last: String = "".to_string();

            for (idx, character) in binding.iter().enumerate() {
                let value = &binding[0..(idx)].join("");

                match value {
                    x if x.contains("one") => first.push('1'),
                    x if x.contains("two") => first.push('2'),
                    x if x.contains("three") => first.push('3'),
                    x if x.contains("four") => first.push('4'),
                    x if x.contains("five") => first.push('5'),
                    x if x.contains("six") => first.push('6'),
                    x if x.contains("seven") => first.push('7'),
                    x if x.contains("eight") => first.push('8'),
                    x if x.contains("nine") => first.push('9'),
                    _ => {}
                }

                if first.len() > 0 {
                    break;
                }

                if let Some(char) = character.parse::<usize>().ok() {
                    first.push_str(&char.to_string());
                    break;
                }
            }

            for (idx, character) in binding.iter().enumerate().rev() {
                let value = &binding[idx..binding.len()].join("");

                match value {
                    x if x.contains("one") => last.push('1'),
                    x if x.contains("two") => last.push('2'),
                    x if x.contains("three") => last.push('3'),
                    x if x.contains("four") => last.push('4'),
                    x if x.contains("five") => last.push('5'),
                    x if x.contains("six") => last.push('6'),
                    x if x.contains("seven") => last.push('7'),
                    x if x.contains("eight") => last.push('8'),
                    x if x.contains("nine") => last.push('9'),
                    _ => {}
                }

                if last.len() > 0 {
                    break;
                }

                if let Some(char) = character.parse::<usize>().ok() {
                    last.push_str(&char.to_string());
                    break;
                }
            }

            /*
            println!("{}", binding);
            let values: Vec<_> = binding
                .split("")
                .filter(|char| char.parse::<usize>().ok().is_some())
                .collect();

            let first = values.first().unwrap().to_owned();
            let last = values.last().unwrap().to_owned();
             */

            let concat = "".to_owned() + &first + &last;

            println!("{}", concat);

            return concat.parse::<usize>().ok();
        })
        .sum();

    println!("{}", sum);
    return Ok(());
}
