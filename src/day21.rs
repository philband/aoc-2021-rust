use std::collections::{HashMap};
use itertools::Itertools;
use itertools::iproduct;

const END_SCORE: u8 = 21;

type Data = Vec<Player>;

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Clone)]
pub struct Player {
    score: i32,
    pos: u8,
}

impl Player {
    fn roll(&mut self, dice: &mut i32) {
        let advance = *dice * 3 + 6;
        *dice += 3;
        self.pos =((self.pos as i32 + advance - 1) % 10 +1) as u8;
        self.score += self.pos as i32
    }
}


#[aoc_generator(day21)]
pub fn generator(input: &str) -> Data {
    vec![Player{score: 0, pos: 1}, Player{score: 0, pos: 6}]
}


#[aoc(day21, part1)]
pub fn part1(inputs: &Data) -> i32 {
    let mut players = inputs.clone();
    let mut dice = 0;

    loop {
        for (id, mut p) in players.iter_mut().enumerate() {
            p.roll(&mut dice);
            if p.score >= 1000 {
                break;
            }
        }
        if players.clone().iter().map(|p| p.score).max().unwrap() >= 1000 {
            return players.iter().map(|p| p.score).min().unwrap() * dice
        }
    }
}

type Cache = HashMap<(u8, u8, u8, u8), (u64, u64)>;
pub fn play_game(cache: &mut Cache, rolls: &Vec<(u8, u64)>, pos1: u8, pos2: u8, score1: u8, score2: u8, success_lookups: &mut u64, iterations: &mut i64) -> (u64, u64) {
    *iterations += 1;
    if score1 >= END_SCORE {
        return (1, 0)
    } else if score2 >= END_SCORE {
        return (0, 1)
    }

    if let Some(&score) = cache.get(&(pos1,pos2,score1,score2)) {
        *success_lookups += 1;
        return score
    }

    let mut wins = (0,0);

    for (advance, multi) in rolls {
        let nextpos = (pos1 + *advance - 1) % 10 + 1;
        // swap players for next turn
        let (uw1, uw2) = play_game(cache, rolls,pos2, nextpos, score2, score1 + nextpos, success_lookups, iterations);
        wins.0 += uw2 * *multi;
        wins.1 += uw1 * *multi;
    }

    cache.insert((pos1,pos2,score1,score2), wins);
    wins
}


pub fn play_game_naive(cache: &mut Cache, pos1: u8, pos2: u8, score1: u8, score2: u8, success_lookups: &mut u64, iterations: &mut i64) -> (u64, u64) {
    *iterations += 1;
    if score1 >= END_SCORE {
        return (1, 0)
    } else if score2 >= END_SCORE {
        return (0, 1)
    }

    if let Some(&score) = cache.get(&(pos1,pos2,score1,score2)) {
        *success_lookups += 1;
        return score
    }

    let mut wins = (0,0);

    for (x, y, z) in iproduct!([1,2,3], [1,2,3], [1,2,3]) {
        let nextpos = (pos1 + x + y +z - 1) % 10 + 1;
        // swap players for next turn
        let (uw1, uw2) = play_game_naive(cache, pos2, nextpos, score2, score1 + nextpos, success_lookups, iterations);
        wins.0 += uw2;
        wins.1 += uw1;
    }

    cache.insert((pos1,pos2,score1,score2), wins);
    wins
}

#[aoc(day21, part2, cache)]
pub fn part2_cache(inputs: &Data) -> u64 {
    let rolls = {
        let map = iproduct!([1,2,3], [1,2,3], [1,2,3]).fold(HashMap::<u8, u64>::new(), |mut acc, (x,y,z)| {
            *acc.entry((x+y+z)).or_insert(0) += 1;
            acc
        });
        map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<(u8, u64)>>()
    };

    let mut success_lookups = 0;
    let mut iterations = 0;
    let mut cache = HashMap::new();
    let (u1, u2) = play_game(&mut cache, &rolls, inputs[0].pos, inputs[1].pos, 0, 0, &mut success_lookups, &mut iterations);

    println!("Successfull cache lookups: {}", success_lookups);
    println!("Total iterations: {}", iterations);
    println!("Cache size: {}", cache.len());
    println!("Player 1: {}", u1);
    println!("Player 2: {}", u2);

    u1.max(u2)
}

#[aoc(day21, part2, cache_naive)]
pub fn part2_cache_naive(inputs: &Data) -> u64 {
    let mut success_lookups = 0;
    let mut iterations = 0;
    let mut cache = HashMap::new();
    let (u1, u2) = play_game_naive(&mut cache, inputs[0].pos, inputs[1].pos, 0, 0, &mut success_lookups, &mut iterations);

    println!("Successfull cache lookups: {}", success_lookups);
    println!("Total iterations: {}", iterations);
    println!("Cache size: {}", cache.len());
    println!("Player 1: {}", u1);
    println!("Player 2: {}", u2);

    u1.max(u2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = vec![Player{score: 0, pos: 4}, Player{score: 0, pos: 8}];
        assert_eq!(part1(&input), 739785);
    }

    #[test]
    pub fn test2_cache() {
        assert_eq!(part2_cache(&vec![Player{score: 0, pos: 4}, Player{score: 0, pos: 8}]), 444356092776315);
        assert_eq!(part2_cache(&vec![Player{score: 0, pos: 1}, Player{score: 0, pos: 6}]), 157253621231420);
    }

    #[test]
    pub fn test2_cache_naive() {
        assert_eq!(part2_cache_naive(&vec![Player{score: 0, pos: 4}, Player{score: 0, pos: 8}]), 444356092776315);
        assert_eq!(part2_cache_naive(&vec![Player{score: 0, pos: 1}, Player{score: 0, pos: 6}]), 157253621231420);
    }


}
