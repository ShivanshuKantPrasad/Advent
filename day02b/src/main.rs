fn main() {
    let input = include_str!("../input.txt");
    let rounds = input.lines();
    let mut total_score = 0;
    for round in rounds {
        let a: Vec<&str> = round.split(" ").collect::<_>();
        let mut score = match a[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        };
        score += match (a[0], a[1]) {
            ("A", x) => match x {
                "X" => 3,
                "Y" => 1,
                "Z" => 2,
                _ => 0,
            },
            ("B", x) => match x {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            },
            ("C", x) => match x {
                "X" => 2,
                "Y" => 3,
                "Z" => 1,
                _ => 0,
            },
            _ => 0,
        };
        total_score += score;
    }
    println!("Answer: {:?}", total_score);
}
