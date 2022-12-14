fn main() {
    let sum: u32 = include_str!("../input.txt") //Read Data
        .lines() // Split into Lines
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2)) // Split into two compartments
        .map(|(first, second)| first.chars().find(|x| second.contains(*x)).unwrap()) //Find items that are in both compartments
        .map(|item| {
            if item.is_uppercase() {
                item as u32 - 64 + 26
            } else {
                item as u32 - 96
            }
        }) // Calculate their priority
        .sum(); // Sum them

    println!("{}", sum);
}
