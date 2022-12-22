from tool.runners.python import SubmissionPy

from collections import deque
import re
import math


class Blueprint:
    # order of costs, robots, stocks: ore, clay, obsidian, geode
    def __init__(self, costs, robots, stocks, t):
        self.costs = costs
        self.robots = robots  # [1, 0, 0, 0]
        self.stocks = stocks  # [0, 0, 0, 0]
        self.t = t

        # Useless to have more robots than what can be built in 1min
        self.max_ore_robots = sum(cost[0] for cost in self.costs)
        self.max_clay_robots = self.costs[2][1]
        self.max_obsidian_robots = self.costs[3][2]

    def __eq__(self, other):
        return (
            self.t == other.t
            and self.robots == other.robots
            and self.stocks == other.stocks
        )

    def __hash__(self):
        return hash((self.t, frozenset(self.robots), frozenset(self.stocks)))

    def __str__(self):
        return "t={} - robots: {} - stocks: {}".format(self.t, self.robots, self.stocks)

    def upper_bound(self, max_time):
        time_left = max_time - self.t
        return (
            # current stock
            self.stocks[-1]
            # what current robots will produce in the time left
            + self.robots[-1] * time_left
            # best case: we build a geode robot each minute
            + (time_left * (time_left - 1)) // 2
        )

    def possible_moves_for_the_minute(self, max_time):
        next_blueprints = []

        if self.robots[2] > 0:
            nb_minutes_for_geode_robot = max(
                0,
                math.ceil((self.costs[3][0] - self.stocks[0]) / self.robots[0]),
                math.ceil((self.costs[3][2] - self.stocks[2]) / self.robots[2]),
            )
            if self.t + nb_minutes_for_geode_robot < max_time:
                robots = self.robots[:]
                stocks = [
                    stock + (1 + nb_minutes_for_geode_robot) * self.robots[i]
                    for i, stock in enumerate(self.stocks)
                ]
                robots[3] += 1
                stocks[0] = stocks[0] - self.costs[3][0]
                stocks[2] = stocks[2] - self.costs[3][2]
                next_blueprint = Blueprint(
                    self.costs, robots, stocks, self.t + 1 + nb_minutes_for_geode_robot
                )
                next_blueprints.append(next_blueprint)

        if self.robots[2] < self.max_obsidian_robots and self.robots[1] > 0:
            nb_minutes_for_obsidian_robot = max(
                0,
                math.ceil((self.costs[2][0] - self.stocks[0]) / self.robots[0]),
                math.ceil((self.costs[2][1] - self.stocks[1]) / self.robots[1]),
            )
            if self.t + nb_minutes_for_obsidian_robot < max_time:
                robots = self.robots[:]
                stocks = [
                    stock + (1 + nb_minutes_for_obsidian_robot) * self.robots[i]
                    for i, stock in enumerate(self.stocks)
                ]
                robots[2] += 1
                stocks[0] = stocks[0] - self.costs[2][0]
                stocks[1] = stocks[1] - self.costs[2][1]
                next_blueprint = Blueprint(
                    self.costs,
                    robots,
                    stocks,
                    self.t + 1 + nb_minutes_for_obsidian_robot,
                )
                next_blueprints.append(next_blueprint)

        if self.robots[1] < self.max_clay_robots:
            nb_minutes_for_clay_robot = max(
                0,
                math.ceil((self.costs[1][0] - self.stocks[0]) / self.robots[0]),
            )
            if self.t + nb_minutes_for_clay_robot < max_time:
                robots = self.robots[:]
                stocks = [
                    stock + (1 + nb_minutes_for_clay_robot) * self.robots[i]
                    for i, stock in enumerate(self.stocks)
                ]
                robots[1] += 1
                stocks[0] = stocks[0] - self.costs[1][0]
                next_blueprint = Blueprint(
                    self.costs, robots, stocks, self.t + 1 + nb_minutes_for_clay_robot
                )
                next_blueprints.append(next_blueprint)

        if self.robots[0] < self.max_ore_robots:
            nb_minutes_for_ore_robot = max(
                0,
                math.ceil((self.costs[0][0] - self.stocks[0]) / self.robots[0]),
            )
            if self.t + nb_minutes_for_ore_robot < max_time:
                robots = self.robots[:]
                stocks = [
                    stock + (1 + nb_minutes_for_ore_robot) * self.robots[i]
                    for i, stock in enumerate(self.stocks)
                ]
                robots[0] += 1
                stocks[0] = stocks[0] - self.costs[0][0]
                next_blueprint = Blueprint(
                    self.costs, robots, stocks, self.t + 1 + nb_minutes_for_ore_robot
                )
                next_blueprints.append(next_blueprint)

        return next_blueprints


def get_max_geodes(root_blueprint, max_time=24):
    q = deque()
    seen = set()
    seen.add(root_blueprint)
    q.append(root_blueprint)
    current_max = 0
    while q:
        blueprint = q.popleft()
        if blueprint.t >= max_time:
            current_max = max(current_max, blueprint.stocks[-1])
            continue

        # if we stop building robots
        current_max = max(
            current_max,
            blueprint.stocks[-1] + (max_time - blueprint.t) * blueprint.robots[-1],
        )

        for next_blueprint in blueprint.possible_moves_for_the_minute(max_time):
            if (
                next_blueprint.upper_bound(max_time) > current_max
                and next_blueprint not in seen
            ):
                seen.add(next_blueprint)
                q.appendleft(next_blueprint)

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
            blueprint = Blueprint(costs, [1, 0, 0, 0], [0, 0, 0, 0], 0)
            quality_level = get_max_geodes(blueprint)
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
