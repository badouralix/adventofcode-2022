use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::env::args;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let graph = ValvesGraph::from_str(input).unwrap();
    let nonzero_valves = graph
        .flows
        .iter()
        .enumerate()
        .filter(|&(_, f)| *f != 0)
        .map(|(i, _)| i)
        .collect::<BTreeSet<usize>>();
    let start = State {
        minutes: 30,
        position: *graph.label_to_idx.get("AA").unwrap(),
        remaining_valves: nonzero_valves,
    };
    graph.solve(start)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    minutes: usize,
    position: usize,
    remaining_valves: BTreeSet<usize>,
}

std::thread_local! {
    static SOLVE_MEMO: RefCell<HashMap<State, usize>> = RefCell::new(HashMap::new());
}

impl ValvesGraph {
    fn solve(&self, state: State) -> usize {
        let dist = &self.adjacency;
        let flows = &self.flows;
        let r = SOLVE_MEMO.with(|hm| {
            let hm = hm.borrow_mut();
            hm.get(&state).cloned()
        });
        if let Some(p) = r {
            return p;
        };
        let res = state
            .remaining_valves
            .iter()
            .filter(|&&valve| dist[state.position][valve] + 1 < state.minutes)
            .map(|&valve| {
                *flows.get(valve).unwrap() * (state.minutes - dist[state.position][valve] - 1)
                    + self.solve(State {
                        minutes: state.minutes - dist[state.position][valve] - 1,
                        position: valve,
                        remaining_valves: state
                            .remaining_valves
                            .difference(&BTreeSet::from_iter([valve]))
                            .cloned()
                            .collect::<BTreeSet<usize>>(),
                    })
            })
            .max()
            .unwrap_or(0);
        SOLVE_MEMO.with(|hm| {
            let mut hm = hm.borrow_mut();
            hm.insert(state, res);
        });
        res
    }
}

#[derive(Debug)]
struct ValvesGraph {
    flows: Vec<usize>,
    adjacency: Vec<Vec<usize>>,
    node_labels: Vec<String>,
    label_to_idx: HashMap<String, usize>,
}

impl FromStr for ValvesGraph {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get all valves and their flow rate
        let mut label_to_idx: HashMap<String, usize> = HashMap::new();
        let mut node_labels: Vec<String> = vec![];
        let mut flows: Vec<usize> = vec![];
        for line in s.lines() {
            let tokens = line.split(' ').collect::<Vec<&str>>();
            let node = tokens[1];
            let flow = tokens[4][5..tokens[4].len() - 1].parse::<usize>()?;
            label_to_idx.insert(node.to_string(), flows.len());
            node_labels.push(node.to_string());
            flows.push(flow);
        }

        // Build the adjacency matrix
        let mut adjacency = vec![vec![usize::MAX; flows.len()]; flows.len()];
        for line in s.lines() {
            let tokens = line.split(' ').collect::<Vec<&str>>();
            let node = tokens[1];
            let node_idx = *label_to_idx.get(node).ok_or("unknown node")?;
            for &token in &tokens[9..] {
                let neighbor = token.trim_end_matches(',');
                let neighbor_idx = *label_to_idx.get(neighbor).ok_or("unknown neighbor")?;
                adjacency[node_idx][neighbor_idx] = 1;
            }
        }

        // Create edges between all node pairs with weights corresponding to the
        // shortest path length. Use the Floyd-Warshall algorithm to compute
        // all shortest distances in place.
        for k in 0..flows.len() {
            for i in 0..flows.len() {
                for j in 0..flows.len() {
                    adjacency[i][j] =
                        adjacency[i][j].min(adjacency[i][k].saturating_add(adjacency[k][j]));
                }
            }
        }

        Ok(Self {
            flows,
            adjacency,
            node_labels,
            label_to_idx,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"),
            1651,
        )
    }
}
