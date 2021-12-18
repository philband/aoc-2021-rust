use std::{iter::Peekable, str::Chars};
use std::fmt::{Display, Formatter};
use itertools::Itertools;

type Data = Vec<Element>;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Value(u8),
    Pair(Box<Element>, Box<Element>)
}

impl Element {
    fn from_string(s: &str) -> Element {
        let mut it = s.chars().into_iter().peekable();
        Self::parse_element(&mut it)
    }

    fn parse_element(it: &mut Peekable<Chars>) -> Element {
        match it.peek() {
            Some('[') => {
                it.next();
                let l = Box::new(Self::parse_element(it));
                debug_assert_eq!(it.peek(), Some(&','));
                it.next();
                let r = Box::new(Self::parse_element(it));
                debug_assert_eq!(it.peek(), Some(&']'));
                it.next();
                Self::Pair(l, r)
            }
            Some(c) => {
                let v = c.to_digit(10).unwrap() as u8;
                it.next();
                Self::Value(v)
            },
            None => unreachable!("Unexpected end")
        }
    }

    fn leftmost(&mut self) -> &mut u8 {
        match self {
            Self::Value(v) => v,
            Self::Pair(l, _) => l.leftmost(),
        }
    }
    fn rightmost(&mut self) -> &mut u8 {
        match self {
            Self::Value(v) => v,
            Self::Pair(_, r) => r.rightmost(),
        }
    }

    fn sum(self, other: Self) -> Element {
        let mut sum = Self::Pair(
            Box::from(self),
            Box::from(other)
        );
        sum.reduce();
        sum
    }

    fn reduce(&mut self) {
        while self.reduce_once() {}
    }

    fn reduce_once(&mut self) -> bool {
        match self {
            Self::Pair(l, r) => {
                if l.explode(1, None, Some(r.leftmost())) || r.explode(1, Some(l.rightmost()), None) {
                    true
                } else if l.split() || r.split() {
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }

    fn explode(&mut self, depth: usize, left_value: Option<&mut u8>, right_value: Option<&mut u8>) -> bool {
        if depth == 4 {
            match self {
                Self::Value(_) => false,
                Self::Pair(l, r) => {
                    l.explode_help(left_value);
                    r.explode_help(right_value);
                    *self = Self::Value(0);
                    true
                }
            }
        } else {
            match self {
                Self::Value(_) => false,
                Self::Pair(l, r) => {
                    l.explode(depth + 1, left_value, Some(r.leftmost()))
                    || r.explode(depth + 1, Some(l.rightmost()), right_value)
                }
            }
        }
    }

    fn explode_help(&self, v: Option<&mut u8>) {
        match (self, v) {
            (Self::Value(l), Some(v)) => { *v += *l; },
            (Self::Value(_), None) => {},
            _ => unreachable!()
        };
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Value(v) if *v >= 10 => {
                let l = *v / 2;
                let r = *v / 2 + *v % 2;
                *self = Self::Pair(
                    Box::from(Self::Value(l)),
                    Box::from(Self::Value(r))
                );
                true
            },
            Self::Pair(l, r) => l.split() || r.split(),
            _ => false
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Value(v) => *v as usize,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Pair(l, r) => write!(f, "[{},{}]", l, r)
        }
    }
}

impl std::str::FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Element::from_string(s))
    }
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Data {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn sum(inputs: &Data) -> Element {
    let start = inputs[0].clone();
    inputs
        .into_iter()
        .skip(1)
        .fold(start, |acc, e| acc.sum(e.clone()))
}

#[aoc(day18, part1)]
pub fn part1(inputs: &Data) -> usize {
    sum(inputs).magnitude()
}

#[aoc(day18, part2)]
pub fn part2(inputs: &Data) -> usize {
    inputs.into_iter().cartesian_product(inputs.into_iter()).filter_map(|(x, y)| {
        if *x == *y {
            None
        } else {
            Some(x.clone().sum(y.clone()).magnitude())
        }
    }).max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse() {
        assert_eq!(Element::from_string("[1,2]"), Element::Pair(
            Box::new(Element::Value(1)),
            Box::new(Element::Value(2)),
        ));
        assert_eq!(Element::from_string("[[1,2],3]"), Element::Pair(
            Box::new(Element::Pair(
                Box::new(Element::Value(1)),
                Box::new(Element::Value(2))
            )),
            Box::new(Element::Value(3)),
        ));
    }

    #[test]
    pub fn test_print() {
        assert_eq!(Element::Pair(
            Box::new(Element::Pair(
                Box::new(Element::Value(1)),
                Box::new(Element::Value(2))
            )),
            Box::new(Element::Value(3)),
        ).to_string(), "[[1,2],3]")
    }

    #[test]
    pub fn test_explode() {
        let examples = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
        ];

        for (i, o) in examples {
            let mut e = Element::from_string(i);
            assert!(e.reduce_once());
            assert_eq!(e, Element::from_string(o));
        }
    }

    #[test]
    pub fn test_sum() {
        assert_eq!(Element::from_string("[1,2]").sum(Element::from_string("[[3,4],5]")), Element::from_string("[[1,2],[[3,4],5]]"));
    }

    #[test]
    pub fn test_magnitude() {
        assert_eq!(Element::from_string("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(Element::from_string("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
        assert_eq!(Element::from_string("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(Element::from_string("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(Element::from_string("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
        assert_eq!(Element::from_string("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
    }

    #[test]
    pub fn test1_1() {
        let input = generator("[1,1]
[2,2]
[3,3]
[4,4]");
        assert_eq!(sum(&input), Element::from_string("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
    }

    #[test]
    pub fn test1_2() {
        let input = generator("[1,1]
[2,2]
[3,3]
[4,4]
[5,5]");
        assert_eq!(sum(&input), Element::from_string("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
    }

    #[test]
    pub fn test1_3() {
        let input = generator("[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]");
        assert_eq!(sum(&input), Element::from_string("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    }

    #[test]
    pub fn test1_4() {
        let input = generator("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]");
        assert_eq!(sum(&input), Element::from_string("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    }

    #[test]
    pub fn test1_5() {
        let input = generator("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");
        assert_eq!(part1(&input), 4140);
    }


    #[test]
    pub fn test2() {
        let input = generator("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");

        assert_eq!(part2(&input), 3993);
    }

}
