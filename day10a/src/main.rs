fn main() {
    let input = include_str!("../input.txt");
    let mut x = 1;
    let mut c = 1;
    let mut s = 0;

    input.lines().for_each(|ins| {
        s += ((c + 20) % 40 == 0) as i32 * c * x;
        c += 1;
        if ins.starts_with("addx") {
            let v = ins.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            s += ((c + 20) % 40 == 0) as i32 * c * x;
            c += 1;
            x += v;
        };
    });

    println!("{s}");
}
