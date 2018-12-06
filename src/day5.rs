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

    let filtered_chars: Vec<Vec<&char>> = char_range
        .iter()
        .map(|c: &char| -> Vec<&char> {
            return freshChars.iter().filter(move |&ck| c != ck).collect();
        })
        .collect();

    // let xx: String = filtered_chars[0].iter().map(|c| c.to_string()).collect();
    // println!("{}", xx);

    let counts: Vec<i32> = filtered_chars
        .into_iter()
        .map(|clist| {
            let mut clist_clone = clist.clone();
            let mut grid = clist.clone();
            let mut i = 0;
            let mut current_len = clist_clone.len();

            let mut iteration_count = 1;
            loop {
                let len = clist_clone.len();
                while i < len - 1 {
                    if i >= len {
                        break;
                    }
                    let c = clist_clone[i];
                    let cy = clist_clone[i + 1];

                    if (c.is_uppercase() && cy.is_lowercase())
                        || (c.is_lowercase() && cy.is_uppercase())
                    {
                        if c.to_lowercase().to_string() == cy.to_lowercase().to_string() {
                            grid[i] = &'0';
                            grid[i + 1] = &'0';
                            i += 2;
                            continue;
                        }
                    }
                    i += 1;
                }

                // let xx: String = grid.iter().map(|c| c.to_string()).collect();
                // println!("printing grid {}", xx);

                clist_clone = grid.clone().into_iter().filter(|fc| *fc != &'0').collect();

                grid = clist_clone.clone();
                let new_len = clist_clone.len();
                if current_len == new_len {
                    println!("breaking with iteration count {}", iteration_count);
                    break;
                } else {
                    current_len = new_len;
                    iteration_count += 1;
                }
            }
            return current_len as i32;
        })
        .collect();
    return 10;
}
