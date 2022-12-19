use std::collections::HashSet;

fn main() {
    fn direction_from_str(s: &str) -> (i32, i32) {
        return match s {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => unreachable!(),
        };
    }
    let input = include_str!("../input.txt");
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = Default::default();
    visited.insert(tail);
    input
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .for_each(|(dir, num)| {
            (0..num.parse::<usize>().unwrap()).for_each(|_| {
                let direction = direction_from_str(dir);
                head = (head.0 + direction.0, head.1 + direction.1);
                if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                    let delta = (
                        (head.0 - tail.0).signum() * (head.0 - tail.0).abs().min(1),
                        (head.1 - tail.1).signum() * (head.1 - tail.1).abs().min(1),
                    );

                    tail = (tail.0 + delta.0, tail.1 + delta.1);
                    visited.insert(tail);
                }
            })
        });

    println!("{}", visited.len());
    // fn compare(a: (i32, i32), b: (i32, i32)) -> Ordering {
    //     if a.0 > b.0 {
    //         return Ordering::Greater;
    //     } else if a.0 == b.0 {
    //         if a.1 > b.1 {
    //             return Ordering::Greater;
    //         } else if a.1 == b.1 {
    //             return Ordering::Equal;
    //         } else {
    //             return Ordering::Less;
    //         }
    //     } else {
    //         return Ordering::Less;
    //     }
    // }
    // let mut sorted = visited.iter().collect::<Vec<_>>();

    // sorted.sort_by(|x, y| compare(**x, **y));
    // sorted.iter().for_each(|x| {
    //     println!("{:?}", x);
    // });
}
