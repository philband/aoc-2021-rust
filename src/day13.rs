use std::collections::HashSet;

type Data = (Vec<Point>, Vec<Fold>);
type Point = (i32, i32);
type Fold = (char, i32);


#[aoc_generator(day13)]
pub fn generator(input: &str) -> Data {
    let mut split = input.split("\n\n");
    let points = split.next().unwrap().lines().map(|l| {
        let mut c = l.split(",").map(|x| x.parse::<i32>().unwrap());
        (c.next().unwrap(), c.next().unwrap())
    }).collect::<Vec<Point>>();

    let instructions = split.next().unwrap().lines().map(|l| {
        let mut i = l.strip_prefix("fold along ").unwrap().split("=");
        (i.next().unwrap().chars().next().unwrap(), i.next().unwrap().parse::<i32>().unwrap())
    }).collect::<Vec<Fold>>();
    (points, instructions)
}

pub fn fold(f: Fold, set: &mut HashSet<Point>) {
    let foldline = f.1;
    let compare = match f.0 {
        'x' => |(x,_y), f| x > f,
        'y' => |(_x,y), f| y > f,
        _ => panic!("unknown {}", f.0)
    };

    let folder = match f.0 {
        'x' => |(x,y), f| (2 * f - x, y),
        'y' => |(x,y), f| (x, 2 * f - y),
        _ => panic!("unknown {}", f.0)
    };

    set.clone().iter().filter(|&p| compare(*p, foldline)).for_each(|p| {
        set.remove(p);
        set.insert(folder(*p, foldline));
    });
}


#[aoc(day13, part1)]
pub fn part1(inputs: &Data) -> usize {
    let mut set: HashSet<Point> = HashSet::new();
    let (pts, ins) = inputs;
    pts.iter().for_each(|p| { set.insert(*p); });

    fold(ins[0], &mut set);
    set.iter().count()
}

#[aoc(day13, part2)]
pub fn part2(inputs: &Data) -> usize {
    let mut set: HashSet<Point> = HashSet::new();
    let (pts, ins) = inputs;
    pts.iter().for_each(|p| { set.insert(*p); });

    ins.iter().for_each(|i| fold(*i, &mut set));

    let xmax = set.iter().map(|(x, _)| *x).max().unwrap();
    let ymax = set.iter().map(|(_, y)| *y).max().unwrap();

    for y in 0..=ymax {
        for x in 0..=xmax {
            print!("{}", set.contains(&(x, y)).then(|| '#').or_else(|| Some('.')).unwrap())
        }
        println!();
    }

    set.iter().count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5");

        assert_eq!(part1(&input), 17);
    }

    #[test]
    pub fn test2() {
        let input = generator("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5");

        assert_eq!(part2(&input), 16);
    }
}
