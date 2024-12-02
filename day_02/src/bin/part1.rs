fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);   
}

fn part1(input: &str) -> String {
    
    let mut res : i32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut level: Vec<i32> = Vec::new();
        let mut safe = true;
        for part in parts {
            level.push(part.parse().unwrap());
        }
        let first_incr = level[1] - level[0];
        if first_incr.abs() <= 3  && first_incr != 0 {
            for i in 1..level.len() - 1 {
                let incr = level[i + 1] - level[i];
                if incr.abs() > 3 || incr * first_incr < 0 || incr == 0 {
                    safe = false;
                    break;
                }
            }
        } else {
            safe = false;
        }
        if safe {
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
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9".to_string();

        let result = part1(input.as_ref());
        assert_eq!(result, "2");
    }
}