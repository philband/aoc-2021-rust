use itertools::Itertools;


#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

pub fn points_pt1(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char: {}", c)
    }
}

pub fn points_pt2(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid char: {}", c)
    }
}

pub fn inverse(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("no inverse for {}", c)
    }
}

pub fn is_line_corrupted(l: &String) -> bool {
    let mut state = Vec::<char>::new();
    l.chars().into_iter().filter_map(| c| {
        //println!("Exp: {}", c);
        match c {
            '('|'['|'{'|'<' => {state.push(c); None},
            ')'|']'|'}'|'>' => {
                match state.pop() {
                    Some(d) => {
                        (c != inverse(d)).then(|| points_pt1(c))
                    },
                    _ => None,
                }
            },
            _ => panic!("unknown"),
        }
    }).into_iter().filter(|x| x != &0).count() > 0
}

#[aoc(day10, part1)]
pub fn part1(inputs: &[String]) -> i32 {
    inputs.iter().filter_map(|l| {
        let mut state = Vec::<char>::new();
        l.chars().into_iter().filter_map(| c| {
            match c {
                '('|'['|'{'|'<' => {
                    state.push(c);
                    None
                },
                ')'|']'|'}'|'>' => {
                    match state.pop() {
                        Some(d) => (c != inverse(d)).then(|| points_pt1(c)),
                        _ => None,
                    }
                },
                _ => panic!("unknown"),
            }
        }).into_iter().filter(|x| x != &0).next()
    })
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[String]) -> i64 {
    let scores = inputs.iter()
        .filter(|l| ! is_line_corrupted(l))
        .map(|l| {
            let mut state = Vec::<char>::new();
            l.chars().into_iter().for_each(| c| {
                //println!("Exp: {}", c);
                match c {
                    '('|'['|'{'|'<' => {state.push(c);},
                    ')'|']'|'}'|'>' => {state.pop();},
                    _ => panic!("unknown"),
                };
            });
            state.iter().rev().fold(0i64, |acc, c| acc * 5 + points_pt2(inverse(*c)))
    })
        .collect::<Vec<_>>();
    let middle = scores.len() / 2;
    scores.into_iter().sorted().nth(middle).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]");
        assert_eq!(part1(&input), 26397)
    }

    #[test]
    pub fn test2() {
        let input = generator("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]");
        assert_eq!(part2(&input), 288957)
    }
}
