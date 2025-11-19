use std::{
    collections::HashSet,
    str::{Chars, Lines, SplitN},
};

use hex_literal::hex;
use md5::{Digest, Md5};

use crate::core::{cube_surface_area, get_input_data, print_challenge_info};

pub(crate) fn print_2015() {
    day_1();
    day_2();
    day_3();
    day_4();
}

fn day_1() {
    let input_data: String = get_input_data(2015, 1);
    let mut floor: i128 = 0;
    let mut steps_to_basement: i128 = 0;
    let mut found_basement: bool = false;
    let route: Chars<'_> = input_data.chars();

    route.for_each(|step: char| {
        if floor == -1 {
            found_basement = true;
        }

        if found_basement == false {
            steps_to_basement += 1;
        }

        if step == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    });

    print_challenge_info(2015, 1);
    println!("\tFinal Floor: {}", floor);
    println!("\tSteps to Basement: {}", steps_to_basement);
}

fn day_2() {
    let input_data: String = get_input_data(2015, 2);
    let boxes: Lines<'_> = input_data.lines();
    let mut total_area: u32 = 0;
    let mut total_ribbon: u32 = 0;

    boxes.for_each(|line: &str| {
        let mut parts: SplitN<'_, char> = line.splitn(3, 'x');

        let length: u32 = parts.next().unwrap().to_string().parse::<u32>().unwrap();
        let width: u32 = parts.next().unwrap().to_string().parse::<u32>().unwrap();
        let height: u32 = parts.next().unwrap().to_string().parse::<u32>().unwrap();
        let area: u32 = cube_surface_area(length, width, height);

        total_area += area;

        let min_side: u32 = length.min(width).min(height);
        let max_side: u32 = length.max(width).max(height);
        let second_smallest: u32 = length + width + height - min_side - max_side;

        let box_ribbon: u32 = 2 * min_side + 2 * second_smallest;
        let bow_ribbon: u32 = length * width * height;

        total_ribbon += box_ribbon + bow_ribbon;
    });

    print_challenge_info(2015, 2);
    println!("\tSurface Area of All Presents: {}", total_area);
    println!("\tTotal Feet of Ribbon: {}", total_ribbon);
}

fn day_3() {
    let input_data: String = get_input_data(2015, 3);
    let directions: Chars<'_> = input_data.chars();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut duo_x: i32 = 0;
    let mut duo_y: i32 = 0;
    let mut robo_x: i32 = 0;
    let mut robo_y: i32 = 0;

    let mut robo_turn: bool = false;

    let mut solo_coordinate_set: HashSet<(i32, i32)> = HashSet::new();
    let mut duo_coordinate_set: HashSet<(i32, i32)> = HashSet::new();

    directions.for_each(|step: char| {
        match step {
            '^' => y += 1,
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            _ => println!("Invalid char input \"{}\"", step),
        }

        solo_coordinate_set.insert((x, y));

        if robo_turn {
            match step {
                '^' => robo_y += 1,
                '>' => robo_x += 1,
                'v' => robo_y -= 1,
                '<' => robo_x -= 1,
                _ => println!("Invalid char input \"{}\"", step),
            }

            robo_turn = false;

            duo_coordinate_set.insert((robo_x, robo_y));
        } else {
            match step {
                '^' => duo_y += 1,
                '>' => duo_x += 1,
                'v' => duo_y -= 1,
                '<' => duo_x -= 1,
                _ => println!("Invalid char input \"{}\"", step),
            }

            robo_turn = true;

            duo_coordinate_set.insert((duo_x, duo_y));
        }
    });

    let solo_houses_visited: usize = solo_coordinate_set.len();
    let duo_houses_visited: usize = duo_coordinate_set.len();

    print_challenge_info(2015, 3);
    println!("\tHouses Visited Solo: {}", solo_houses_visited);
    println!("\tHouses Visited w/ Robo Santa: {}", duo_houses_visited);
}

fn day_4() {
    let mut hasher = Md5::new();

    let secret: &str = "pqrstuv";

    let number: u16 = 0;
    let number_string: String = number.to_string();

    let combo: String = format!("{}{}", secret, number_string);
    let combo_bytes: &[u8] = combo.as_bytes();

    hasher.update(b"pqrstuv1048970");

    let result = hasher.finalize();

    // let result = hasher.finalize().starts_with(&[0, 0]);

    println!("{:?}", result);
    println!("{:?}", hex!("abcdef609043"))
}
