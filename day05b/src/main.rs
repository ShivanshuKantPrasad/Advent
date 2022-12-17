fn main() {
    let data = include_str!("../input.txt");
    let (crates, instructions) = data.split_once("\n\n").unwrap();
    let length = crates
        .lines()
        .last()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    println!("{length}");
    let mut stacks: [Vec<char>; 10] = Default::default();
    for line in crates.lines().rev().skip(1) {
        let mut i = 0;
        for char in line.chars() {
            i += 1;
            if char.is_alphabetic() {
                // 2 -> 0, 6 -> 1, 10 -> 2
                // (n - 2) / 4
                stacks[(i - 2) / 4].push(char);
            }
        }
    }
    for instruction in instructions.lines() {
        let bit = instruction
            .split(' ')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>();
        if let [number, from, to] = bit[..] {
            let mut temp: Vec<char> = vec![];
            for _ in 0..number {
                temp.push(stacks[from - 1].pop().unwrap());
            }
            for _ in 0..number {
                stacks[to - 1].push(temp.pop().unwrap());
            }
        }
    }

    for stack in stacks {
        println!("{stack:?}");
    }
}
