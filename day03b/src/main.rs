fn main() {
    let sum: u32 = include_str!("../input.txt") // Read file
        .lines() //Split them into lines
        .collect::<Vec<_>>() // chunks() function only exists for vector
        .chunks(3) //Create a new iterator with three element at each step
        .map(|arr| {
            arr[0]
                .chars()
                .find(|x| arr[1].contains(*x) && arr[2].contains(*x))
                .unwrap()
        }) //Find items that are common in all three rucksacks
        .map(|item| {
            if item.is_uppercase() {
                item as u32 - 64 + 26
            } else {
                item as u32 - 96
            }
        }) // Calculate their priority
        .sum(); // Sum them

    println!("{:?}", sum);
}
