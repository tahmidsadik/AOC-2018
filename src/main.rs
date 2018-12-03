use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod day2;

fn read_input_from_file(filename: String) -> String {
    let mut file_contents = String::new();
    let mut f = File::open(filename).expect("Couldn't open file for reading");
    f.read_to_string(&mut file_contents)
        .expect("Something went wrong while reading the file");

    return file_contents.trim().to_string();
}

fn p1(input: String) -> i32 {
    let input_data = read_input_from_file(input);

    let sum = input_data
        .split('\n')
        .map(|val| val.parse::<i32>().unwrap())
        .fold(0, |acc, val| acc + val);
    return sum;
}

fn p2(input: String) -> i32 {
    let input_data = read_input_from_file(input);
    let mut sum = 0;
    let frequencies: Vec<&str> = input_data.split('\n').collect();
    let mut freq_hm: HashMap<i32, i32> = HashMap::new();

    loop {
        let mut iteration_count = 1;
        for s in &frequencies {
            let (sign, number) = s.split_at(1);
            let n = number.parse::<i32>().unwrap();

            match sign.as_ref() {
                "+" => sum += n,
                "-" => sum -= n,
                _ => panic!("Bad Input"),
            }

            if freq_hm.contains_key(&sum) {
                println!("Key Found, Key = {}", sum);
                return sum;
            }
            freq_hm.insert(sum, 0);
        }
        iteration_count += 1;
        if iteration_count > 100 {
            break;
        }
    }
    return -1;
}

fn main() {
    let filename = "puzzle1-input.txt";
    let puzzle_input_2 = "puzzle-input-2.txt";
    // println!("Solution to problem 1 = {}", p1(String::from(filename)));
    // println!("Solution to problem 2 = {}", p2(String::from(filename)));
    // println!(
    //     "Solution to problem 3 = {}",
    //     day2::day2_problem1(String::from(puzzle_input_2))
    // );
    // println!(
    //     "Solution to problem 3 = {}",
    //     day2::day2_problem2(String::from(puzzle_input_2))
    // );
    day2::day2_problem2(String::from(puzzle_input_2));
}
