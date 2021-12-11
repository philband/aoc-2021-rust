use std::collections::{HashSet, HashMap};

type Point = (i32, i32);

trait Neighbors {
    fn neighbors_diag(self) -> [Point; 8];
}

impl Neighbors for Point {
    fn neighbors_diag(self) -> [Point; 8] {
        let (x,y) = self;
        [(x-1, y), (x+1, y), (x, y-1), (x, y+1), (x-1, y-1), (x-1, y+1), (x+1, y-1), (x+1, y+1)]
    }
}


#[aoc_generator(day11)]
pub fn generator(input: &str) -> HashMap<Point, i32>{
    let mut map: HashMap::<Point, i32> = HashMap::new();
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            map.entry((i as i32, j as i32)).or_insert(c.to_digit(10).unwrap() as i32);
        })
    });
    map.clone()
}

pub fn flash(p: Point, map: &mut HashMap<Point, i32>, flashes: &mut HashSet<Point>) {
    if flashes.contains(&p) {
        return
    } else {
        flashes.insert(p);
    }

    p.neighbors_diag().iter().for_each(|p| {
        if map.contains_key(p) {
            let e = map.get_mut(p).unwrap();
            *e += 1;
            (*e > 9).then(|| flash(*p, map, flashes));
        }
    })
}

pub fn step(map: &mut HashMap<Point, i32>) -> HashSet<Point> {
    let mut flashes = HashSet::<Point>::new();
    map.iter_mut().for_each(|(_p, e)| *e += 1);
    map.clone().iter().filter(|&(_p, e)| *e > 9).for_each(|(&p, _e)| flash(p, map, &mut flashes));
    map.iter_mut().filter(|(_p, e)| **e > 9).for_each(|(_p, e)| *e = 0);
    flashes
}

#[aoc(day11, part1)]
pub fn part1(map: &HashMap<Point, i32>) -> usize {
    let mut state = map.clone();
    (0..100).into_iter().map(|_| step(&mut state).iter().count()).sum()
}

#[aoc(day11, part2)]
pub fn part2(map: &HashMap<Point, i32>) -> i32 {
    let mut state = map.clone();
    (1..).find(|_| step(&mut state).iter().count() == 100).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526");
        assert_eq!(part1(&input), 1656)
    }

    #[test]
    pub fn test2() {
        let input = generator("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526");
        assert_eq!(part2(&input), 195)
    }
}
