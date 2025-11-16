use std::str::{Chars, Lines, SplitN};

use crate::core::{
    cube_surface_area, get_input_data, print_challenge_footer, print_challenge_info,
};

pub(crate) fn print_2015() {
    day_1();
    day_2();
    day_3();
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
    print_challenge_footer();
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
    print_challenge_footer();
}

fn day_3() {
    let input_data: String = get_input_data(2015, 3);

    print_challenge_info(2015, 3);

    print_challenge_footer();
}
