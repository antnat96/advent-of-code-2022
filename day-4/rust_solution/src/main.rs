// Started here: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut part_one: i32 = 0;
        let mut part_two: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let arr: Vec<&str> = ip.split(",").collect();
                let first: &str = arr[0];
                let first_range_str: Vec<&str> = first.split("-").collect();
                let first_range_start_int: i32 = first_range_str[0].parse::<i32>().unwrap();
                let first_range_end_int: i32 = first_range_str[1].parse::<i32>().unwrap();

                let second: &str = arr[1];
                let second_range_str: Vec<&str> = second.split("-").collect();
                let second_range_start_int: i32 = second_range_str[0].parse::<i32>().unwrap();
                let second_range_end_int: i32 = second_range_str[1].parse::<i32>().unwrap();

                // Part 1 (check for complete overlap)
                if first_range_start_int >= second_range_start_int && first_range_end_int <= second_range_end_int {
                    part_one += 1;
                } else if second_range_start_int >= first_range_start_int && second_range_end_int <= first_range_end_int {
                    part_one += 1;
                }

                // Part 2 (check for any overlap)
                if (first_range_start_int <= second_range_end_int) && (first_range_end_int >= second_range_start_int) {
                    part_two += 1;
                }


            }
        }
        println!("Part 1 Result: {}", part_one);
        println!("Part 2 Result: {}", part_two);
    }
}
