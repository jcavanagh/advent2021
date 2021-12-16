use std::collections::BTreeMap;
use crate::input::p6_lanternfish::fish;

#[derive(Debug)]
struct Lanternfish {
    // age -> number of fish at that age
    fish_timers: BTreeMap<i64, i64>
}

impl Lanternfish {
    pub fn new(fish: Vec<i64>) -> Lanternfish {
        let mut fish_timers: BTreeMap<i64, i64> = BTreeMap::new();
        for i in 0..=8 {
            fish_timers.insert(i, 0);
        }
        for f in fish {
            *fish_timers.entry(f).or_insert(0) += 1;
        }

        Lanternfish { fish_timers }
    }

    pub fn fish(self) -> i64 {
        self.fish_timers.iter().fold(0, |tot, item| {
            tot + item.1
        })
    }

    pub fn advance(&mut self, days: i32) -> &mut Self {
        let mut fish = self.fish_timers.clone();
        println!("Initial: {:?}", fish);
        (0..days).for_each(|_| {
            let new_fish = fish.get(&0).unwrap().clone();
            for i in 1..=8 {
                fish.insert(i-1, fish.get(&i).unwrap().clone());
            }
            *fish.entry(6).or_default() += new_fish;
            fish.insert(8, new_fish);
        });

        self.fish_timers = fish;
        self
    }
}

pub fn main() {
    let mut fish_80 = Lanternfish::new(fish());
    fish_80.advance(80);

    let mut fish_256 = Lanternfish::new(fish());
    fish_256.advance(256);

    println!("Problem 5:");
    println!("Fish after 80 days: {}", fish_80.fish());
    println!("Fish after 256 days: {}", fish_256.fish());
}
