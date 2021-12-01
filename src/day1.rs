use itermore::IterMore;

type Data = Vec<i32>;

#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Data {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(inputs: &Data) -> usize {
    inputs.iter().zip(inputs[1..].iter()).filter(|(x,y)| y > x).count()
}

#[aoc(day1, part1, golf1)]
pub fn part1_golf1(inputs: &Data) -> usize {
    inputs.iter().windows().filter(|[x, y]| y > x).count()
}

#[aoc(day1, part2)]
pub fn part2(inputs: &Data) -> usize {
    let windows: Vec<i32> = inputs.windows(3).into_iter().map(|x| x.iter().sum()).collect();
    windows.iter().zip(windows[1..].iter()).filter(|(x,y)| y > x).count()
}

#[aoc(day1, part2, golf1)]
pub fn part2_golf1(inputs: &Data) -> usize {
    inputs.iter().windows().filter(|[x, _, _, y]| y > x).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [i32; 10] = [199,200,208,210,200,207,240,269,260,263];

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 7);
        assert_eq!(part1_golf1(&SAMPLE), 7);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 5);
        assert_eq!(part2_golf1(&SAMPLE), 5);
    }
}
