use std::collections::{HashMap, VecDeque};

type Data = (Vec<u8>, Vec<([u8;2], usize, usize)>, Vec<u64>);
type Search = (char, char);


#[aoc_generator(day14)]
pub fn generator(input: &str) -> Data {
    let (base, rules) = input.split_once("\n\n").unwrap();
    let base = base.as_bytes().to_vec();

    // map to tuples of [a,b] => [a,c]
    // a and b are the source chars, c is the replacement char
    let mut rules = rules.lines().map(|l| {
        let (f, t) = l.split_once(" -> ").unwrap();
        let (f, t) = (f.as_bytes(), t.as_bytes()[0]);
        ([f[0], f[1]], [f[0], t])
    }).collect::<Vec<_>>();
    // sort by source
    rules.sort_unstable_by_key(|r| r.0);

    // compile rules
    // [a,b] will expand to [a,c], [c,b]
    let rule = rules.iter().map(|r| {
        (
            r.0,
            rules.binary_search_by_key(&r.1, |r| r.0).unwrap(),
            rules.binary_search_by_key(&[r.1[1], r.0[1]], |r| r.0).unwrap()
        )
    }).collect::<Vec<_>>();

    let mut num = vec![0u64; rule.len()];
    // create initial tuple frequencies
    base.windows(2).for_each(|k| num[rule.binary_search_by_key(&k, |r| &r.0).unwrap()] += 1);

    (base, rule, num)
}

pub fn step_n(base: Vec<u8>, num: Vec<u64>, rule: Vec<([u8;2], usize, usize)>, n: i32) -> u64 {

    let mut num = num.clone();
    let mut next = vec![0u64; rule.len()];

    for _ in 0..n {
        // apply frequency manipulation
        // every n char tuple will expand to n*2 new char tuples in the iteration. reset original count to 0
        // num and rule are sorted in order for the zipping to have correct pairs
        num.iter_mut().zip(&rule).for_each(|(n, r)| {
            next[r.1] += *n;
            next[r.2] += *n;
            *n = 0;
        });
        // swap
        std::mem::swap(&mut num, &mut next);
    }

    let mut occurences = [0u64; 256];
    occurences[*base.last().unwrap() as usize] += 1;

    rule.iter().zip(num).for_each(|(r, n)| occurences[r.0[0] as usize] += n as u64);

     occurences.iter().max().unwrap() - occurences.iter().filter(|&&x| x != 0).min().unwrap()
}


#[aoc(day14, part1)]
pub fn part1(inputs: &Data) -> u64 {
    let (base, rule, mut num) = inputs.clone();
    step_n(base, num, rule, 10)
}

#[aoc(day14, part2)]
pub fn part2(inputs: &Data) -> u64 {
    let (base, rule, mut num) = inputs.clone();
    step_n(base, num, rule, 40)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C");

        assert_eq!(part1(&input), 1588);
    }

    #[test]
    pub fn test2() {
        let input = generator("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C");


        assert_eq!(part2(&input), 2188189693529);
    }
}
