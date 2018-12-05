use std::fs::File;
use std::io::prelude::*;

fn read_input_from_file(filename: &str) -> String {
    let mut file_contents = String::new();
    let mut f = File::open(filename).expect("Couldn't open file for reading");
    f.read_to_string(&mut file_contents)
        .expect("Something went wrong while reading the file");

    return file_contents.trim().to_string();
    // return "Hello".to_string();
}

pub fn solve_problem_1(filename: &str) -> usize {
    let polymer = read_input_from_file(filename);

    let mut chars: Vec<char> = polymer.chars().collect();
    let mut charsClone = chars.clone();

    for j in 0..500 {
        let mut i = 0;
        while i < chars.len() - 1 {
            if (i >= chars.len()) {
                break;
            }
            let c = chars[i];
            let cy = chars[i + 1];

            if (c.is_uppercase() && cy.is_lowercase()) || (c.is_lowercase() && cy.is_uppercase()) {
                if c.to_lowercase().to_string() == cy.to_lowercase().to_string() {
                    charsClone[i] = '0';
                    charsClone[i + 1] = '0';
                    i += 2;
                    continue;
                }
            }
            i += 1;
        }

        //
        chars = charsClone
            .clone()
            .into_iter()
            .filter(|c| *c != '0')
            .collect();

        charsClone = chars.clone();
    }

    return chars.len();
}

pub fn solve_problem_2(filename: &str) -> i32 {
    let polymer = read_input_from_file(filename);

    let mut chars: Vec<char> = polymer.chars().collect();
    let freshChars = chars.clone();
    let mut charsClone = chars.clone();
    let char_range = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let filtered_chars: Vec<Vec<char>> = char_range
        .iter()
        .map(move |c: char| -> Vec<char> {
            return freshChars.iter().filter(move |ck| c != ck).collect();
        })

    let counts: Vec<i32> = filtered_chars
        .into_iter()
        .map(|char_list| {
            let mut char_list_clone = char_list.clone();
            let mut i = 0;
            while i < char_list.len() {
                if i >= char_list.len() {
                    break;
                }
                let c = char_list[i];
                let cy = char_list[i + 1];

                if (c.is_uppercase() && cy.is_lowercase())
                    || (c.is_lowercase() && cy.is_uppercase())
                {
                    if c.to_lowercase().to_string() == cy.to_lowercase().to_string() {
                        char_list_clone[i] = &'0';
                        char_list_clone[i + 1] = &'0';
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }
            return 0;
        })
        .collect();

    let mut current_len = 50000;

    let mut counts: Vec<i32> = Vec::new();

    for character in char_range {
        chars = freshChars
            .clone()
            .into_iter()
            .filter(|c| *c != character && *c.to_string() != character.to_uppercase().to_string())
            .collect();

        current_len = chars.len();

        for j in 0..1000 {
            println!("count = {}, chars len {}", current_len, chars.len());
            let mut i = 0;
            while i < chars.len() - 1 {
                println!("index = {}, chars len {}", i, chars.len());

                if (i >= chars.len()) {
                    break;
                }
                let c = chars[i];
                let cy = chars[i + 1];

                if (c.is_uppercase() && cy.is_lowercase())
                    || (c.is_lowercase() && cy.is_uppercase())
                {
                    if c.to_lowercase().to_string() == cy.to_lowercase().to_string() {
                        charsClone[i] = '0';
                        charsClone[i + 1] = '0';
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }

            //
            chars = charsClone
                .clone()
                .into_iter()
                .filter(|c| *c != '0')
                .collect();

            charsClone = chars.clone();
            if current_len == chars.len() {
                break;
            } else {
                current_len = chars.len();
            }
        }

        counts.push(current_len as i32);
    }

    return counts
        .iter()
        .fold(50000, |acc, n| if n < &acc { *n } else { acc });
}
