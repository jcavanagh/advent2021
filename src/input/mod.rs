pub mod p1_sonar_sweep {
    pub fn data() -> Vec<i32> {
        include_str!("p1_sonar_sweep.txt")
            .split("\n")
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}

pub mod p2_dive {
    pub fn data() -> Vec<(&'static str, i32)> {
        include_str!("p2_dive.txt")
            .split("\n")
            .map(|s| {
                let mut parts = s.split(" ");
                let direction = parts.next().unwrap();
                let amount = parts.next().unwrap().parse::<i32>().unwrap();
                return (direction, amount)
            })
            .collect()
    }
}

pub mod p3_binary_diagnostic {
    use crate::util::util::transpose_strs;

    const DATA: &str = include_str!("p3_binary_diagnostic.txt");
    pub fn rows() -> Vec<String> {
        DATA
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    pub fn cols() -> Vec<String> {
        transpose_strs(DATA.split("\n").map(|l| l.to_string()).collect())
    }
}

pub mod p4_giant_squid {
    use crate::problems::p4_giant_squid::Board;

    const DATA: &str = include_str!("p4_giant_squid.txt");
    pub fn numbers() -> Vec<i32> {
        DATA
            .split("\n")
            .next().unwrap()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    pub fn boards() -> Vec<Board> {
        let raw_boards: Vec<&str> = DATA
            .split("\n")
            .skip(2)
            .filter(|l| !l.is_empty())
            .collect();

        raw_boards.chunks(5)
            .map(|chunk| {
                let board_i32 = chunk.iter()
                    .flat_map(|c|
                        c.split(" ")
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>()
                    )
                    .collect();

                Board::new(board_i32)
            })
            .collect()
    }
}

pub mod p5_hydrothermal_venture {
    use crate::problems::p5_hydrothermal_venture::Line;

    const DATA: &str = include_str!("p5_hydrothermal_venture.txt");

    pub fn lines() -> Vec<Line> {
        DATA
            .split("\n")
            .map(|l| {
                let mut points = l.split(" -> ");
                let mut p1_raw = points.next().unwrap().split(",");
                let mut p2_raw = points.next().unwrap().split(",");

                Line::new(
                    p1_raw.next().unwrap().parse::<i32>().unwrap(),
                    p1_raw.next().unwrap().parse::<i32>().unwrap(),
                    p2_raw.next().unwrap().parse::<i32>().unwrap(),
                    p2_raw.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect()
    }

    pub fn lines_simple() -> Vec<Line> {
        let mut lines = lines();
        lines.retain(|l|
            l.start.x == l.end.x || l.start.y == l.end.y
        );
        lines
    }
}
