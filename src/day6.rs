
#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<u64> {
    // parse all the ages
    let fishes = input
        .split(',')
        .map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    // group ages into vec with index = age and value = count
    (0..=8).into_iter().map(|age| fishes.iter().filter(|&x| *x == age).count() as u64).collect()
}

pub fn run(inputs: &[u64], n: u32) -> u64 {
    let lantern = inputs.clone().to_vec();
    (0..n).into_iter().fold(lantern, |v, _| {
        (0..=8).into_iter().fold(vec![0u64; 9], |mut w, i| {
            let j = match i {
                0 => {w[8] += v[i]; 6},
                _ => i-1
            };
            w[j] += v[i];
            w
        })
    }).iter().sum()
}

#[aoc(day6, part1)]
pub fn part1(inputs: &[u64]) -> u64 {
    run(inputs, 80)
}

#[aoc(day6, part2)]
pub fn part2(inputs: &[u64]) -> u64 {
    run(inputs, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("3,4,3,1,2");
        assert_eq!(part1(&input), 5934)
    }

    #[test]
    pub fn test2() {
        let input = generator("3,4,3,1,2");
        assert_eq!(part2(&input), 26984457539)
    }
}
