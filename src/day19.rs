use scan_fmt::scan_fmt;

use nalgebra::*;

type Data = Vec<Scanner>;


#[derive(Debug, PartialEq, Clone)]
pub struct Scanner {
    id: i32,
    pos: Vector3<f64>,
    beacons: Matrix3xX<f64>,
    transformation: Matrix3<f64>,
}

impl Scanner {
    pub fn centroid(&self) -> Vector3<f64> {
        self.beacons.column_mean().into()
    }
}


#[aoc_generator(day19)]
pub fn generator(input: &str) -> Data {
    input.split("\n\n").map(|block| {
        let mut it = block.lines().into_iter();
        let scanner = it.next().unwrap();
        let id = scan_fmt!(scanner, "--- scanner {} ---", i32).unwrap();
        let size = block.lines().count() - 1;
        let beacons: Matrix3xX<f64> = it.map(|l| scan_fmt!(l, "{},{},{}", f64, f64, f64).unwrap())
            .enumerate()
            .fold(Matrix3xX::zeros(size), |mut acc, (n, (x,y,z))| {
                acc[(0, n)] = x;
                acc[(1, n)] = y;
                acc[(2, n)] = z;
                acc
            });
        Scanner{
            id,
            pos: Vector3::new(0.0,0.0,0.0),
            beacons,
            transformation: Matrix3::default(),
        }

    }).collect()
}

#[aoc(day19, part1)]
pub fn part1(inputs: &Data) -> usize {
    let mut it = inputs.iter();
    let s0 = it.next().unwrap();
    let c0 = s0.centroid();
    let mut m0 = Matrix3xX::zeros(s0.beacons.ncols());
    m0.fill_row(0, c0.x);
    m0.fill_row(1, c0.y);
    m0.fill_row(2, c0.z);
    let m00 = m0 - s0.clone().beacons;
    it.map(|s| {
        let cx = s.centroid();
        let mut mx = Matrix3xX::zeros(s.beacons.ncols());
        mx.fill_row(0, cx.x);
        mx.fill_row(1, cx.y);
        mx.fill_row(2, cx.z);

        let mxx = mx - s.clone().beacons;

        let h = m00.clone() * mxx.transpose();
        let x = SVD::new(h, true, true);
        let r = x.v_t.unwrap().transpose() * x.u.unwrap().transpose();

        let t = cx -r * c0;



        println!("{:?}", r);
        r
    }).collect::<Vec<_>>();
    69
}

#[aoc(day19, part2)]
pub fn part2(inputs: &Data) -> usize {
    69
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14");
        assert_eq!(part1(&input), 79);
    }


    #[test]
    pub fn test2() {
        let input = generator("");

        assert_eq!(part2(&input), 3993);
    }

}
