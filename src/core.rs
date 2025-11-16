use std::{fs::File, io::Read};

pub(crate) fn print_challenge_info(year: u16, day: u8) {
    println!("Advent of Code");
    println!("Year: {}", year);
    println!("Day: {}", day);
}

pub(crate) fn print_challenge_footer() {
    println!("\nChallenges completed by Aylah(aycippo)\n");
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
    let a = length * width;
    let b = width * height;
    let c = length * height;

    let smallest_side = a.min(b).min(c);

    2 * a + 2 * b + 2 * c + smallest_side
}
