use std::collections::{HashSet, HashMap};
use itertools::Itertools;

type Point = (i32, i32);

trait Neighbors {
    fn neighbors(self) -> [Point; 4];
    fn neighbors_diag(self) -> [Point; 8];
}

impl Neighbors for Point {
    fn neighbors(self) -> [Point; 4] {
        let (x,y) = self;
        [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
    }
    fn neighbors_diag(self) -> [Point; 8] {
        let (x,y) = self;
        [(x-1, y), (x+1, y), (x, y-1), (x, y+1), (x-1, y-1), (x-1, y+1), (x+1, y-1), (x+1, y+1)]
    }
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> HashMap<Point, i32>{
    let mut map: HashMap::<Point, i32> = HashMap::new();
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            map.entry((i as i32, j as i32)).or_insert(c.to_digit(10).unwrap() as i32);
        })
    });
    map.clone()
}

#[aoc(day9, part1)]
pub fn part1(map: &HashMap<Point, i32>) -> i32 {
    // iterate over all points
    map.iter()
        // filter for minima
        .filter_map(|(p, v)| {
            // all neighbors must be lower
            p.neighbors().iter()
                .filter_map(|n| map.get(n))
                .all(|x| x > map.get(p).unwrap())
                .then(||v+1)
        })
        .sum()
}

pub fn remove_point_recurse(p: Point, set: &mut HashSet<Point>) -> i32 {
    // already in another basin or out of board
    if ! set.remove(&p) {
        return 0
    }
    // recurse to all neighbors
    1 + p.neighbors().iter().map(|&n| remove_point_recurse(n, set)).sum::<i32>()
}

#[aoc(day9, part2)]
pub fn part2(map: &HashMap<Point, i32>) -> i32 {
    // clean up initial map to remove 9s (boundary)
    let mut points = map.iter().filter_map(|(&p, &v)| (v != 9).then(||p)).collect::<HashSet<Point>>();

    let mut basins = vec![];
    // now remove points recursively and split into basins
    while let Some(&p) = points.iter().next() {
        basins.push(remove_point_recurse(p, &mut points));
    }

    basins.iter().sorted().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("2199943210
3987894921
9856789892
8767896789
9899965678");
        assert_eq!(part1(&input), 15)
    }

    #[test]
    pub fn test2() {
        let input = generator("2199943210
3987894921
9856789892
8767896789
9899965678");
        assert_eq!(part2(&input), 1134)
    }
}
