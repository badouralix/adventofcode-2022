from tool.runners.python import SubmissionPy

from collections import defaultdict


def dfs(opened_valves, dist, valves, current_valve, t):
    if current_valve not in opened_valves:
        opened_valves[current_valve] = t
    if t >= 30:
        score = sum(
            (30 - opening) * valves[opened_valve]["flow_rate"]
            for opened_valve, opening in opened_valves.items()
        )
        return score

    valves_worth_opening = [
        possible_valve
        for possible_valve in valves.keys()
        if valves[possible_valve]["flow_rate"] > 0
        and not possible_valve in opened_valves
        and t + dist[current_valve][possible_valve] + 1 <= 30
    ]
    if not valves_worth_opening:
        return dfs(opened_valves, dist, valves, current_valve, 30)
    return max(
        dfs(
            opened_valves.copy(),
            dist,
            valves,
            next_valve,
            t + dist[current_valve][next_valve] + 1,
        )
        for next_valve in valves_worth_opening
    )


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        valves = {}
        for line in s.splitlines():
            valve = line[6:8]
            line = line[23:]
            flow_rate, tunnels = line.split(";")
            flow_rate = int(flow_rate)
            tunnels = (
                tunnels.removeprefix(" tunnels lead to valves ")
                .removeprefix(" tunnel leads to valve ")
                .split(", ")
            )
            valves[valve] = {"flow_rate": flow_rate, "tunnels": tunnels}

        # Floyd–Warshall
        dist = defaultdict(lambda: defaultdict(lambda: float("inf")))
        for valve in valves.keys():
            dist[valve][valve] = 0
            for next_valve in valves[valve]["tunnels"]:
                dist[valve][next_valve] = 1
        for k in valves.keys():
            for i in valves.keys():
                for j in valves.keys():
                    dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])

        return dfs({}, dist, valves, current_valve="AA", t=0)


def test_th_ch():
    """
    Run `python -m pytest ./day-16/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
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
