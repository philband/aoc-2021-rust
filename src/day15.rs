use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point(i32, i32);
type Data = HashMap<Point, i32>;


impl Point {
    pub fn neighbors(&self) -> [Point; 4] {
        return [Point(self.0-1, self.1), Point(self.0+1, self.1), Point(self.0, self.1-1), Point(self.0, self.1+1)]
    }

    pub fn costed_neighbors(&self, map: &Data) -> Vec<(Point, i32)> {
        self.neighbors().into_iter().filter_map(|p| {
            match map.get(&p) {
                Some(&cost) => Some((p, cost)),
                _ => None
            }
        }).collect()
    }
}


#[aoc_generator(day15)]
pub fn generator(input: &str) -> Data {
    let mut map = Data::new();
    input
        .lines()
        .enumerate()
        .for_each(|(y, l)| {
            l.chars().enumerate().into_iter().for_each(|(x, c)| {
                map.insert(Point(x as i32, y as i32), c.to_digit(10).unwrap() as i32);
            })
        });
    map
}

pub fn find_shortest_path(map: &Data) -> i32 {
    let mut frontier = VecDeque::<(Point, i32)>::new();
    let mut distances = HashMap::<Point, i32>::new();
    frontier.push_front((Point(0, 0), 0));

    while ! frontier.is_empty() {
        let (cur, d) = frontier.pop_front().unwrap();

        cur.neighbors().iter().filter_map(|next| {
            match map.get(&next) {
                Some(cost) => Some((next, *cost)),
                None => None
            }
        })
            .sorted_by(|(_, d1), (_, d2)| d1.cmp(&d2))
            .for_each(|(next, cost)| {
                if ! distances.contains_key(next) {
                    distances.insert(next.clone(), d+cost);
                    frontier.push_back((next.clone(), d+cost));
                } else {
                    let distance = distances.get_mut(next).unwrap();
                    if *distance > (d+cost) {
                        *distance = d+cost;
                        frontier.push_back((next.clone(), d+cost));
                    }
                }
            });

    }

    let (xmax, ymax) = (map.iter().map(|(&Point(x, _y), _)| x).max().unwrap() + 1, map.iter().map(|(&Point(_x, y), _)| y).max().unwrap() + 1);
    *distances.get(&Point(xmax, ymax)).unwrap()
}

pub fn find_shortest_path2(map: &Data) -> i32 {
    let (xmax, ymax) = (map.iter().map(|(&Point(x, _y), _)| x).max().unwrap(), map.iter().map(|(&Point(_x, y), _)| y).max().unwrap());
    let result = pathfinding::prelude::dijkstra(
        &Point(0,0),
        |p| p.costed_neighbors(map),
        |p| *p == Point(xmax, ymax)
    );
    if result.is_some() {
        let (_, cost) = result.unwrap();
        return cost
    }
    return 0
}




#[aoc(day15, part1)]
pub fn part1(inputs: &Data) -> i32 {
    find_shortest_path2(inputs)

}

#[aoc(day15, part2)]
pub fn part2(inputs: &Data) -> i32 {
    let mut map = inputs.clone();
    let (xsize, ysize) = (map.iter().map(|(&Point(x, _y), _)| x).max().unwrap() + 1, map.iter().map(|(&Point(_x, y), _)| y).max().unwrap() + 1);

    (0..xsize).cartesian_product(0..ysize).for_each(|(x,y)| {
        let source = *map.get(&Point(x,y)).unwrap();
        (0..5).cartesian_product(0..5)
            .skip(1)
            .map(|(xt, yt)|
                (
                    x + (xt * xsize),
                    y + (yt * ysize),
                    (source + xt + yt - 1) % 9 + 1
                ))
            .for_each(|(xt, yt, cost)| { map.insert(Point(xt, yt),cost); });
    });

    find_shortest_path2(&map)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581");

        assert_eq!(part1(&input), 40);
    }

    #[test]
    pub fn test2() {
        let input = generator("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581");


        assert_eq!(part2(&input), 315);
    }

    #[test]
    pub fn test3() {
        let input = generator("19999
19111
11191");

        assert_eq!(part1(&input), 8)
    }
}
