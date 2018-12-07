extern crate itertools;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

const PUZZLE: &str = include_str!("../input");

fn main() {
    let points: Vec<(i32, i32)> = PUZZLE.lines()
        .map(|line| {
            let mut coords = line.split(',');
            (coords.next().unwrap().parse().unwrap(),
             coords.next().unwrap().trim().parse().unwrap())
        })
        .collect();

    let (xmin, xmax, ymin, ymax) = points.iter()
        .fold((std::i32::MAX, std::i32::MIN, std::i32::MAX, std::i32::MIN),
              |(xmin, xmax, ymin, ymax), (x, y)| {
                  (
                      if x.lt(&xmin) { *x } else { xmin },
                      if x.gt(&xmax) { *x } else { xmax },
                      if y.lt(&ymin) { *y } else { ymin },
                      if y.gt(&ymax) { *y } else { ymax },
                  )
              });

    let map: HashMap<_, _> = (xmin..xmax).cartesian_product(ymin..ymax)
        .map(|(x, y)| ((x, y), closest_point(&points, (x, y))))
        .collect();

    let infinite_claims: HashSet<_> = map.iter()
        .filter(|((x, y), _)| *x == xmin || *x == xmax || *y == ymin || *y == ymax)
        .map(|(_, pt)| pt)
        .collect();

    println!("{:#?}", infinite_claims)

    //println!("{} {} {} {}", xmin, xmax, ymin, ymax);
}

fn closest_point(points: &Vec<(i32, i32)>, point: (i32, i32)) -> (i32, i32) {
    *points.iter()
        .min_by(|&&point1, &&point2| {
            manhattan_distance(point1, point)
                .cmp(&manhattan_distance(point2, point))
        })
        .unwrap()
}

fn manhattan_distance(pt1: (i32, i32), pt2: (i32, i32)) -> i32 {
    (pt1.0 - pt2.0).abs() + (pt1.1 - pt2.1).abs()
}