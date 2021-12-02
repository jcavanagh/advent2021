pub mod p1_sonar_sweep {
    pub fn data() -> Vec<i32> {
        include_str!("p1_sonar_sweep.txt").split("\n")
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}
