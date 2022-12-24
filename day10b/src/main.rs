fn main() {
    let input = include_str!("../input.txt");
    let mut x = 1;
    let mut c = 0;
    let mut t = Vec::with_capacity(40 * 60);

    input.lines().for_each(|ins| {
        t.push(((x - 1 <= c % 40 && x + 1 >= c % 40) as u8 * 3 + 32) as char);
        c += 1;

        if ins.starts_with("addx") {
            t.push(((x - 1 <= c % 40 && x + 1 >= c % 40) as u8 * 3 + 32) as char);
            let v = ins.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            c += 1;
            x += v;
        };
    });

    t.chunks(40).for_each(|t| {
        for c in t {
            print!("{c}")
        }
        println!();
    });
}
