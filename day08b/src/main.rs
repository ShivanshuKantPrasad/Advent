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

    let mut scenic: [[i32; 99]; 99] = [[1; 99]; 99];

    // For each tree
    for i in 0..99 {
        for j in 0..99 {
            let height = tree_height[i][j];

            // first index goes from top to bottom
            // Second index goes from left to right

            // Check scenic in each direction
            // Subtract 1, Add 1 to change 0 based index to 1 based index
            // Wrap to deal with border values

            // Check right
            scenic[i][j] *= ((j + 1)..99)
                .map(|x| tree_height[i][x])
                .position(|h| height <= h)
                .unwrap_or_else(|| (98 - j).wrapping_sub(1))
                .wrapping_add(1) as i32;

            // Check left
            scenic[i][j] *= (0..j)
                .map(|x| tree_height[i][j - x - 1])
                .position(|h| height <= h)
                .unwrap_or_else(|| j.wrapping_sub(1))
                .wrapping_add(1) as i32;

            // Check bottom
            scenic[i][j] *= ((i + 1)..99)
                .map(|x| tree_height[x][j])
                .position(|h| height <= h)
                .unwrap_or_else(|| (98 - i).wrapping_sub(1))
                .wrapping_add(1) as i32;

            // Check up
            scenic[i][j] *= (0..i)
                .map(|x| tree_height[i - x - 1][j])
                .position(|h| height <= h)
                .unwrap_or_else(|| i.wrapping_sub(1))
                .wrapping_add(1) as i32;
        }
    }

    println!(
        "{:?}",
        scenic
            .iter()
            .map(|x| x.iter().max().unwrap())
            .max()
            .unwrap()
    )
}
