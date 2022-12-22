from tool.runners.python import SubmissionPy

from collections import deque
import re


class Blueprint:
    # order of costs, robots, stocks: ore, clay, obsidian, geode
    def __init__(self, costs, robots, stocks):
        self.costs = costs
        self.robots = robots  # [1, 0, 0, 0]
        self.stocks = stocks  # [0, 0, 0, 0]

        # Useless to have more robots than what can be built in 1min
        self.max_ore_robots = sum(cost[0] for cost in self.costs)
        self.max_clay_robots = self.costs[2][1]
        self.max_obsidian_robots = self.costs[3][2]

    def __eq__(self, other):
        return self.robots == other.robots and self.stocks == other.stocks

    def __hash__(self):
        return hash((frozenset(self.robots), frozenset(self.stocks)))

    def __str__(self):
        return "robots: {} - stocks: {}".format(self.robots, self.stocks)

    def upper_bound(self, t, max_time):
        time_left = max_time - t
        return (
            # current stock
            self.stocks[-1]
            # what current robots will produce in the time left
            + self.robots[-1] * time_left
            # best case: we build a geode robot each minute
            + time_left * (time_left - 1) // 2
        )

    def possible_moves_for_the_minute(self):
        # each existing robot can harvest 1 resource
        updated_stocks = [sum(x) for x in zip(self.stocks, self.robots)]

        next_blueprints = []

        can_build_geode_robot = (
            self.stocks[0] >= self.costs[3][0] and self.stocks[2] >= self.costs[3][2]
        )
        if can_build_geode_robot:
            robots = self.robots[:]
            robots[3] += 1
            stocks = updated_stocks[:]
            stocks[0] -= self.costs[3][0]
            stocks[2] -= self.costs[3][2]
            next_blueprint = Blueprint(self.costs, robots, stocks)
            next_blueprints.append(next_blueprint)

        next_blueprints.append(Blueprint(self.costs, self.robots[:], updated_stocks[:]))

        can_build_obsidian_robot = (
            self.robots[2] < self.max_obsidian_robots
            and self.stocks[0] >= self.costs[2][0]
            and self.stocks[1] >= self.costs[2][1]
        )
        if can_build_obsidian_robot:
            robots = self.robots[:]
            robots[2] += 1
            stocks = updated_stocks[:]
            stocks[0] -= self.costs[2][0]
            stocks[1] -= self.costs[2][1]
            next_blueprint = Blueprint(self.costs, robots, stocks)
            next_blueprints.append(next_blueprint)

        can_build_clay_robot = (
            self.robots[1] < self.max_clay_robots and self.stocks[0] >= self.costs[1][0]
        )
        if can_build_clay_robot:
            robots = self.robots[:]
            robots[1] += 1
            stocks = updated_stocks[:]
            stocks[0] -= self.costs[1][0]
            next_blueprint = Blueprint(self.costs, robots, stocks)
            next_blueprints.append(next_blueprint)

        can_build_ore_robot = (
            self.robots[0] < self.max_ore_robots and self.stocks[0] >= self.costs[0][0]
        )
        if can_build_ore_robot:
            robots = self.robots[:]
            robots[0] += 1
            stocks = updated_stocks[:]
            stocks[0] -= self.costs[0][0]
            next_blueprint = Blueprint(self.costs, robots, stocks)
            next_blueprints.append(next_blueprint)

        return next_blueprints


def bfs(root_blueprint, max_time=24):
    q = deque()
    seen = set()
    seen.add((0, root_blueprint))
    q.append((0, root_blueprint))
    current_max = 0
    while q:
        t, blueprint = q.popleft()
        if t >= max_time:
            current_max = max(current_max, blueprint.stocks[-1])
            continue

        for next_blueprint in blueprint.possible_moves_for_the_minute():
            if (
                next_blueprint.upper_bound(t + 1, max_time) > current_max
                and (t + 1, next_blueprint) not in seen
            ):
                seen.add((t + 1, next_blueprint))
                q.appendleft((t + 1, next_blueprint))

    return current_max


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        quality_levels = 0
        for line in s.splitlines():
            matches = re.search(
                r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
                line,
            )
            blueprint_id = int(matches.group(1))
            costs = [
                [int(matches.group(2)), 0, 0],
                [int(matches.group(3)), 0, 0],
                [int(matches.group(4)), int(matches.group(5)), 0],
                [int(matches.group(6)), 0, int(matches.group(7))],
            ]
            blueprint = Blueprint(costs, [1, 0, 0, 0], [0, 0, 0, 0])
            quality_level = bfs(blueprint)
            quality_levels += blueprint_id * quality_level

        return quality_levels


def test_th_ch():
    """
    Run `python -m pytest ./day-19/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
""".strip()
        )
        == 33
    )
