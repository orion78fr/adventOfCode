extern crate itertools;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

const PUZZLE: &str = include_str!("../input");

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, pt: &Point) -> i32 {
        (self.x - pt.x).abs() + (self.y - pt.y).abs()
    }
}

fn main() {
    let points: Vec<_> = PUZZLE.lines()
        .map(|line| {
            let mut coords = line.split(',');
            Point {
                x: coords.next().unwrap().parse().unwrap(),
                y: coords.next().unwrap().trim().parse().unwrap(),
            }
        })
        .collect();

    let (xmin, xmax, ymin, ymax) = points.iter()
        .fold((std::i32::MAX, std::i32::MIN, std::i32::MAX, std::i32::MIN),
              |(xmin, xmax, ymin, ymax), Point { x, y }| {
                  (
                      if x.lt(&xmin) { *x } else { xmin },
                      if x.gt(&xmax) { *x } else { xmax },
                      if y.lt(&ymin) { *y } else { ymin },
                      if y.gt(&ymax) { *y } else { ymax },
                  )
              });

    let map: HashMap<_, _> = (xmin..xmax).cartesian_product(ymin..ymax)
        .map(|(x, y)| (Point { x, y }, closest_point(&points, Point { x, y })))
        .collect();

    let infinite_claims: HashSet<_> = map.iter()
        .filter(|(Point { x, y }, _)| { *x == xmin || *x == xmax || *y == ymin || *y == ymax })
        .map(|(_, pt)| pt)
        .collect();

    let res = points.iter()
        .filter(|pt| !infinite_claims.contains(pt))
        .map(|pt| map.iter().filter(|(_, p)| ***p == *pt).count())
        .max()
        .unwrap();

    // For my input : 5187
    println!("{}", res)
}

fn closest_point(points: &Vec<Point>, point: Point) -> &Point {
    points.iter()
        .min_by(|point1, point2| {
            point1.manhattan_distance(&point)
                .cmp(&point2.manhattan_distance(&point))
        })
        .unwrap()
}