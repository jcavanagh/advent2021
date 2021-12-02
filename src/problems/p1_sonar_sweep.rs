use crate::input::p1_sonar_sweep::data;

fn increases_by_chunk(chunk_size: usize) -> i32 {
    count_increases(data().chunks_exact(chunk_size).collect())
}

fn increases_by_window(window_size: usize) -> i32 {
    count_increases(data().windows(window_size).collect())
}

fn count_increases(slices: Vec<&[i32]>) -> i32 {
    let mut increases = 0;
    let mut slice_iter = slices.iter().peekable();

    if slice_iter.peek().is_none() {
        return 0
    }

    let mut last_sum: i32 = slice_iter.next().unwrap().iter().sum();

    while let Some(slice) = slice_iter.next() {
        let slice_sum = slice.iter().sum();
        if slice_sum > last_sum {
            increases += 1;
        }
        last_sum = slice_sum;
    }

    increases
}

pub fn main() {
    let inc_count_chunk1 = increases_by_chunk(1);
    let inc_count_win3 = increases_by_window(3);

    println!("Problem 1:");
    println!("Number of increases (chunk=1): {}", inc_count_chunk1);
    println!("Number of increases (window=3): {}", inc_count_win3);
}
