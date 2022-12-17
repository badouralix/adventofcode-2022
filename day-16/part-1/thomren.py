from collections import defaultdict
import functools
import itertools
import re
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        regex = re.compile(
            r"Valve (\w+) has flow rate=(\d*); tunnels? leads? to valves? (.*)"
        )
        valves = set()
        flows = {}
        dist = defaultdict(lambda: 1000)
        for valve, flow, neighbors in regex.findall(s):
            valves.add(valve)
            if flow != "0":
                flows[valve] = int(flow)
            for neighbor in neighbors.split(", "):
                dist[valve, neighbor] = 1

        # Floyd-Warshall to compute all pairwise distances
        for k, i, j in itertools.product(valves, valves, valves):
            dist[i, j] = min(dist[i, j], dist[i, k] + dist[k, j])

        @functools.cache
        def solve(minutes, start="AA", remaining_valves=frozenset(flows.keys())):
            res = max(
                [
                    flows[valve] * (minutes - dist[start, valve] - 1)
                    + solve(
                        minutes - dist[start, valve] - 1,
                        valve,
                        remaining_valves - {valve},
                    )
                    for valve in remaining_valves
                    if dist[start, valve] + 1 < minutes
                ]
                + [0]
            )
            return res

        return solve(30)


def test_thomren():
    """
    Run `python -m pytest ./day-16/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
""".strip()
        )
        == 1651
    )
