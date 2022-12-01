fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let mut load = 0;
    let mut loads = aoc::paullgdc::LastElem::new(input.lines()).flat_map(|(line, last)| {
        let line = line.trim();
        if line.is_empty() {
            let total = load;
            load = 0;
            Some(total)
        } else if last {
            Some(load)
        } else {
            load += line.parse::<usize>().unwrap();
            None
        }
    }).collect::<Vec<_>>();
    loads.sort();
    loads[loads.len() - 3..].into_iter().sum()
}
