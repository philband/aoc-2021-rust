use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(" | ").collect::<Vec<_>>();
            let mut it = parts.iter();
            (it.next().unwrap().split(" ").map(|s| s.to_string()).collect(), it.next().unwrap().split(" ").map(|s| s.chars().sorted().collect()).collect())
        }).collect()
}

#[aoc(day8, part1)]
pub fn part1(inputs: &[(Vec<String>, Vec<String>)]) -> i32 {
    inputs
        .clone()
        .into_iter()
        .map(|(_obs, out)| out.iter().filter(|d| [2,3,4,7].contains(&(d.len() as i32))).count() as i32)
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(inputs: &[(Vec<String>, Vec<String>)]) -> i32 {
    let _numbers = vec!["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
    inputs
        .clone()
        .into_iter()
        .map(|(obs, out)| {
            let panel_len = obs.clone().into_iter().map(|panel| (panel.clone(), panel.clone().len() as i32)).collect::<Vec<_>>();

            let n1 = panel_len.iter().filter(|(_, l)| l == &2).map(|(s, _)| s).next().unwrap().as_str();
            let n4 = panel_len.iter().filter(|(_, l)| l == &4).map(|(s, _)| s).next().unwrap().as_str();
            let n7 = panel_len.iter().filter(|(_, l)| l == &3).map(|(s, _)| s).next().unwrap().as_str();
            let n8 = panel_len.iter().filter(|(_, l)| l == &7).map(|(s, _)| s).next().unwrap().as_str();
            let n235 = panel_len.iter().filter(|(_, l)| l == &5).map(|(s, _)| s.as_str()).collect::<Vec<_>>();
            let n069 = panel_len.iter().filter(|(_, l)| l == &6).map(|(s, _)| s.as_str()).collect::<Vec<_>>();
            let n6 = n069.iter().filter(|&&x| ! n1.chars().all(|y| x.chars().any(|xx| y == xx))).next().unwrap();
            let n09 = n069.iter().filter(|&&x| &x != n6).map(|s| *s).collect::<Vec<_>>();
            let cc = n8.chars().filter(|x| ! n6.chars().any(|c| &c == x)).next().unwrap();
            let cf = n1.chars().filter(|x| x != &cc).next().unwrap();
            let n5 = n235.iter().filter(|&&x| x.chars().all(|y| y != cc)).next().unwrap();
            let n23 = n235.iter().filter(|&&x| &x != n5).map(|s| *s).collect::<Vec<_>>();
            let n2 = n23.iter().filter(|&&x| x.chars().all(|xx| xx != cf)).next().unwrap();
            let n3 = n23.iter().filter(|&&x| &x != n2).next().unwrap();
            let n0 = n09.iter().filter(|&&x| ! n3.chars().all(|y| x.chars().any(|xx| y == xx))).next().unwrap();
            let n9 = n09.iter().filter(|&&x| &x != n0).next().unwrap();

            let mut map: HashMap<String, i32> = HashMap::new();
            for (v, k) in [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9].iter().enumerate() {
                map.entry(k.chars().sorted().collect()).or_insert(v as i32);
            }

            out.into_iter().map(|s| map.get(s).unwrap()).fold(0, | acc, x| acc * 10 + x)
        }
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        assert_eq!(part1(&input), 26)
    }

    #[test]
    pub fn test2() {
        let _testinput = generator("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
        let input = generator("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        assert_eq!(part2(&input), 61229);
        //assert_eq!(part2(&testinput), 5353)
    }
}
