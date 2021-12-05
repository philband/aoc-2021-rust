use std::collections::HashMap;
use itertools::Itertools;


pub type Point = (i32, i32);
pub type Line = (Point, Point);

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let mut line = l.split(" -> ").map(|points| {
                let mut ps = points.split(",").map(|p| p.parse::<i32>().unwrap());
                (ps.next().unwrap(), ps.next().unwrap())
            });
            (line.next().unwrap(), line.next().unwrap())
        }).collect()
}

#[aoc(day5, part1)]
pub fn part1(inputs: &[Line]) -> usize {
    let mut map: HashMap<Point, i32> = HashMap::new();

    for l in inputs {
        let &((x1, y1), (x2, y2)) = l;
        // skip diagonals
        if ! (x1 == x2 || y1 == y2) {
            continue;
        }
        // swap order for ranges to work
        let (xs, xe) = match x1.cmp(&x2) {
            std::cmp::Ordering::Greater => (x2, x1),
            _ => (x1, x2)
        };
        let (ys, ye) = match y1.cmp(&y2) {
            std::cmp::Ordering::Greater => (y2, y1),
            _ => (y1, y2)
        };
        // iterate over combinations
        for x in xs..=xe {
            for y in ys..=ye {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    map.into_iter().filter(|(_, v)| v >= &2).count()
}

#[aoc(day5, part1, iter)]
pub fn part1_iter(inputs: &[Line]) -> usize {
    inputs
        .iter()
        // filter diagonals
        .filter(|&((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .map(|l| {
            let &((x1, y1), (x2, y2)) = l;
            // fix ranges for iter
            (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2))
        })
        .fold(HashMap::<Point, i32>::new(), |mut map, (x1, y1, x2, y2)| {
            // combine iterators
            (x1..=x2).cartesian_product(y1..=y2).for_each(|(x, y)| *map.entry((x, y)).or_insert(0) += 1);
            map
        })
        .iter()
        .filter(|(_, &v)| v >= 2).count()
}

#[aoc(day5, part1, iter2)]
pub fn part1_iter2(inputs: &[Line]) -> usize {
    inputs
        .iter()
        // filter out diagonals
        .filter(|&((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .fold(HashMap::<Point, i32>::new(), |mut map, &((x1, y1), (x2, y2))| {
            let (xd, yd) = ((x2-x1).signum(), (y2-y1).signum());
            // number of step into_iter
            (0..=((x2-x1).abs().max((y2-y1).abs())))
                .into_iter()
                // create x,y points touched from delta
                .map(|d| (x1 + (xd * d), (y1 + (yd * d))))
                // update map
                .for_each(|(x, y)| *map.entry((x, y)).or_insert(0) += 1);
            map
        })
        .iter()
        .filter(|(_, &v)| v >= 2).count()
}

#[aoc(day5, part2)]
pub fn part2(inputs: &[Line]) -> usize {
    let mut map: HashMap<Point, i32> = HashMap::new();
    for l in inputs {
        let &((x1, y1), (x2, y2)) = l;
        // no diagonals
        if ! (x1 == x2 || y1 == y2) {
            let (xd, yd) = ((x2-x1).signum(), (y2-y1).signum());
            for d in 0..=((x2-x1).abs()) {
                *map.entry((x1+(d*xd), y1+(d*yd))).or_insert(0) += 1;
            }
        } else {
            // swap to make ranges work
            let (xs, xe) = match x1.cmp(&x2) {
                std::cmp::Ordering::Greater => (x2, x1),
                _ => (x1, x2)
            };
            let (ys, ye) = match y1.cmp(&y2) {
                std::cmp::Ordering::Greater => (y2, y1),
                _ => (y1, y2)
            };
            for x in xs..=xe {
                for y in ys..=ye {
                    *map.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
    }
    map.into_iter().filter(|(_, v)| v >= &2).count()
}

#[aoc(day5, part2, iter)]
pub fn part2_iter(inputs: &[Line]) -> usize {
    inputs
        .iter()
        .fold(HashMap::<Point, i32>::new(), |mut map, &((x1, y1), (x2, y2))| {
            let (xd, yd) = ((x2-x1).signum(), (y2-y1).signum());
            // number of step into_iter
            (0..=((x2-x1).abs().max((y2-y1).abs())))
                .into_iter()
                // create x,y points touched from delta
                .map(|d| (x1 + (xd * d), (y1 + (yd * d))))
                // update map
                .for_each(|(x, y)| *map.entry((x, y)).or_insert(0) += 1);
            map
        })
        .iter()
        .filter(|(_, &v)| v >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2");
        assert_eq!(part1(&input), 5)
    }

    #[test]
    pub fn test2() {
        let input = generator("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2");
        assert_eq!(part2(&input), 12)
    }
}
