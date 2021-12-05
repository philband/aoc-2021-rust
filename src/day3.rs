#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[String]) -> i32 {
    let length = inputs.len() as i32;
    let gamma_bin: String = inputs.iter()
        // map each line to a Vec<i32> with 1/0
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        // fold vectors by summing each char position
        .into_iter().fold(vec![0i32; 12], |sum, l| -> Vec<i32> {
            sum.iter().zip(l).map(|(s, e)| s + e).collect()
        })
        // map to binary
        .iter().map(|e| if e >= &(length / 2) { '1' } else { '0' }).collect();

    let gamma = i32::from_str_radix(gamma_bin.as_str(), 2).unwrap();
    let epsilon = ((2_i32.pow(gamma_bin.len() as u32)) - 1) - gamma;
    gamma * epsilon
}

pub fn filter_by_compare(inputs: &[String], compare: char) -> i32 {
    let mut candidates = inputs.clone().to_vec();
    let inverse = match compare {
        '1' => '0',
        _ => '1'
    };

    for i in 0..inputs[0].len() {
        let ones: i32 = candidates.clone().iter().map(|l| if l.chars().nth(i).unwrap() == '1' { 1 } else { 0 }).sum();
        let compare = match ones.cmp(&(candidates.len() as i32 - ones)) {
            std::cmp::Ordering::Less => compare,
            _ => inverse
        };
        candidates = candidates.into_iter().filter(|l| l.chars().nth(i).unwrap() == compare).collect::<Vec<_>>();
        if candidates.len() == 1 {
            return i32::from_str_radix(candidates[0].clone().as_str(), 2).unwrap();
        }
    }
    0
}

#[aoc(day3, part2)]
pub fn part2(inputs: &[String]) -> i32 {
    filter_by_compare(inputs.clone(), '0') * filter_by_compare(inputs.clone(), '1')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        //assert_eq!(part1(&SAMPLE), 1721 * 299);
        assert_eq!(part1(&generator("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010")), 198)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010")), 230)
    }
}
