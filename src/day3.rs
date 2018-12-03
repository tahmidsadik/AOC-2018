use std::fs::File;
use std::io::prelude::*;

pub struct Claim {
    pub id: String,
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32,
}

fn read_input_from_file(filename: String) -> String {
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

pub fn get_offset_diemnsion(claim: &str) -> Vec<i32> {
    let s: Vec<&str> = claim.split("@").collect();
    let splitted: Vec<&str> = s[1].split(":").collect();
    let (offset, dimension) = (splitted[0], splitted[1]);

    let off: Vec<&str> = offset.split(",").collect();
    let (offset_x, offset_y) = (off[0].trim(), off[1].trim());

    let dimen: Vec<&str> = dimension.split("x").collect();
    let (width, height) = (dimen[0].trim(), dimen[1].trim());

    return vec![
        offset_x.parse::<i32>().unwrap(),
        offset_y.parse::<i32>().unwrap(),
        width.parse::<i32>().unwrap(),
        height.parse::<i32>().unwrap(),
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

pub fn is_contained_in(a: &Claim, b: &Claim) -> bool {
    let (a_x_start, b_x_start, a_x_end, b_x_end) = (
        a.offset_x + 1,
        b.offset_x + 1,
        a.offset_x + a.width,
        b.offset_x + b.width,
    );
    (b_x_start >= a_x_start && b_x_start <= a_x_start)
        || (b_x_end >= a_x_start && b_x_end <= a_x_end)
}

pub fn does_intersect(a: &Claim, b: &Claim) -> bool {
    is_contained_in(a, b) || is_contained_in(b, a)
}

pub fn common_square(a: &Claim, b: &Claim) -> i32 {
    match does_intersect(a,b) {
        true -> {

        },
        false -> 0
    }
}

// pub fn get_dimension(claim: String) -> HashMap<String, i32> {}
