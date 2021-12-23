use std::cmp::{Ordering};
use std::collections::{BinaryHeap, HashMap};
use crate::day6::run;


#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    cost: i32,
    maze: Maze
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
pub struct Point (i8, i8);

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
        ! self.is_empty()
    }
}


impl Point {
    fn manhattan(&self, other: &Self) -> i32 {
        ((self.0-other.0).abs() + (self.1-other.1).abs()) as i32
    }

    fn is_room(&self) -> bool {
        self.1 > 1
    }

    fn is_hallway(&self) -> bool {
        ! self.is_room()
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
    fn get_destination(self) -> i8;
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

    fn get_destination(self) -> i8 {
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
    fn is_reachable(&self, from: &Point, to: &Point) -> bool;
    fn is_done(&self) -> bool;
    fn to_vec(&self) -> Vec<(Point, char)>;
    fn print(&self);
}

impl AmphipodGame for Maze {
    fn get_moves(&self) -> Vec<(i32, Maze)> {
        let targets: Vec<Point> = self
            .iter()
            .filter_map(|(p, f)| {
                if f.is_empty() { Some(*p) } else {None}}
            ).collect();
        //println!("Empty fields: {}", targets.len());
        self
            .iter()
            .filter(|(p, f)| f.is_player())
            .flat_map(|(p, f)| {
                if p.is_room() {
                    targets
                        .iter()
                        .filter(|t| t.is_hallway())
                        .filter(|t| self.is_reachable(p, t))
                        .map(|t| {
                            let dist = p.manhattan(t) * f.cost();
                            let mut maze = self.clone();
                            maze.insert(*p, '.');
                            maze.insert(*t, *f);
                            (dist, maze)
                        }).collect::<Vec<(i32, Maze)>>()
                } else {
                    let room_targets =
                        self
                            .iter()
                            .filter(|(p, _)| p.is_room())
                            .filter_map(|(Point(x, y), tf)| {
                                if f.get_destination() == *x && tf.is_empty() {
                                    Some(*y)
                                } else {
                                    None
                                }
                            }).collect::<Vec<_>>();
                    if room_targets.len() >= 1 {
                        let max_room_depth_target = *room_targets.iter().max().unwrap();
                        targets
                            .iter()
                            .filter(|t| t.is_room() && t.0 == f.get_destination() && t.1 == max_room_depth_target)
                            .filter(|t| self.is_reachable(p, t))
                            .map(|t| {
                                let dist = p.manhattan(t) * f.cost();
                                let mut maze = self.clone();
                                maze.insert(*p, '.');
                                maze.insert(*t, *f);
                                (dist, maze)
                            }).collect::<Vec<(i32, Maze)>>()
                    } else {
                        Vec::<(i32, Maze)>::new()
                    }
                }
            }).collect()
    }

    fn is_reachable(&self, from: &Point, to: &Point) -> bool {
        if to.is_forbidden() {
            return false;
        }
        match (self.get(from), self.get(to)) {
            (Some(f), Some(t)) => {
                if f.is_player() && t.is_empty() {
                    return if from.is_room() && to.is_hallway() {
                        if to.is_forbidden() {
                            return false;
                        }
                        for y in 1..=from.1-1 {
                            if ! self.get(&Point(from.0, y)).unwrap().is_empty() {
                                //println!("R->H:: Path out of room not clear");
                                return false;
                            }
                        }
                        let dir = (to.0 - from.0).signum();
                        let mut x = from.0;
                        while x != to.0 {
                            x += dir;
                            if !self.get(&Point(x, 1)).unwrap().is_empty() {
                                //println!("R->H:: Path in hallway not clear");
                                return false;
                            }
                        }
                        //println!("R->H:: Found path out room");
                        true
                    } else if from.is_hallway() && to.is_room() {
                        let target_room = f.get_destination();

                        let invalid_room = self.iter().filter(|(Point(x,y), tf)| {
                            *y > 1 && *x == target_room && tf.is_player() && f != *tf
                        }).count() > 0;

                        let no_valid_room_target = self.iter().filter(|(Point(x, y), tf)| {
                            *y > 1 && *x == target_room && (tf.is_empty() || *tf == f)
                        }).count() < 1;
                        if invalid_room || no_valid_room_target {
                            //println!("H->R:: No empty spot in target room");
                            return false;
                        }

                        let dir = (target_room - from.0).signum();
                        let mut x = from.0;
                        while x != target_room {
                            x += dir;
                            if ! self.get(&Point(x, from.1)).unwrap().is_empty() {
                                //println!("H->R:: Path in hallway not clear");
                                return false;
                            }
                        }
                        for y in from.1..=to.1 {
                            if ! self.get(&Point(target_room, y)).unwrap().is_empty() {
                                //println!("H->R:: Path in room not clear");
                                return false;
                            }
                        }
                        //println!("H->R:: Found path out hallway");
                        true
                    } else if from.is_hallway() && to.is_hallway() {
                        false
                    } else {
                        for y in 1..=from.1-1 {
                            if ! self.get(&Point(from.0, y)).unwrap().is_empty() {
                                println!("R->R:: Path out of room not clear");
                                return false;
                            }
                        }
                        let dir = (to.0 - from.0).signum();
                        let mut x = from.0;
                        while x != to.0 {
                            x += dir;
                            if !self.get(&Point(x, 1)).unwrap().is_empty() {
                                println!("R->R:: Path in hallway not clear");
                                return false;
                            }
                        }

                        for y in 1..=to.1 {
                            if ! self.get(&Point(to.0, y)).unwrap().is_empty() {
                                println!("R->R:: Path into new room not clear");
                                return false;
                            }
                        }
                        true
                    }
                }
            },
            _ => {},
        }
        false
    }

    fn is_done(&self) -> bool {
        self
            .iter()
            .filter(|(p, f)| f.is_player())
            .all(|(p, f)| f.get_destination() == p.0 && p.is_room())
    }

    fn to_vec(&self) -> Vec<(Point, char)> {
        let mut v = self.iter().map(|(p, f)| (*p, *f)).collect::<Vec<_>>();
        v.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        v
    }

    fn print(&self) {
        let maxy = *self.iter().map(|(Point(_,y), _)| y).max().unwrap();
        for y in 1..=maxy {
            for x in 0..=12 {
                let c = match self.get(&Point(x,y)) {
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


#[aoc_generator(day23)]
pub fn generator(input: &str) -> Maze {
    input.lines().enumerate().fold(HashMap::<Point, char>::new(), |mut acc, (y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let data = match c {
                '#' | ' ' => None,
                x => Some(x),
            };
            if let Some(field) = data {
                acc.insert(Point(x as i8,y as i8), field);
            }
        });
        acc
    })
}

pub fn run_game(input: &Maze) -> i32 {
    let mut dist = HashMap::<Vec<(Point, char)>, i32>::new();
    let mut q = BinaryHeap::new();
    q.push(State{ cost: 0, maze: input.clone()});

    while let Some(state) = q.pop() {
        if state.maze.is_done() {
            state.maze.print();
            return state.cost;
        }

        if let Some(&c) = dist.get(&state.maze.to_vec()) {
            if c > state.cost {
                //println!("continue");
                continue;
            }
        }

        for (d, m) in state.maze.get_moves() {
            //println!("Next move");
            let next_cost = state.cost + d;
            let &c = dist.get(&m.to_vec()).unwrap_or(&1000000);
            if c > next_cost {
                dist.insert(m.to_vec(), next_cost);
                q.push(State{ cost: next_cost, maze: m});
                //println!("Found target maze");
            }
        }
    }
    69
}


#[aoc(day23, part1)]
pub fn part1(inputs: &Maze) -> i32 {
    inputs.print();
    run_game(inputs)
}

pub fn extend(inputs: &Maze) -> Maze {
    let mut maze = Maze::new();
    inputs.iter().for_each(|(&Point(x,y), f)| {
        let np = match y {
            1 => Point(x,y),
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

#[aoc(day23, part2)]
pub fn part2(inputs: &Maze) -> i32 {
    let maze = extend(inputs);

    maze.print();
    run_game(&maze)
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
