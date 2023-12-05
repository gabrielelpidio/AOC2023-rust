#[derive(Debug)]
struct Map {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl Map {
    fn map_to_value(&self, value: usize) -> Option<usize> {
        let range = self.source_start..=self.source_start + (self.length - 1);
        // println!(
        //     "{} {:?} {} {}",
        //     self.source_start, range, self.dest_start, value
        // );
        if range.contains(&value) {
            return Some(usize::abs_diff(self.source_start, value) + self.dest_start);
        }
        return None;
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;

    let (raw_seeds, rest) = file.split_once("\n\n").unwrap();

    let seeds = raw_seeds
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let maps = rest
        .split("\n\n")
        .map(|x| {
            x.split_once(":\n")
                .unwrap()
                .1
                .split("\n")
                .map(|range| {
                    let map = range
                        .split(" ")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();

                    return Map {
                        dest_start: map[0],
                        source_start: map[1],
                        length: map[2],
                    };
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let locations = seeds
        .iter()
        .map(|x| {
            return maps.iter().fold(*x, |acc, curr| {
                return curr
                    .iter()
                    .find_map(|map| map.map_to_value(acc))
                    .unwrap_or(acc);
            });
        })
        .min();
    // .collect::<Vec<_>>();

    println!("{:?}", locations);

    return Ok(());
}
