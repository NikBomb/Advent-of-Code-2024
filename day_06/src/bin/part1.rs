use std::{collections::HashMap, default, vec};

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
    let mut num_row = 0;
    let mut num_col =0;

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

    for (i, row) in matrix.iter().enumerate() {
        for (j,c) in row.iter().enumerate() {
            if i > num_row {
                num_row = i;
            }
            if j > num_col {
                num_col = j;
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
                    let v: Vec<usize> = vec![j];
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
    
    for  (_, value) in obstacles_row.iter_mut() {
        value.sort();
    }

    for  (_, value) in obstacles_col.iter_mut() {
        value.sort();
    }

    
    let mut out_of_bounds: bool = false;
    let mut counter_sim = 0;
    while !out_of_bounds {
        counter_sim += 1 ;
        dbg!("Step : {}", counter_sim);
        for row in matrix.iter_mut() 
        {
            println!("{:?}", row);
        }

        let mut column_barrier: usize = 0;
        let mut row_barrier : usize = 0;

        if player_facing == 'E' {
            let obstacles_at_row = obstacles_row.get(&player_row);
            if obstacles_at_row.is_some() {
                let found_block = obstacles_at_row.unwrap().iter().find(|y| **y > player_col);
                if found_block.is_some(){
                    column_barrier = *(found_block.unwrap()) - 1; 
                } else {
                    column_barrier = num_col;
                }
            } else {
                column_barrier = num_col;
            }
        }
        else if player_facing == 'W' {
            let obstacles_at_row = obstacles_row.get(&player_row);
            if obstacles_at_row.is_some() {
                let found_block = obstacles_at_row.unwrap().iter().rev().find(|y| **y < player_col);
                if found_block.is_some(){
                    column_barrier = *(found_block.unwrap()) + 1; 
                } else {
                    column_barrier = 0;
                }
            } else {
                column_barrier = 0;
            }
        }
        else if player_facing == 'S' {
            let obstacles_at_col = obstacles_col.get(&player_col);
            if obstacles_at_col.is_some() {
                let found_block = obstacles_at_col.unwrap().iter().find(|y| **y > player_row);
                if found_block.is_some(){
                    row_barrier = *(found_block.unwrap()) - 1; 
                } else {
                    row_barrier = num_row;
                }
            } else {
                row_barrier = num_row;
            }
        } 
        else if player_facing == 'N' {
            let obstacles_at_col = obstacles_col.get(&player_col);
            if obstacles_at_col.is_some() {
                let found_block = obstacles_at_col.unwrap().iter().rev().find(|y| **y < player_row);
                if found_block.is_some(){
                    row_barrier = *(found_block.unwrap()) + 1; 
                } else {
                    row_barrier = 0;
                }
            } else {
                row_barrier = 0;
            }
        } 
        
        matrix[player_row][player_col] = 'X';
         match player_facing {
            'E' => { 
                for i in player_col..column_barrier {
                    matrix[player_row][i]= 'X';
                }    
                player_col = column_barrier;
                if player_col <= 0 {
                    out_of_bounds = true;
                } else {
                    player_facing = 'S';
                    matrix[player_row][player_col] = 'v'
                }
            }
            'W' => {
                for i in column_barrier..player_col {
                    matrix[player_row][i]= 'X';
                }
                player_col = column_barrier;
                if player_col >= num_col {
                    out_of_bounds = true;
                } else {
                    player_facing = 'N';
                    matrix[player_row][player_col] = '^'
                }
            }
            'S' => {
                for i in player_row..row_barrier {
                    matrix[i][player_col]= 'X';
                }
                player_row = row_barrier;
                if player_row >= num_row {
                    out_of_bounds = true;
                } else {
                    player_facing = 'W';
                    matrix[player_row][player_col] = '<'
                }
            }
            'N' => {
                for i in row_barrier..player_row {
                    matrix[i][player_col]= 'X';
                }
                player_row = row_barrier;
                if player_row <= 0 {
                    out_of_bounds = true;
                } else {
                    player_facing = 'E';
                    matrix[player_row][player_col] = '>'
                }
            }
            
            _ => {}
        }
        for (row, obst) in &obstacles_row {
            for idx in obst {
                if matrix[*row][*idx] != '#' {
                    dbg!("overwritten obstacles");
                    dbg!("{}, {}", player_facing, counter_sim);
                    dbg!("Row: {} Column: {}, Found {}",*row , *idx, matrix[*row][*idx]);
                    panic!()
                }
            }
        }
    }
    
    matrix[player_row][player_col] = 'X';
    dbg!("The End");
        for row in matrix.iter_mut() 
        {
            println!("{:?}", row);
        }
        
    for (_ , row) in matrix.iter().enumerate() {
        for (_,c) in row.iter().enumerate() {
            if *c == 'X'{
                res += 1;
            }
        }
    }
  
    return res.to_string();
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