fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;

    let lines = file
        .lines()
        .map(|s| {
            s.split(" ")
                .filter_map(|n| n.parse::<isize>().ok())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut differences_sum = 0;

    for line in lines {
        let mut differences = vec![
            line.clone(),
            line.windows(2).map(|f| f[1] - f[0]).collect::<Vec<_>>(),
        ];

        while !differences.last().unwrap().iter().all(|f| *f == 0) {
            if differences.last().unwrap().len() == 1 {
                differences.push(vec![0]);
                continue;
            }
            differences.push(
                differences
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|f| f[1] - f[0])
                    .collect::<Vec<_>>(),
            );
        }

        println!("{:?}", differences);

        differences_sum += differences.iter().map(|f| f.last().unwrap()).sum::<isize>()
    }

    println!("{}", differences_sum);

    return Ok(());
}
