use std::collections::HashSet;

fn main() {
    fn direction_from_str(s: &str) -> (i32, i32) {
        match s {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => unreachable!(),
        }
    }
    let input = include_str!("../input.txt");
    let mut rope = [(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = Default::default();
    visited.insert(rope[9]);
    input
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .for_each(|(dir, num)| {
            (0..num.parse::<usize>().unwrap()).for_each(|_| {
                let direction = direction_from_str(dir);

                rope[0] = (rope[0].0 + direction.0, rope[0].1 + direction.1);
                (0..9).for_each(|i| {
                    let head = rope[i];
                    let tail = rope[i + 1];
                    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                        let delta = (
                            (head.0 - tail.0).signum() * (head.0 - tail.0).abs().min(1),
                            (head.1 - tail.1).signum() * (head.1 - tail.1).abs().min(1),
                        );

                        rope[i + 1] = (tail.0 + delta.0, tail.1 + delta.1);
                    }
                    visited.insert(rope[9]);
                });
            })
        });

    println!("{}", visited.len());

    //     fn compare(a: (i32, i32), b: (i32, i32)) -> Ordering {
    //         match a.0.cmp(&b.0) {
    //             Ordering::Equal => a.1.cmp(&b.1),
    //             x => x,
    //         }
    //     }
    //     let mut sorted = visited.iter().collect::<Vec<_>>();

    //     sorted.sort_by(|x, y| compare(**x, **y));
    //     sorted.iter().for_each(|x| {
    //         println!("{:?}", x);
    //     });
}
