use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_input_from_file(filename: String) -> String {
    let mut file_contents = String::new();
    let mut f = File::open(filename).expect("Couldn't open file for reading");
    f.read_to_string(&mut file_contents)
        .expect("Something went wrong while reading the file");

    return file_contents.trim().to_string();
}

// takes two strings and checks their similarity
// if the two strings differ in only and exactly in one index returns that index
// else returns -1
//
// example:
// difer_by("abxcd", "abycd") will return 2
// difer_by("abxcd", "abxcd") will return -1
// difer_by("abxcd", "abxyd") will return -1
//

pub fn differ_by(s1: String, s2: String) -> i32 {
    let mut differ_count = 0;
    let mut differ_index: i32 = -1;
    if s1 == s2 {
        return -1;
    }

    if s1.len() != s2.len() {
        return -1;
    }

    let chars1: Vec<&str> = s1.split("").collect();
    let chars2: Vec<&str> = s2.split("").collect();

    for i in 0..chars1.len() {
        if differ_count > 1 {
            return -1;
        }

        if chars1[i] != chars2[i] {
            differ_count += 1;
            differ_index = i as i32;
            print!("\n");
            println!("i = {} and idx = {}", i, differ_index);
        }
    }

    if differ_count == 1 {
        return differ_index;
    }

    return -1;
}

// remove a single character from a String at the given idx
fn remove_char_at(s: String, idx: usize) -> String {
    match idx {
        x if idx > s.len() => panic!("Index is greater than the size of the string"),
        x if idx == s.len() => s[0..s.len() - 1].to_owned(),
        0 => s[1..s.len()].to_owned(),
        n => {
            let mut new_string = s[0..n].to_owned();
            new_string.push_str(&s[n + 1..s.len()]);
            return new_string;
        }
    }
}

pub fn day2_problem1(input_file: String) -> i32 {
    let input_data = read_input_from_file(input_file);

    let inputs: Vec<&str> = input_data.split('\n').collect();
    let mut checksum_count_two = 0;
    let mut checksum_count_three = 0;

    for s in &inputs {
        let mut hm: HashMap<&str, i32> = HashMap::new();
        let chars: Vec<&str> = s.split("").collect();

        for c in chars {
            match hm.get(&c) {
                Some(val) => hm.insert(c, val + 1),
                None => hm.insert(c, 1),
            };
        }

        if hm.values().any(|&v| v == 2) {
            checksum_count_two += 1;
            println!("Checksum 2 = {}", checksum_count_two);
        }
        if hm.values().any(|&v| v == 3) {
            checksum_count_three += 1;
            println!("Checksum 3 = {}", checksum_count_three);
        }
    }
    return checksum_count_three * checksum_count_two;
}

pub fn day2_problem2(input_file: String) -> String {
    let input_data = read_input_from_file(input_file);
    let inputs: Vec<String> = input_data.split('\n').map(|s| s.to_string()).collect();

    let hashMap: HashMap<String, i32> = HashMap::new();

    for i in 0..inputs.len() {
        for j in 0..inputs.len() {
            let idx = differ_by(inputs[i].to_owned(), inputs[j].to_owned());
            match idx {
                -1 => continue,
                n => {
                    println!("{}\n{}", inputs[i], inputs[j]);
                    println!("index {}", n);
                    println!(
                        "index removed {}",
                        remove_char_at(inputs[i].to_owned(), n as usize)
                    );
                    return remove_char_at(inputs[i].to_owned(), (n - 1) as usize);
                }
            }
        }
    }
    return String::from("Hello");
}
