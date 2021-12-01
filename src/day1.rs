#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(inputs: &[i32]) -> i32 {
    inputs.iter().zip(inputs[1..].iter()).filter(|(x,y)| y > x).count() as i32
}

#[aoc(day1, part2)]
pub fn part2(inputs: &[i32]) -> i32 {
    let mut windows: Vec<i32> = Vec::new();
    for i in 0..inputs.len()-2 {
        windows.push(inputs[i] + inputs[i+1] + inputs[i+2])
    }
    windows.iter().zip(windows[1..].iter()).filter(|(x,y)| y > x).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [i32; 10] = [199,200,208,210,200,207,240,269,260,263];

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 7);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 5);
    }
}
