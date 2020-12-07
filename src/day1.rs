use std::fs;

fn read_data() -> String {
    let contents = fs::read_to_string("./data/day1.txt")
        .expect("Something went wrong reading the file");

    
    return contents;
}

fn find_two() {
    let contents = read_data();
    let as_lines : Vec<_> = contents.lines().collect();
    let count = contents.lines().count();
    for num in 0..count {
        let num_i: i32 = as_lines[num].parse().unwrap();
        for other_num in 1..count {
            let o_num_i: i32 = as_lines[other_num].parse().unwrap();
            if num_i + o_num_i == 2020 {
                println!("{} + {} == 2020", num_i, o_num_i);
                println!("ans: {}", num_i * o_num_i);
            }
        }
        
    }
}

fn find_three() {
    let contents = read_data();
    let as_lines : Vec<_> = contents.lines().collect();
    let count = contents.lines().count();
    for num in 0..count {
        let num_i: i32 = as_lines[num].parse().unwrap();
        for other_num in 1..count {
            let o_num_i: i32 = as_lines[other_num].parse().unwrap();
            for other_other_num in 2..count {
                let o_o_num_i: i32 = as_lines[other_other_num].parse().unwrap();
                if num_i + o_num_i + o_o_num_i == 2020 {
                    println!("{} + {} + {} == 2020", num_i, o_num_i, o_o_num_i);
                    println!("ans: {}", num_i * o_num_i * o_o_num_i);
                }    
            }
        }
        
    }
}

pub fn test() {
    find_three();
}