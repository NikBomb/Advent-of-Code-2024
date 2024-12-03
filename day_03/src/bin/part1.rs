
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);   
}

fn process(input: &str) -> String {
    
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

    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let result = process(input.as_ref());
        assert_eq!(result, "161");
    }
}