use std::fs;

fn main() {
    let max_calories = fs::read_to_string("input.txt")   //Open file
        .expect("Unable to read file")                       //Error check
        .split("\n\n")                                       //Separate Elves
        .map(|elf| {                                         //For each elf
            elf.split("\n")                                  //Separate calories
                .filter_map(|x| x.parse::<i32>().ok())       //Parse calories
                .sum::<i32>()                                //Add calories
        })
        .max();

    println!("{max_calories:?}");
}
