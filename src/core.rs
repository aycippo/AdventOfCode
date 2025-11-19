use std::{fs::File, io::Read};

pub(crate) fn print_challenge_info(year: u16, day: u8) {
    println!("Year: {} / Day: {}", year, day);
}

pub(crate) fn get_input_data(year: u16, day: u8) -> String {
    let filename: String = format!("data/{}/day_{}.txt", year, day);

    let mut file: File = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("Could not open file: {}", err),
    };

    let mut contents: String = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(err) => panic!("Could not read file: {}", err),
    }
}

pub(crate) fn cube_surface_area(length: u32, width: u32, height: u32) -> u32 {
    let a: u32 = length * width;
    let b: u32 = width * height;
    let c: u32 = length * height;

    let smallest_side: u32 = a.min(b).min(c);

    2 * a + 2 * b + 2 * c + smallest_side
}
