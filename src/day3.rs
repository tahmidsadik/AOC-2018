use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;

pub struct Claim {
    pub id: String,
    pub offset_x: usize,
    pub offset_y: usize,
    pub width: usize,
    pub height: usize,
}

fn read_input_from_file(filename: &str) -> String {
    let mut file_contents = String::new();
    let mut f = File::open(filename).expect("Couldn't open file for reading");
    f.read_to_string(&mut file_contents)
        .expect("Something went wrong while reading the file");

    return file_contents.trim().to_string();
}

pub fn get_id(claim: &str) -> String {
    let splitted: Vec<&str> = claim.split("@").collect();
    let len = splitted[0].len();
    return ((&splitted[0])[1..len]).to_string();
}

pub fn get_offset_diemnsion(claim: &str) -> Vec<usize> {
    let s: Vec<&str> = claim.split("@").collect();
    let splitted: Vec<&str> = s[1].split(":").collect();
    let (offset, dimension) = (splitted[0], splitted[1]);

    let off: Vec<&str> = offset.split(",").collect();
    let (offset_x, offset_y) = (off[0].trim(), off[1].trim());

    let dimen: Vec<&str> = dimension.split("x").collect();
    let (width, height) = (dimen[0].trim(), dimen[1].trim());

    return vec![
        offset_x.parse::<usize>().unwrap(),
        offset_y.parse::<usize>().unwrap(),
        width.parse::<usize>().unwrap(),
        height.parse::<usize>().unwrap(),
    ];
}

pub fn extract_data(claim: String) -> Claim {
    let id = get_id(claim.as_ref());
    let off_d = get_offset_diemnsion(claim.as_ref());
    let offset_x = off_d[0];
    let offset_y = off_d[1];
    let width = off_d[2];
    let height = off_d[3];

    return Claim {
        id,
        offset_x,
        offset_y,
        width,
        height,
    };
}

pub fn solve_problem_1(input: &str) -> i32 {
    let inputs: Vec<Claim> = read_input_from_file(input)
        .split("\n")
        .map(|claim| extract_data(claim.to_string()))
        .collect();
    let (len, mut sum) = (inputs.len(), 0);
    let mut area: [[i32; 1001]; 1001] = [[0; 1001]; 1001];
    for i in 0..len {
        let cc = &inputs[i];

        for j in cc.offset_x + 1..=cc.width + cc.offset_x {
            for k in cc.offset_y + 1..=cc.height + cc.offset_y {
                area[j][k] = area[j][k] + 1;
            }
        }
    }

    for i in 0..area.len() {
        for j in 0..area[i].len() {
            if area[i][j] > 1 {
                sum += 1;
            }
        }
    }

    return sum;
}

fn does_overlap(grid: &[[i32; 1001]; 1001], c: &Claim) -> bool {
    let mut overlapped = false;

    for i in c.offset_x + 1..=c.width + c.offset_x {
        for j in c.offset_y + 1..=c.height + c.offset_y {
            if grid[i][j] == 1 {
                continue;
            } else {
                overlapped = true;
                return overlapped;
            }
        }
    }

    return overlapped;
}

pub fn solve_problem_2(input: &str) -> String {
    let inputs: Vec<Claim> = read_input_from_file(input)
        .split("\n")
        .map(|claim| extract_data(claim.to_string()))
        .collect();
    let (len, mut sum) = (inputs.len(), 0);
    let mut area: [[i32; 1001]; 1001] = [[0; 1001]; 1001];
    for i in 0..len {
        let cc = &inputs[i];

        for j in cc.offset_x + 1..=cc.width + cc.offset_x {
            for k in cc.offset_y + 1..=cc.height + cc.offset_y {
                area[j][k] = area[j][k] + 1;
            }
        }
    }

    for i in 0..len {
        let cc = &inputs[i];
        if !does_overlap(&area, cc) {
            return cc.id.to_string();
        }
    }
    String::from("Not Found")
}

// pub fn get_dimension(claim: String) -> HashMap<String, i32> {}
