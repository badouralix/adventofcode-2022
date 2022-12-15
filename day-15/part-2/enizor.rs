use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"), 4000000);
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_line(input: &str) -> (isize, isize, isize, isize) {
    // Sensor at x=9, y=16: closest beacon is at x=10, y=16
    let p1 = 12;
    let p2 = p1 + input[p1..].find(',').unwrap();
    let x1 = input[p1..p2].parse().unwrap();
    let p3 = p2 + 4;
    let p4 = p3 + input[p3..].find(':').unwrap();
    let y1 = input[p3..p4].parse().unwrap();
    let p5 = p4 + 25;
    let p6 = p5 + input[p5..].find(',').unwrap();
    let x2 = input[p5..p6].parse().unwrap();
    let y2 = input[p6 + 4..].parse().unwrap();
    (x1, y1, x2, y2)
}

fn manhattan_dist(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

/// A disk centerd on a sensor,
/// representing an area where the missing beacon cannot be.
#[derive(Debug, Clone, Copy)]
struct Disk {
    x: isize,
    y: isize,
    radius: isize,
}

impl Disk {
    fn contains(&self, p: (isize, isize)) -> bool {
        manhattan_dist(p.0, p.1, self.x, self.y) <= self.radius
    }

    /// Constructs, if possible, the line neighboring both self and other
    fn tangent(&self, other: &Self) -> Option<Tangent> {
        let d = manhattan_dist(self.x, self.y, other.x, other.y);
        if d == self.radius + other.radius + 2 {
            let dx = (other.x - self.x).signum();
            let dy = (other.y - self.y).signum();
            if self.x == other.x {
                Some(Tangent {
                    start: (self.x, self.y + dy * (self.radius + 1)),
                    end: (self.x, self.y + dy * (self.radius + 1)),
                })
            } else if self.y == other.y {
                Some(Tangent {
                    start: (self.x + dx * (self.radius + 1), self.y),
                    end: (self.x + dx * (self.radius + 1), self.y),
                })
            } else {
                let mut start = (self.x, self.y + dy * (self.radius + 1));
                while manhattan_dist(other.x, other.y, start.0, start.1) > other.radius + 1 {
                    start.0 += dx;
                    start.1 -= dy;
                }
                let mut end = (self.x + dx * (self.radius + 1), self.y);
                while manhattan_dist(other.x, other.y, end.0, end.1) > other.radius + 1 {
                    end.0 -= dx;
                    end.1 += dy;
                }
                Some(Tangent { start, end })
            }
        } else {
            None
        }
    }
}

/// A line start -> end that is neighboring at least 2 disks
#[derive(Debug, Clone, Copy)]
struct Tangent {
    start: (isize, isize),
    end: (isize, isize),
}

impl Tangent {
    fn direction(&self) -> (isize, isize) {
        (
            (self.end.0 - self.start.0).signum(),
            (self.end.1 - self.start.1).signum(),
        )
    }

    fn cut(&self, d: &Disk) -> Option<Tangent> {
        match (d.contains(self.start), d.contains(self.end)) {
            (true, true) => None,
            (false, false) => Some(*self),
            (true, false) => {
                let dir = self.direction();
                let mut start = self.start;
                while d.contains(start) {
                    start.0 += dir.0;
                    start.1 += dir.1;
                }
                Some(Tangent {
                    start,
                    end: self.end,
                })
            }
            (false, true) => {
                let dir = self.direction();
                let mut end = self.end;
                while d.contains(end) {
                    end.0 -= dir.0;
                    end.1 -= dir.1;
                }
                Some(Tangent {
                    start: self.start,
                    end,
                })
            }
        }
    }
}

fn run(input: &str, max: isize) -> isize {
    let mut disks = Vec::new();
    for line in input.lines() {
        let (x, y, x2, y2) = parse_line(line);
        let radius = manhattan_dist(x, y, x2, y2);
        // hypothesis: all sensors are in [0, max]
        // => all tangents are inside [0, max]
        assert!(x >= 0);
        assert!(y >= 0);
        assert!(x <= max);
        assert!(y <= max);
        disks.push(Disk { x, y, radius });
    }
    // if the solution is in ]0, max[
    // then all of its neightbors are in a sensor's radius
    // Since a given sensor disk can only cover two of its sides,
    // it must be in a "Tangent" of two dofferent disks
    for i in 0..disks.len() - 1 {
        for j in i + 1..disks.len() {
            let mut opt_tan = disks[i].tangent(&disks[j]);
            let mut k = 0;
            while opt_tan.is_some() && k < disks.len() {
                opt_tan = opt_tan.unwrap().cut(&disks[k]);
                k += 1;
            }
            if let Some(t) = opt_tan {
                // sanity check unicity of solution
                assert_eq!(t.start, t.end);
                return t.start.0 * 4000000 + t.start.1;
            }
        }
    }
    // if the solution has only a single disk for neighbors
    // then it covers only 2 sides
    // the 2 others must be "covered" by the cave border -1 / max+1
    // i.e. it's a corner
    for x in [0, max] {
        for y in [0, max] {
            let mut solution = true;
            for d in &disks {
                if d.contains((x, y)) {
                    solution = false;
                    break;
                }
            }
            if solution {
                return x * 4000000 + y;
            }
        }
    }
    panic!("No solution was found :(")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(
                "Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                20
            ),
            56000011
        )
    }
}
