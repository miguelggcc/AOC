pub fn day1(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    let reindeers = input.split("\r\n\r\n");
    let max: u32 = reindeers
        .clone()
        .map(|r| r.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap();

    println!("Max calories are {}", max);

    let max_3: [u32; 3] = reindeers
        .map(|r| r.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .fold([0; 3], |mut max, c| {
            if c > max[0] {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = c;
            } else if c > max[1] {
                max[2] = max[1];
                max[1] = c;
            } else if c > max[2] {
                max[2] = c;
            }
            max
        });

    println!(
        "Max 3 are {:?}, with a total of {}",
        max_3,
        max_3.iter().sum::<u32>()
    );
}
