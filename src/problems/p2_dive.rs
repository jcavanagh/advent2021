use crate::input::p2_dive::data;

fn final_position(with_aim: bool) -> (i32, i32) {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    let data = data();
    let mut iter = data.iter();

    while let Some(movement) = iter.next() {
        if movement.0 == "forward" {
            horiz += movement.1;
            if with_aim {
                depth += movement.1 * aim
            }
        } else if movement.0 == "up" {
            if with_aim {
                aim -= movement.1;
            } else {
                depth -= movement.1;
            }
        } else if movement.0 == "down" {
            if with_aim {
                aim += movement.1;
            } else {
                depth += movement.1;
            }
        }
    }

    (depth, horiz)
}

pub fn main() {
    let (depth, horiz) = final_position(false);
    let (depth_aimed, horiz_aimed) = final_position(true);

    println!("Problem 2:");
    println!("Final position product (depth={}, horiz={}): {}", depth, horiz, depth * horiz);
    println!("Final position product (aimed) (depth={}, horiz={}): {}", depth_aimed, horiz_aimed, depth_aimed * horiz_aimed);
}
