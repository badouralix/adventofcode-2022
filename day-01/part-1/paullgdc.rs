fn main() {
    aoc::run(run)
}

fn run(input: &str) -> usize {
    let mut load = 0;
    aoc::paullgdc::LastElem::new(input.lines())
        .flat_map(|(line, last)| {
            let line = line.trim();
            if line.is_empty() {
                let total = load;
                load = 0;
                return Some(total)
            }
            load += line.parse::<usize>().unwrap();
            last.then_some(load)
        })
        .max()
        .unwrap()
}
