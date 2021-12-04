use crate::input::p3_binary_diagnostic::*;
use crate::util::util::transpose_strs;

fn most_common_bits(cols: Vec<String>, tiebreaker: i32) -> Vec<i32> {
    // Count all the ones in each column and compare to the length to find out if the ones are more numerous
    cols.iter()
        .map(|c| {
            let ones = c.chars().fold(0, |tot, c| {
                if c == '1' {
                    tot + 1
                } else {
                    tot
                }
            });

            if ones == c.len() / 2 {
                return tiebreaker
            }

            if ones > c.len() / 2 {
                1
            } else {
                0
            }
        })
        .collect()
}

fn least_common_bits(cols: Vec<String>, tiebreaker: i32) -> Vec<i32> {
    let mcbs = most_common_bits(cols, tiebreaker);
    mcbs.iter().map(|b| if *b == 1 { 0 } else { 1 }).collect()
}

fn mcbs_str_to_i32(mcbs: String) -> i32 {
    mcbs_to_i32(
        mcbs.chars()
            .map(|b| b.to_digit(10).unwrap() as i32)
            .collect()
    )
}

fn mcbs_to_i32(mcbs: Vec<i32>) -> i32 {
    mcbs.iter().fold(0, |tot, &b| (tot << 1) ^ b)
}

fn power_consumption() -> (i32, i32) {
    let most_common_bits = most_common_bits(cols(), 1);
    let len = most_common_bits.len() as u32;

    // Set each bit where ones were the most common
    let epsilon_rate = mcbs_to_i32(most_common_bits);

    // Flip all the bits we previously set
    let gamma_rate = epsilon_rate ^ (2_i32.pow(len) - 1);

    (epsilon_rate, gamma_rate)
}

fn life_support_rating() -> (i32, i32) {
    let mut oxygen = rows();
    for i in 0..12 {
        let mcb_in_col = most_common_bits(
            transpose_strs(oxygen.clone()), 1
        )[i].to_string();

        oxygen.retain(|c| {
            c.chars().nth(i).unwrap().to_string().eq(&mcb_in_col)
        });

        if oxygen.len() <= 1 {
            break
        }
    }

    let mut co2 = rows();
    for i in 0..12 {
        let lcb_in_col = least_common_bits(
            transpose_strs(co2.clone()), 1
        )[i].to_string();

        co2.retain(|c| {
            c.chars().nth(i).unwrap().to_string().eq(&lcb_in_col)
        });

        if co2.len() <= 1 {
            break
        }
    }

    let oxygen_rating = mcbs_str_to_i32(oxygen.first().unwrap().to_string());
    let co2_rating = mcbs_str_to_i32(co2.first().unwrap().to_string());

    (oxygen_rating, co2_rating)
}

pub fn main() {
    let (epsilon_rate, gamma_rate) = power_consumption();
    let (oxygen, co2) = life_support_rating();

    println!("Problem 3:");
    println!("Power Consumption (gamma={}, epsilon={}): {}", epsilon_rate, gamma_rate, epsilon_rate * gamma_rate);
    println!("Life Support Rating (oxygen={}, CO2={}): {}", oxygen, co2, oxygen * co2);
}
