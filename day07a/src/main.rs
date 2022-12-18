fn main() {
    let input = include_str!("../input.txt");

    let mut stack = vec![("/", 0)];
    let mut final_count = vec![];

    for line in input.lines() {
        match line.split(' ').collect::<Vec<_>>().as_slice() {
            ["$", "cd", dir] => match dir {
                &".." => {
                    let dir = stack.pop().unwrap();
                    final_count.push(dir);
                    stack.last_mut().unwrap().1 += dir.1;
                }
                x => stack.push((x, 0)),
            },
            ["$", "ls"] => (),
            ["dir", _] => (),
            [amount, _] => stack.last_mut().unwrap().1 += amount.parse::<u32>().unwrap(),
            _ => (),
        }
    }

    while stack.len() > 1 {
        let dir = stack.pop().unwrap();
        stack.last_mut().unwrap().1 += dir.1;
        final_count.push(dir);
    }
    final_count.push(stack.pop().unwrap());

    println!(
        "{:?}",
        final_count
            .iter()
            .filter_map(|x| if x.1 <= 100_000 { Some(x.1) } else { None })
            .sum::<u32>()
    );
}
