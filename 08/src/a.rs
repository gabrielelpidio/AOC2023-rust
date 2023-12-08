use std::{
    collections::HashMap,
    io::{stdout, Write},
};

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
        .next()
        .unwrap()
        .trim()
        .split_once(" =")
        .unwrap()
        .0;

    let last_map = raw_maps
        .split("\n")
        .last()
        .unwrap()
        .trim()
        .split_once(" =")
        .unwrap()
        .0;

    let maps = raw_maps
        .split("\n")
        .map(|x| {
            let (r_k, r_v) = x.split_once("=").unwrap();

            let binding = r_v.replace("(", "").replace(")", "");

            let (v_a, v_b) = binding.trim().split_once(", ").unwrap();

            return (r_k.trim(), (v_a.to_string(), v_b.to_string()));
        })
        .collect::<HashMap<_, _>>();

    println!("{:?} {:?}", first_map, last_map);

    let mut current_map: String = "AAA".to_string();
    let mut steps: usize = 0;

    while current_map != "ZZZ" {
        let current_instruction = *instructions
            .get(steps % &instructions.len())
            .expect("Should have instruction");

        steps += 1;
        print!("\r {:?}", steps);

        let (l, r) = maps
            .get(current_map.clone().as_str())
            .expect("Map doesn't exist");

        if current_instruction == "L" {
            current_map = l.clone();
        } else {
            current_map = r.clone();
        }
        stdout.flush().unwrap();
    }

    println!("");

    return Ok(());
}
