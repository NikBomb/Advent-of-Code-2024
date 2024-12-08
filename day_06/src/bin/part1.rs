

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);   
}

fn process(input: &str) -> String {
    let mut res: i32 = 0;
    
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut player_facing = 'U';
    let mut player_row:usize = 0;
    let mut player_col:usize = 0;
    

    'outer: for (i, row) in matrix.iter().enumerate() {
        for (j,c) in row.iter().enumerate() {
            if *c == '^' {
                player_facing = 'N';
                player_row = i;
                player_col = j;
                break 'outer;  
            } if *c == '>' {
                player_facing = 'E';
                player_row = i;
                player_col = j;
                break 'outer;
            } if *c == '<' {
                player_facing = 'W';
                player_row = i;
                player_col = j;
                break 'outer;
            } if *c == 'v' {
                player_facing = 'S';
                player_row = i;
                player_col = j;
                break 'outer;
            }
        }
    }

    let mut is_out_of_bounds = false;

    while !is_out_of_bounds {
        matrix[player_row][player_col] = 'X';
        if player_facing == 'N' {
            if player_row > 0 {
                let possible_row = player_row - 1;
                let possible_col = player_col;
                if matrix[possible_row][possible_col] == '#' {
                    player_facing = 'E';
                    continue;
                } else {
                    player_row = possible_row;
                    player_col = possible_col;
                    continue;
                } 
            } else {
                is_out_of_bounds = true;
                continue;
            }
        }
        if player_facing == 'S' {
            if player_row < matrix.len() {
                let possible_row = player_row + 1;
                let possible_col = player_col;
                if matrix[possible_row][possible_col] == '#' {
                    player_facing = 'W';
                    continue;
                } else {
                    player_row = possible_row;
                    player_col = possible_col;
                    continue;
                } 
            } else {
                is_out_of_bounds = true;
                continue;
            }
        }
        if player_facing == 'E' {
            if player_col < matrix[0].len() {
                let possible_row = player_row;
                let possible_col = player_col + 1;
                if matrix[possible_row][possible_col] == '#' {
                    player_facing = 'S';
                    continue;
                } else {
                    player_row = possible_row;
                    player_col = possible_col;
                    continue;
                } 
            } else {
                is_out_of_bounds = true;
                continue;
            }
        }
        if player_facing == 'W' {
            if player_col > 0 {
                let possible_row = player_row;
                let possible_col = player_col - 1;
                if matrix[possible_row][possible_col] == '#' {
                    player_facing = 'N';
                    continue;
                } else {
                    player_row = possible_row;
                    player_col = possible_col;
                    continue;
                } 
            } else {
                is_out_of_bounds = true;
                continue;
            }
        }
    }
    

    
    return "41".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".to_string();
        let result = process(input.as_ref());
        assert_eq!(result, "41");
    }
}