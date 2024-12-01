fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);   
}

fn part1(input: &str) -> String {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        v1.push(parts[0].parse().unwrap());
        v2.push(parts[1].parse().unwrap());
    }    

    v1.sort();
    v2.sort();

    let mut res : i32 = 0;
    for (x, y) in v1.iter().zip(v2.iter()) {
        res += (x - y).abs();
    }   
    
    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3".to_string();

        let result = part1(input.as_ref());
        assert_eq!(result, "11");
    }
}