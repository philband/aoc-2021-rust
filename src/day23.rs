use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap};

use itertools::Itertools;

use crate::day6::run;

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    cost: i32,
    maze: Maze,
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


type Maze = HashMap<Point, Field>;

#[derive(Debug, Hash, Eq, Ord, PartialOrd, PartialEq, Clone, Copy)]
pub struct Point(i8, i8);

type Field = char;

trait AmphipodType {
    fn is_empty(&self) -> bool;
    fn is_player(&self) -> bool;
}

impl AmphipodType for Field {
    fn is_empty(&self) -> bool {
        *self == '.'
    }
    fn is_player(&self) -> bool {
        !self.is_empty()
    }
}


impl Point {
    fn manhattan(&self, other: &Self) -> i32 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as i32
    }

    fn is_room(&self) -> bool {
        self.1 > 1
    }

    fn is_hallway(&self) -> bool {
        !self.is_room()
    }

    fn is_forbidden(&self) -> bool {
        match (self.is_hallway(), self.0) {
            (true, 3 | 5 | 7 | 9) => true,
            _ => false
        }
    }
}

trait AmphipodField {
    fn cost(self) -> i32;
    fn destination(self) -> i8;
}

impl AmphipodField for Field {
    fn cost(self) -> i32 {
        match self {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        }
    }

    fn destination(self) -> i8 {
        match self {
            'A' => 3,
            'B' => 5,
            'C' => 7,
            'D' => 9,
            _ => unreachable!()
        }
    }
}

trait AmphipodGame {
    fn get_moves(&self) -> Vec<(i32, Self)> where Self: Sized;
    fn is_path_x_clear(&self, y: i8, from: i8, to: i8, skip_first: bool) -> bool;
    fn is_path_y_clear(&self, x: i8, from: i8, to: i8, skip_first: bool) -> bool;
    fn is_done(&self) -> bool;
    fn to_int(&self) -> u128;
    fn print(&self);
}

impl AmphipodGame for Maze {
    fn get_moves(&self) -> Vec<(i32, Maze)> {
        // Enumerate all possible target points (empty spaces), that we are allowed to move to
        let targets: Vec<Point> = self
            .iter()
            .filter(|(p, _)| !p.is_forbidden())
            .filter_map(|(p, f)| f.is_empty().then(|| *p)).collect();

        // Iterate over everything movable
        self.iter()
            .filter(|(p, f)| f.is_player())
            // collect all possible moves for all movables
            .flat_map(|(from, f)| {
                let mut possibles: Box<dyn Iterator<Item=&Point>> = Box::new(targets.iter());
                // filter target iterator based on what we are and where we are
                possibles = match from.is_room() {
                    true => {
                        // only moving to hallways that are reachable
                        Box::new(possibles.filter(|to| {
                            to.is_hallway()
                                && self.is_path_y_clear(from.0, from.1, 1, true)
                                && self.is_path_x_clear(1, from.0, to.0, false)
                        }))

                    }
                    false => {
                        // for amphipods currently in a hallway only the deepest field (max-y) of the target room is valid
                        let dest_room = f.destination();
                        // find out if there are amphipods of other type in our room
                        let no_others_in_room = self.iter().filter(|(tp, tf)| {
                            tp.is_room()
                                && tp.0 == dest_room
                                && tf.is_player()
                                && f != *tf
                        }).count() == 0;

                        let deepest = self.iter()
                            // only moving to rooms
                            .filter(|(p, _)| p.is_room())
                            // that are "ours" and empty
                            .filter_map(|(Point(x, y), tf)| (f.destination() == *x && tf.is_empty()).then(|| *y))
                            .max();

                        match (no_others_in_room, deepest)
                        {
                            (true, Some(depth)) if depth >= 1 => Box::new(possibles.filter(move |to| {
                                to.is_room()
                                    && to.0 == dest_room
                                    && to.1 == depth
                                    && self.is_path_x_clear(from.1, from.0, dest_room, true)
                                    && self.is_path_y_clear(dest_room, from.1, to.1, false)
                            })),
                            _ => Box::new(possibles.filter(|_| false))
                        }
                    }
                };

                possibles
                    .map(|t| {
                        let mut maze = self.clone();
                        maze.insert(*from, '.');
                        maze.insert(*t, *f);
                        (from.manhattan(t) * f.cost(), maze)
                    })
            }).collect()
    }

    #[inline]
    fn is_path_x_clear(&self, y: i8, from: i8, to: i8, skip_first: bool) -> bool {
        let dir = (to - from).signum();
        let mut x = from;
        if skip_first {
            x += dir;
        }
        while x != to {
            if !self.get(&Point(x, y)).unwrap().is_empty() {
                return false;
            }
            x += dir;
        }
        true
    }

    #[inline]
    fn is_path_y_clear(&self, x: i8, from: i8, to: i8, skip_first: bool) -> bool {
        let dir = (to - from).signum();
        let mut y = from;
        if skip_first {
            y += dir;
        }
        while y != to {
            if !self.get(&Point(x, y)).unwrap().is_empty() {
                return false;
            }
            y += dir;
        }
        true
    }


    fn is_done(&self) -> bool {
        self
            .iter()
            .filter(|(p, f)| f.is_player())
            .all(|(p, f)| f.destination() == p.0 && p.is_room())
    }

    fn to_int(&self) -> u128 {
        self
            .iter()
            .sorted_by_key(|&(a, _)| a)
            .fold(0, |acc, (_, f)| {
                (acc << 3) + match &f {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    'D' => 4,
                    '.' => 5,
                    _ => unreachable!()
                }
            })
    }

    fn print(&self) {
        let maxy = *self.iter().map(|(Point(_, y), _)| y).max().unwrap();
        for y in 1..=maxy {
            for x in 0..=12 {
                let c = match self.get(&Point(x, y)) {
                    Some(&x) => x,
                    _ => '#',
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}


pub fn run_game(input: &Maze) -> (Vec<Maze>, i32) {
    let mut dist = HashMap::<u128, (Maze, i32)>::new();
    let mut q = BinaryHeap::new();
    q.push(State { cost: 0, maze: input.clone() });

    // Iterate over queue
    while let Some(state) = q.pop() {
        // Check if sloves
        if state.maze.is_done() {
            // Retrace steps
            let mut moves = Vec::new();
            let mut rev = state.maze.clone();
            while rev != *input {
                moves.push(rev.clone());
                rev = dist.get(&rev.to_int()).unwrap().0.clone();
            }
            moves.push(rev);
            return (moves, state.cost);
        }

        // Add possible moves to queue
        for (d, m) in state.maze.get_moves() {
            let next_cost = state.cost + d;
            // Only add if it is the least costly move to that target
            let &(_, c) = dist.get(&m.to_int()).unwrap_or(&(HashMap::new(), 1000000));
            if c > next_cost {
                dist.insert(m.to_int(), (state.maze.clone(), next_cost));
                q.push(State { cost: next_cost, maze: m });
            }
        }
    }
    (Vec::new(), 69)
}



pub fn extend(inputs: &Maze) -> Maze {
    let mut maze = Maze::new();
    inputs.iter().for_each(|(&Point(x, y), f)| {
        let np = match y {
            1 => Point(x, y),
            2 => Point(x, y),
            3 => Point(x, y + 2),
            _ => unreachable!()
        };
        maze.insert(np, *f);
    });

    // INSERT
    // #D#C#B#A#
    maze.insert(Point(3, 3), 'D');
    maze.insert(Point(5, 3), 'C');
    maze.insert(Point(7, 3), 'B');
    maze.insert(Point(9, 3), 'A');

    // #D#B#A#C#
    maze.insert(Point(3, 4), 'D');
    maze.insert(Point(5, 4), 'B');
    maze.insert(Point(7, 4), 'A');
    maze.insert(Point(9, 4), 'C');

    maze
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Maze {
    input.lines().enumerate().fold(HashMap::<Point, char>::new(), |mut acc, (y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let data = match c {
                '#' | ' ' => None,
                x => Some(x),
            };
            if let Some(field) = data {
                acc.insert(Point(x as i8, y as i8), field);
            }
        });
        acc
    })
}


#[aoc(day23, part1)]
pub fn part1(inputs: &Maze) -> i32 {
    let (_, cost) = run_game(inputs);

    /*for m in moves.iter().rev() {
        m.print();
    }*/

    cost
}

#[aoc(day23, part2)]
pub fn part2(inputs: &Maze) -> i32 {
    let maze = extend(inputs);

    let (_, cost) = run_game(&maze);

    cost
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########");
        assert_eq!(part1(&input), 12521);
    }

    #[test]
    pub fn test2() {
        let input = generator("#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########");
        assert_eq!(part2(&input), 44169);
    }
}
