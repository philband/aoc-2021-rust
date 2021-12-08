use statistical::{mean, median};

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[i32]) -> i32 {
    let median = median(inputs.clone());
    inputs.iter().map(|x| (median - x).abs()).sum()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[i32]) -> i32 {
    let m = mean(inputs.clone().iter().map(|&x| x as f32).collect::<Vec<_>>().as_slice()).floor() as i32;
    inputs.clone().into_iter().map(|&x| (1..=((m - x).abs())).into_iter().sum::<i32>()).sum()
    /*(m-2..=m+2).into_iter()
        .map(|m| {
            inputs
                .clone()
                .into_iter()
                .map(|&x|
                    (1..=((m - x).abs()))
                        .into_iter()
                        .sum::<i32>()
                ).sum()
        })
        .min()
        .unwrap()*/

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(part1(&input), 37)
    }

    #[test]
    pub fn test2() {
        let input = generator("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(part2(&input), 168)
    }
}
