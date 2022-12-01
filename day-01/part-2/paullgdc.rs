fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let mut load = 0;
    let mut maxs = [0; 3];
    for l in aoc::paullgdc::LastElem::new(input.lines()).flat_map(|(line, last)| {
        let line = line.trim();
        if line.is_empty() {
            let total = load;
            load = 0;
            return Some(total);
        }
        load += line.parse::<usize>().unwrap();
        last.then_some(load)
    }) {
        if l > maxs[0] {
            (maxs[2], maxs[1]) = (maxs[1], maxs[0]);
            maxs[0] = l
        } else if l > maxs[1] {
            maxs[2] = maxs[1];
            maxs[1] = l
        } else if l > maxs[2] {
            maxs[2] = l
        }
    }
    maxs[0] + maxs[1] + maxs[2]
}
