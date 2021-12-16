use crate::input::p7_the_treachery_of_whales::data;

fn avg(positions: &Vec<i32>) -> i32 {
    positions.iter().sum::<i32>() / positions.len() as i32
}

fn median(positions: &Vec<i32>) -> i32 {
    let mut sorted = positions.to_owned();
    sorted.sort();
    sorted[positions.len() / 2].clone()
}

fn least_fuel_alignment(positions: Vec<i32>) -> i64 {
    let pos_median = median(&positions);

    positions.iter().fold(0, |tot, item| {
        tot + (pos_median - item).abs() as i64
    })
}

fn least_fuel_alignment_increasing(positions: Vec<i32>) -> i64 {
    let pos_avg = avg(&positions);

    positions.iter().fold(0, |tot, item| {
        let distance = (pos_avg - item).abs() as i64;
        let fuel_cost = (distance * (distance + 1)) / 2;
        tot + fuel_cost
    })
}

pub fn main() {
    let least_fuel_simple = least_fuel_alignment(data());
    let least_fuel_increasing = least_fuel_alignment_increasing(data());

    println!("Problem 7:");
    println!("Least fuel: {}", least_fuel_simple);
    println!("Least fuel (increasing): {}", least_fuel_increasing);
}
