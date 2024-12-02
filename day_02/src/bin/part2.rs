fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);   
}

fn is_safe(level: &Vec<i32>) -> Vec<i32> {
    
    let mut res : Vec<i32> = Vec::new();


    let first_incr = level[1] - level[0];
    if first_incr.abs() <= 3  && first_incr != 0 {
        for i in 1..level.len() - 1 {
            let incr = level[i + 1] - level[i];
            if incr.abs() > 3 || incr * first_incr < 0 || incr == 0 {
                res.push(i as i32 - 1);
                res.push(i as i32);
                res.push(i as i32 + 1);
            }
        }
    } else {
        res.push(0);
        res.push(1);
    }

    return res;
}


fn part1(input: &str) -> String {
    
    let mut res : i32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut level: Vec<i32> = Vec::new();
        for part in parts {
            level.push(part.parse().unwrap());
        }
        let wrong_parts: Vec<i32> = is_safe(&level);
        if wrong_parts.len() == 0 {
            res += 1;
        } else {
            for wrong_part in wrong_parts {
                let mut new_level = level.clone();
                new_level.remove(wrong_part as usize);
                let wrong_parts2: Vec<i32> = is_safe(&new_level);
                if wrong_parts2.len() == 0 {
                    res += 1;
                    break;
                }
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
        let input = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20".to_string();

        let result = part1(input.as_ref());
        assert_eq!(result, "10");
    }
}