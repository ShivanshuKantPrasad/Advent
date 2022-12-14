fn main() {
    let count = include_str!("../input.txt") // Read the file
        .lines() // Split into lines, each line is a pair
        .map(|pair| pair.split(",")) // Split the pair up
        .map(|pair| {
            pair.map(|y| {
                y.split("-")
                    .map(|t| t.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>() // Split the section start and end value
            })
            .collect::<Vec<_>>()
        })
        .filter(|pair| {
            (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1])
                || (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
        }) // Find the sections that completely overlap and filter them
        .count(); // Count them
    println!("{:?}", count);
}
