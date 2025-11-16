use std::str::Chars;

use crate::core::{
    cube_surface_area, get_input_data, print_challenge_footer, print_challenge_info,
};

pub(crate) fn print_2015() {
    day_1();
    day_2();
}

fn day_1() {
    let floorplan: String = get_input_data(2015, 1);
    let mut floor: i128 = 0;
    let mut steps_to_basement: i128 = 0;
    let mut found_basement: bool = false;
    let route: Chars<'_> = floorplan.chars();

    route.for_each(|step| {
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
    let binding = get_input_data(2015, 2);
    let boxes = binding.lines();
    let mut total_area = 0;

    boxes.for_each(|line| {
        let mut parts = line.splitn(3, 'x');

        let length: u32 = parts.next().unwrap().to_string().parse::<u32>().unwrap();
        let width: u32 = parts.next().unwrap().to_string().parse::<u32>().unwrap();
        let height: u32 = parts.next().unwrap().to_string().parse::<u32>().unwrap();
        let area: u32 = cube_surface_area(length, width, height);

        total_area += area;
    });

    print_challenge_info(2015, 2);
    println!("\tSurface Area of All Presents: {}", total_area);
    print_challenge_footer();
}
