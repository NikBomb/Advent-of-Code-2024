

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);   
}

fn process(input: &str) -> String {
    let mut res: i32 = 0;

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start_pos: Vec<(usize, usize)> = Vec::new();
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    for (i, row) in matrix.iter().enumerate() {
        if i > rows {
            rows = i;
        }
        for (j, ch) in row.iter().enumerate() {
            if j > cols {
                cols = j;
            }
            if *ch == 'A' {
                start_pos.push((i, j));
            }
        }
    }

    //From Start pos check in every direction the MAS word in shape of X


    for pos in start_pos {
        let mut north_east: Vec<char> = Vec::new();
        let mut north_west: Vec<char> = Vec::new();

        //Check NE
        if (pos.0 > 0) && (pos.1 < cols) {
            let mut c : char = matrix[pos.0 - 1][pos.1 + 1];
            if c == 'M' || c == 'S' {
                north_east.push(c);
                //Check SW
                if (pos.0 < rows) && (pos.1 > 0) {
                    c = matrix[pos.0 + 1][pos.1 - 1];
                    if c == 'M' || c == 'S' {
                        north_east.push(c);
                    }
                }
            }
        }

        //Check NW
        if (pos.0 > 0) && (pos.1 > 0) {
            let mut c : char = matrix[pos.0 - 1][pos.1 - 1];
            if c == 'M' || c == 'S' {
                north_west.push(c);
                //Check SE
                if (pos.0 < rows) && (pos.1 < cols) {
                    c = matrix[pos.0 + 1][pos.1 + 1];
                    if c == 'M' || c == 'S' {
                        north_west.push(c);
                    }
                }
            }
        }

        if north_west.len() == 2 && north_west.contains(&'S') && north_west.contains(&'M') && north_east.len() == 2 && north_east.contains(&'S') && north_east.contains(&'M') {
            res += 1;
        }
    }
    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".to_string();
        let result = process(input.as_ref());
        assert_eq!(result, "9");
    }
}