fn main() {
    // Solved following the method described here:
    // https://www.robertdickau.com/lattices.html
    //
    // Here the variable grid represents this image:
    // https://www.robertdickau.com/binomialpaths4.png
    // but rotated by 45 degrees clockwise.

    let size = 20;
    let nodes = (size + 1) * (size + 1);

    let mut grid: Vec<Vec<u64>> = vec![];

    // Init grid
    for row in 0..(size + 1) {

        // Add row
        grid.push(vec![]);

        for node in 0..(size + 1) {
            // Add node
            grid[row].push(0);
        }
    }

    let mut result: u64 = 0;

    // Init values
    for row in 0..grid.len() {

        let mut previous_row_value: u64 = 0;

        for column in 0..(grid[0].len()) {

            // Init first row and column to 1.
            // Otherwise calculate the value of the previous row till
            // the column of current node.
            if row == 0 || column == 0 {
                grid[row][column] = 1;

                if column == 0 {
                    previous_row_value += 1
                }

            } else {
                previous_row_value += grid[row - 1][column];
                grid[row][column] = previous_row_value;
            }
            
        }

        if row == grid.len() - 1 {
            result = previous_row_value as u64;
        }
    }

    // Result
    println!("{}", result);

}
