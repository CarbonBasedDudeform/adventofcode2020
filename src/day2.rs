
use crate::common;


pub fn get_ans() {
    let contents = common::read_data("./data/day2.txt".to_string());
    let lines = contents.lines();
    let mut count = 0;
    for line in lines {
        let mut data = line.split(" ");
        let limits_dirty = data.nth(0).unwrap();
        let mut limits = limits_dirty.split("-");
        let low_limit: usize = limits.nth(0).unwrap().parse().expect("NaN"); 
        let hi_limit: usize = limits.nth(0).unwrap().parse().expect("NaN");
        let candidate_dirty = data.nth(0).unwrap();
        let c_len = candidate_dirty.len();
        let candidate = &candidate_dirty[..c_len-1];
        let password = data.nth(0).unwrap();
        //println!("low: {} hi: {} candidate: {} pwd: {}", low_limit, hi_limit, candidate, password);
        //let occurence = password.matches(candidate).count();
        //if (occurence >= low_limit && occurence <= hi_limit) {
        if (password.chars().nth(low_limit-1) == candidate.chars().nth(0) && password.chars().nth(hi_limit-1) != candidate.chars().nth(0) ||
            password.chars().nth(low_limit-1) != candidate.chars().nth(0) && password.chars().nth(hi_limit-1) == candidate.chars().nth(0)) {
            count += 1;
        }
    }

    println!("count: {}", count);
}