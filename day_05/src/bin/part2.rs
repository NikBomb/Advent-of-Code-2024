fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);   
}

fn process(input: &str) -> String {
    
    let lines: Vec<&str> = input.lines().collect();
    let mut rules: Vec<(i32, i32)>  = Vec::new();
    let mut volumes : Vec<Vec<i32>> = Vec::new(); 
    let mut violating_volumes: Vec<Vec<i32>> = Vec::new();



    for line in lines {
        if line.contains("|") {
            let pair:Vec<&str> = line.split("|").collect();
            rules.push((pair[0].parse().unwrap(), pair[1].parse().unwrap()));
        }
        if line.contains(",") {
            let vls: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            volumes.push(vls);   
        }
    }

    let mut violating_index : Vec<bool> = vec![false; volumes.len()];
    

    for rule in &rules {
        // Find which volumes violate at least a rule 
        for i in 0..volumes.len() {
            let index0: Option<usize> = volumes[i].iter().position(|&x| x == rule.0);
            let index1: Option<usize> = volumes[i].iter().position(|&x| x == rule.1);
            if index0.is_some() && index1.is_some() {
                if index0.unwrap() > index1.unwrap() {
                    violating_index[i] = true;
                }            
            }
        }
    }

    for i in 0..volumes.len() {
        if violating_index[i] {
            violating_volumes.push(volumes[i].clone());
        }
    }
    for volume in &mut violating_volumes {
        let mut is_violating = true;
        while is_violating {
            is_violating = false;
            for rule in &rules {
                let index0: Option<usize> = volume.iter().position(|&x| x == rule.0);
                let index1: Option<usize> = volume.iter().position(|&x| x == rule.1);
                if index0.is_some() && index1.is_some() {
                    if index0.unwrap() > index1.unwrap() {
                        is_violating = true;
                        volume.swap(index0.unwrap(), index1.unwrap());
                    }
                }
            }
        }
    }

    let mut res = 0;

    for volume in violating_volumes {
        let middle = volume[volume.len() / 2];
        res += middle
    }

    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();
        let result = process(input.as_ref());
        assert_eq!(result, "123");
    }
}