use std::{
    collections::HashMap,
    io::{stdout, Write},
};

use num::integer::lcm;

fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout();

    let file = std::fs::read_to_string("input.txt")?;
    let (raw_instructions, raw_maps) = file.split_once("\n\n").unwrap();

    let instructions = raw_instructions
        .split("")
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let first_map = raw_maps
        .split("\n")
        .into_iter()
        .filter_map(|x| {
            let map_key = x.split_once(" =").unwrap().0;
            if map_key.ends_with("A") {
                return Some(map_key);
            }
            return None;
        })
        .collect::<Vec<_>>();

    let maps = raw_maps
        .split("\n")
        .map(|x| {
            let (r_k, r_v) = x.split_once("=").unwrap();

            let binding = r_v.replace("(", "").replace(")", "");

            let (v_a, v_b) = binding.trim().split_once(", ").unwrap();

            return (r_k.trim(), (v_a.to_string(), v_b.to_string()));
        })
        .collect::<HashMap<_, _>>();

    println!("{:?}", first_map);

    let steps_per_map = raw_maps
        .split("\n")
        .into_iter()
        .filter_map(|x| {
            let map_key = x.split_once(" =").unwrap().0;
            if map_key.ends_with("A") {
                return Some(map_key);
            }
            return None;
        })
        .map(|x| {
            let mut steps: usize = 0;
            let mut current_map = x.to_string();

            while !current_map.ends_with("Z") {
                let current_instruction = *instructions
                    .get(steps % &instructions.len())
                    .expect("Should have instruction");

                steps += 1;

                let (l, r) = maps.get(current_map.as_str()).expect("Map doesn't exist");

                if current_instruction == "L" {
                    current_map = l.to_string();
                } else {
                    current_map = r.to_string();
                }
                stdout.flush().unwrap();
            }
            return steps;
        })
        .collect::<Vec<_>>();

    println!(
        "{:?}",
        steps_per_map.iter().fold(1, |acc, curr| lcm(*curr, acc))
    );

    return Ok(());
}
