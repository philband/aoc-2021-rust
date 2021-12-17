use itertools::Itertools;
use scan_fmt::scan_fmt;

type Data = (i32, i32, i32, i32);

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Data {
    scan_fmt!(input, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap()
}

pub fn check_trajectory(target: &Data, svs: &(i32, i32)) -> Option<i32> {
    let (x0, x1, y0, y1) = *target;
    let (mut xv, mut yv) = *svs;
    let (mut x, mut y) = (0,0);
    let mut maxy = 0;

    loop {
        x += xv;
        y += yv;
        if xv > 0 {
            xv -= 1;
        }
        yv -= 1;
        if y > maxy {
            maxy = y;
        }
        if x > x1 || y < y0 {
            break;
        }
        if x >= x0 && x <= x1 && y >= y0 && y <= y1 {
            return Some(maxy)
        }
    }
    None
}


#[aoc(day17, part1)]
pub fn part1(inputs: &Data) -> i32 {
    (1..1000)
        .cartesian_product(1..1000)
        .into_iter()
        .filter_map(|sv| check_trajectory(inputs, &sv))
        .max()
        .unwrap()
}

#[aoc(day17, part2)]
pub fn part2(inputs: &Data) -> usize{
    (1..1000)
        .cartesian_product(-1000..1000)
        .into_iter()
        .filter_map(|sv| check_trajectory(inputs, &sv))
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(part1(&(20, 30, -10, -5)), 45);
    }


    #[test]
    pub fn test2() {
        assert_eq!(part2(&(20, 30, -10, -5)), 112);
    }

}
