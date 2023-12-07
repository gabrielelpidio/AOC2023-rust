use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Hand {
    hand: Vec<String>,
}

fn get_stregth(card: &str) -> Option<usize> {
    let strength: Vec<&str> = vec![
        "A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J",
    ];

    return strength.iter().position(|c| c == &card);
}

impl Hand {
    fn get_type(&self) -> usize {
        let mut matches = self.hand.iter().fold(HashMap::new(), |mut map, s| {
            map.insert(s.to_string(), map.get(s).unwrap_or(&0) + 1);

            return map;
        });

        if let Some(js) = matches.clone().get("J") {
            let mut temp_matches = matches;

            temp_matches.remove("J");

            let mut temp_matches = temp_matches.into_iter().collect::<Vec<_>>();

            temp_matches.sort_by(|(_, a), (_, b)| b.cmp(a));

            if temp_matches.len() == 0 {
                temp_matches.push(("J".to_string(), *js));
            } else {
                temp_matches.first_mut().unwrap().1 += js;
            }

            matches = temp_matches.into_iter().collect::<HashMap<_, _>>();
        }

        if matches.len() == 1 {
            return 7;
        }

        if matches.len() == 2 {
            if matches.iter().any(|(_x, v)| v == &4) {
                return 6;
            }
            return 5;
        }

        if matches.len() == 3 {
            if matches.iter().any(|(_x, v)| v == &3) {
                return 4;
            }

            return 3;
        }

        if matches.len() == 5 {
            return 1;
        }

        return 2;
    }

    fn compare_hand(&self, other: &Hand) -> Ordering {
        let a_type = self.get_type();
        let b_type = other.get_type();

        // println!("{:?}, {:?}", (a_type, b_type), (&self.hand, &other.hand));

        if a_type > b_type {
            return Ordering::Greater;
        }
        if b_type > a_type {
            return Ordering::Less;
        }

        for (idx, card) in self.hand.iter().enumerate() {
            let s1 = get_stregth(card);
            let s2 = get_stregth(other.hand.get(idx).unwrap());
            if s1 > s2 {
                return Ordering::Less;
            } else if s1 < s2 {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;
    let lines = file.lines();

    let mut hands = lines
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bet)| {
            (
                Hand {
                    hand: hand
                        .trim()
                        .split("")
                        .filter(|x| !x.is_empty())
                        .map(|x| x.to_string())
                        .collect(),
                },
                bet.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    hands.sort_by(|(a, _), (b, _)| {
        return a.compare_hand(b);
    });

    println!(
        "{:?}",
        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.1)
            .sum::<usize>() // .collect::<Vec<_>>()
    );

    return Ok(());
}
