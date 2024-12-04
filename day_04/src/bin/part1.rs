

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);   
}

fn process(input: &str) -> String {
    let mut res: i32 = 0;
    let mut resN: i32 = 0;
    let mut resS: i32 = 0;
    let mut resE: i32 = 0;
    let mut resW: i32 = 0;
    let mut resNW: i32 = 0;
    let mut resNE: i32 = 0;
    let mut resSW: i32 = 0;
    let mut resSE: i32 = 0;
    
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
            if *ch == 'X' {
                start_pos.push((i, j));
            }
        }
    }

    //From Start pos check in every direction the XMAS word


    for pos in start_pos {
        //Check E 
        if pos.1 <= cols - 3 {
            if matrix[pos.0][pos.1 + 1] == 'M' && matrix[pos.0][pos.1 + 2] == 'A' && matrix[pos.0][pos.1 + 3] == 'S' {
                resE += 1;
            }
        }
        //Check W
        if pos.1 >= 3 { 
            if matrix[pos.0][pos.1.saturating_sub(1)] == 'M' && matrix[pos.0][pos.1.saturating_sub(2)] == 'A' && matrix[pos.0][pos.1.saturating_sub(3)] == 'S' {
                resW += 1;
            }
        }

        //Check N
        if pos.0 >= 3 {
            if matrix[pos.0.saturating_sub(1)][pos.1] == 'M' && matrix[pos.0.saturating_sub(2)][pos.1] == 'A' && matrix[pos.0.saturating_sub(3)][pos.1] == 'S' {
                resN += 1;
            }
        }

        //Check S
        if pos.0 <= rows -3 {
            if matrix[pos.0 + 1][pos.1] == 'M' && matrix[pos.0 + 2][pos.1] == 'A' && matrix[pos.0 + 3][pos.1] == 'S' {
                resS += 1;
            }
        }

        //Check NE
        if (pos.1 <= cols - 3) && (pos.0 >= 3) {
            if matrix[pos.0.saturating_sub(1)][pos.1 + 1] == 'M' && matrix[pos.0.saturating_sub(2)][pos.1 + 2] == 'A' && matrix[pos.0.saturating_sub(3)][pos.1 + 3] == 'S' {
                resNE += 1;
            }
        }
        //Check NW
        if (pos.0 >= 3) && (pos.1 >= 3){
           if matrix[pos.0.saturating_sub(1)][pos.1.saturating_sub(1)] == 'M' && matrix[pos.0.saturating_sub(2)][pos.1.saturating_sub(2)] == 'A' && matrix[pos.0.saturating_sub(3)][pos.1.saturating_sub(3)] == 'S' {
            resNW += 1;
            }
        }

        //Check SE
        if (pos.1 <= cols - 3) &&  (pos.0 <= rows -3){
            if matrix[pos.0 + 1][pos.1 + 1] == 'M' && matrix[pos.0 + 2][pos.1 + 2] == 'A' && matrix[pos.0 + 3][pos.1 + 3] == 'S' {
                resSE += 1;
            }
        }

        //Check SW
        if (pos.0 <= rows -3) && (pos.1 >= 3) {
            if matrix[pos.0 + 1][pos.1.saturating_sub(1)] == 'M' && matrix[pos.0 + 2][pos.1.saturating_sub(2)] == 'A' && matrix[pos.0 + 3][pos.1.saturating_sub(3)] == 'S' {
                resSW += 1;
            }
        }

    }
    res = resN + resS + resE + resW + resSW + resSE + resNE + resNW;
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
        assert_eq!(result, "18");
    }
}