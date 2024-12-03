
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);   
}

fn process_partial(input: &str) -> i32 {
    let mut res : i32 = 0;
    let pattern = r"mul\(\d+,\d+\)";
    let re = Regex::new(pattern).unwrap();

    let matches: Vec<&str> = re.find_iter(input).map(|mat| mat.as_str()).collect();

    for _match in matches {
        let nums: Vec<&str> = _match.split(",").collect();
        let num1: i32 = nums[0].replace("mul(", "").parse().unwrap();
        let num2: i32 = nums[1].replace(")", "").parse().unwrap();
        res += num1 * num2;
    }

    return res;
}

fn process(input: &str) -> String {
    
    let mut res : i32 = 0;

    let mut curr_start = 0;
    let mut curr_end = input.len();
    
    let mut curr_token: &str = "do()";
    let mut barrier_token = "don't()";

    while curr_start < input.len() {
        if curr_token == "do()" {
            barrier_token = "don't()";
        } else {
            barrier_token = "do()";
        }
        curr_end = input[curr_start..input.len()].find(barrier_token).unwrap_or(input[curr_start..input.len()].len()) + input[0..curr_start].len();
        let to_process_now = &input[curr_start..curr_end];
        if curr_token == "do()" {
            res += process_partial(to_process_now);
        }
        curr_token = barrier_token;
        curr_start = curr_end;
    }

    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
".to_string();
        let result = process(input.as_ref());
        assert_eq!(result, "48");
    }
}