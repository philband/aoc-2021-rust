use itertools::Itertools;
use nalgebra::Matrix5;

pub type Board = Matrix5<(u32, bool)>;

pub struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Input {
    let mut lines = input.split("\n\n");

    let numbers = lines.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();

    let boards = lines
        .map(|b| {
            Matrix5::from_iterator(b.lines().flat_map(|l| {
                l.split_whitespace().map(|x| (x.parse::<u32>().unwrap(), false))
            }))
        }).collect::<Vec<Board>>();
    Input {numbers, boards}
}

fn update_played(board: &mut Board, num: u32) {
    board.iter_mut().for_each(|x| if x.0 == num {x.1 = true;})
}

fn check_win(board: &Board) -> bool {
    board.column_iter().any(|c| c.iter().all(|(_, played)| *played)) ||
        board.row_iter().any(|r| r.iter().all(|(_, played)| *played))
}

fn sum_board_unmarked(board: &Board) -> u32 {
    board.iter().filter_map(|(x, played)| if ! *played {Some(*x)} else {None}).sum()
}

#[aoc(day4, part1)]
pub fn part1(inputs: &Input) -> u32 {
    let (numbers, mut boards) = (inputs.numbers.clone(), inputs.boards.clone());
    for num in numbers {
        for b in boards.iter_mut() {
            update_played(b, num);
            if check_win(b) {
                return num * sum_board_unmarked(b)
            }
        }
    }
    0
}

#[aoc(day4, part2)]
pub fn part2(inputs: &Input) -> u32 {
    let (numbers, mut boards) = (inputs.numbers.clone(), inputs.boards.clone());
    let mut won_boards = vec![false; boards.len()];

    for num in numbers {
        for (i, b) in boards.iter_mut().enumerate(){
            update_played(b, num);
            if check_win(b) {
                won_boards[i] = true;
                if won_boards.iter().all(|x| *x) {
                    return num * sum_board_unmarked(&b)
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");
        assert_eq!(part1(&input), 4512)
    }

    #[test]
    pub fn test2() {
        let input = generator("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");
        assert_eq!(part2(&input), 1924)
    }
}
