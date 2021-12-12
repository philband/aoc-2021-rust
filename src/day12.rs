use std::collections::{HashSet, HashMap};
type Data = HashMap<String, Cave>;

#[derive(Clone, Debug)]
pub struct Cave {
    connections: HashSet<String>,
    big: bool,
}

impl Cave {
    fn new(name: String) -> Self {
        Self {
            connections: HashSet::new(),
            big: name.chars().all(|c| c.is_uppercase())
        }
    }

    fn connect_to(&mut self, name: String) {
        self.connections.insert(name);
    }

    fn is_big(&self) -> bool {
        self.big
    }

    fn is_small(&self) -> bool {
        ! self.big
    }
}



#[aoc_generator(day12)]
pub fn generator(input: &str) -> Data {
    let mut map: Data = HashMap::new();
    input
        .lines()
        .for_each(|l| {
            let mut caves = l.split('-');
            let (from, to) = (caves.next().unwrap(), caves.next().unwrap());
            map.entry( from.to_string()).or_insert(Cave::new(from.to_string())).connect_to(to.to_string());
            map.entry(to.to_string()).or_insert(Cave::new(to.to_string())).connect_to(from.to_string());
        });
    map.clone()
}


fn visit(c: &Cave, caves: &Data, visited: &mut Vec<String>, double_visit: bool) -> usize {
    c.connections.iter().map(|path| {
        match path.as_str() {
            "end" => 1,
            "start" => 0,
            _ => {
                if double_visit && visited.contains(path) {
                    0
                } else {
                    let double = visited.contains(path) || double_visit;
                    let target = caves.get(path).unwrap();
                    if target.is_small() {
                        visited.push(path.to_string())
                    }
                    let res = visit(target, caves, visited, double);
                    if target.is_small() {
                        visited.pop();
                    }
                    res
                }

            }
        }
    }).sum()
}


#[aoc(day12, part1)]
pub fn part1(map: &Data) -> usize {
    let mut visited: Vec<String> = Vec::new();
    visit(map.get("start").unwrap(), map, &mut visited, true)
}

#[aoc(day12, part2)]
pub fn part2(map: &Data) -> usize {
    let mut visited: Vec<String> = Vec::new();
    visit(map.get("start").unwrap(), map, &mut visited, false)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("start-A
start-b
A-c
A-b
b-d
A-end
b-end");

        assert_eq!(part1(&input), 10);

        let input = generator("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc");
        assert_eq!(part1(&input), 19);

        let input = generator("fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW");
        assert_eq!(part1(&input), 226)
    }

    #[test]
    pub fn test2() {
        let input = generator("start-A
start-b
A-c
A-b
b-d
A-end
b-end");

        assert_eq!(part2(&input), 36);

        let input = generator("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc");
        assert_eq!(part2(&input), 103);

        let input = generator("fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW");
        assert_eq!(part2(&input), 3509)
    }
}
