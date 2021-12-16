use crate::input::p5_hydrothermal_venture::*;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point
}

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2:i32) -> Line {
        return Line {
            start: Point {
                x: x1,
                y: y1
            },
            end: Point {
                x: x2,
                y: y2
            }
        }
    }

    pub fn x_range(&self) -> Vec<usize> {
        if self.start.x <= self.end.x {
            return (self.start.x..=self.end.x).map(|i| i as usize).collect();
        } else {
            return (self.end.x..=self.start.x).map(|i| i as usize).rev().collect();
        }
    }

    pub fn y_range(&self) -> Vec<usize> {
        if self.start.y <= self.end.y {
            return (self.start.y..=self.end.y).map(|i| i as usize).collect();
        } else {
            return (self.end.y..=self.start.y).map(|i| i as usize).rev().collect();
        }
    }
}

pub struct Plane {
    lines: Vec<Line>
}

pub struct PlaneDimensions {
    x: usize,
    y: usize
}

impl Plane {
    fn all_line_x(&self) -> Vec<i32> {
        self.lines.iter().flat_map(|l| [l.start.x, l.end.x]).collect()
    }

    fn all_line_y(&self) -> Vec<i32> {
        self.lines.iter().flat_map(|l| [l.start.y, l.end.y]).collect()
    }

    pub fn max_x(&self) -> i32 {
        *self.all_line_x().iter().max().unwrap()
    }

    pub fn max_y(&self) -> i32 {
        *self.all_line_y().iter().max().unwrap()
    }

    pub fn dimensions(&self) -> PlaneDimensions {
        PlaneDimensions {
            x: (self.max_x() + 1) as usize,
            y: (self.max_y() + 1) as usize
        }
    }

    pub fn plot(&self) -> Vec<Vec<i32>> {
        let dim = self.dimensions();
        let mut plane: Vec<Vec<i32>> = vec![vec![0i32; dim.x]; dim.y];

        self.lines.iter()
            .for_each(|l| {
                let big_range_is_x = l.x_range().len() >= l.y_range().len();
                let big_range = if big_range_is_x { l.x_range() } else { l.y_range() }.into_iter();
                let mut little_range = if big_range_is_x { l.y_range() } else { l.x_range() }.into_iter();

                let mut last_little = 0usize;
                for big in big_range {
                    let little = little_range.next().unwrap_or(last_little);
                    let x = if big_range_is_x { big } else { little };
                    let y = if big_range_is_x { little } else { big };

                    plane[x][y] = plane[x][y] + 1;
                    last_little = little;
                }
            });

        plane
    }

    pub fn at_least_intersections(&self, threshold: i32) -> i32 {
        self.plot().iter().flatten()
            .filter(|i| *i >= &threshold)
            .collect::<Vec<&i32>>()
            .len() as i32
    }
}

pub fn main() {
    let simple_intersection_points = Plane { lines: lines_simple() }.at_least_intersections(2);
    let all_intersection_points = Plane { lines: lines() }.at_least_intersections(2);

    println!("Problem 5:");
    println!("Intersections (simple,threshold=2): {}", simple_intersection_points);
    println!("Intersections (all,threshold=2): {}", all_intersection_points);
}