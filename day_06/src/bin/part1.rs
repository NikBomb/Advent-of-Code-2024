use std::{collections::HashMap, vec};



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

    // Parse obstacles

    // These are maps where the key is either the row/ column, and for every of those you have the sorted obstacles 
    // w.r.t the other dimension  
    let mut obstacles_row: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut obstacles_col: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i , row) in matrix.iter().enumerate() {
        for (j,c) in row.iter().enumerate() {
            if *c == '#' {
                if obstacles_row.contains_key(&i) {
                    obstacles_row.get_mut(&i).unwrap().push(j);
                } else {
                    let mut v: Vec<usize> = vec![j];
                    obstacles_row.insert(i, v);
                }

                if obstacles_col.contains_key(&j) {
                    obstacles_col.get_mut(&j).unwrap().push(i);
                } else {
                    let mut v: Vec<usize> = vec![i];
                    obstacles_col.insert(j, v);
                }

            }
        }
    }
    
    for  (_, value) in &mut obstacles_row {
        value.sort();
    }

    for  (_, value) in &mut obstacles_col {
        value.sort();
    }

    let mut is_out_of_bounds = false;


    
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