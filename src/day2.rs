type Data = Vec<Instruction>;

pub struct Instruction {
    direction: char,
    distance: i32,
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Data {
    input.lines().map(|l|{
        Instruction { direction: l.chars().next().unwrap(), distance: l.split(' ').skip(1).next().unwrap().parse::<i32>().unwrap()}
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(inputs: &Data) -> i32 {
    let (depth, distance) = inputs
        .iter()
        .fold((0, 0), |mut acc, i| {
            let change = match i.direction {
                'u' => (0, -1 * i.distance),
                'd' => (0, 1 * i.distance),
                'f' => (1 * i.distance, 0),
                _ => panic!("unknown op {}", i.direction)
            };
            acc.0 += change.0;
            acc.1 += change.1;
            acc
        });
    depth * distance
}

#[aoc(day2, part2)]
pub fn part2(inputs: &Data) -> i32 {
    let (depth, distance, _) = inputs
        .iter()
        .fold((0, 0, 0), |mut acc, i| {
            let change = match i.direction {
                'u' => (0, 0, -1 * i.distance),
                'd' => (0, 0, 1 * i.distance),
                'f' => (acc.2 * i.distance, 1 * i.distance, 0),
                _ => panic!("unknown op {}", i.direction)
            };
            acc.0 += change.0;
            acc.1 += change.1;
            acc.2 += change.2;
            acc
        });
    depth * distance
}
