fn main() {
    // Read tree heights
    let tree_height = (include_str!("../input.txt"))
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible: [[bool; 99]; 99] = [[false; 99]; 99];

    // Check the visibility of each element O(n^2)
    for i in 0..99 {
        // first index goes from up to bottom
        // second index goes from left to right

        // Go from top to bottom and check visibility to the right
        let mut tallest = -1;
        for j in 0..99 {
            if tallest < tree_height[i][j] {
                visible[i][j] = true;
                tallest = tree_height[i][j];
            }
        }

        // Go from top to bottom and check visibility to the left
        let mut tallest = -1;
        for j in 0..99 {
            if tallest < tree_height[i][98 - j] {
                visible[i][98 - j] = true;
                tallest = tree_height[i][98 - j];
            }
        }

        // Go from left to right and check visibility to the bottom
        let mut tallest = -1;
        for j in 0..99 {
            if tallest < tree_height[j][i] {
                visible[j][i] = true;
                tallest = tree_height[j][i];
            }
        }

        // Go from left to right and check visibility to the up
        let mut tallest = -1;
        for j in 0..99 {
            if tallest < tree_height[98 - j][i] {
                visible[98 - j][i] = true;
                tallest = tree_height[98 - j][i];
            }
        }
    }

    println!(
        "{:?}",
        visible
            .iter()
            .map(|x| x.iter().filter(|x| **x).count() as u32)
            .sum::<u32>()
    );
}
