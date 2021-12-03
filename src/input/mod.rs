pub mod p1_sonar_sweep {
    pub fn data() -> Vec<i32> {
        include_str!("p1_sonar_sweep.txt").split("\n")
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}

pub mod p2_dive {
    pub fn data() -> Vec<(&'static str, i32)> {
        include_str!("p2_dive.txt").split("\n")
            .map(|s| {
                let mut parts = s.split(" ");
                let direction = parts.next().unwrap();
                let amount = parts.next().unwrap().parse::<i32>().unwrap();
                return (direction, amount)
            })
            .collect()
    }
}
