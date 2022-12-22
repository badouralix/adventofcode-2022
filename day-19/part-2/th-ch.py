from tool.runners.python import SubmissionPy

from importlib import import_module
import re

part1 = import_module("day-19.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        multiplied = 1
        for line in s.splitlines()[:3]:
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
            blueprint = part1.Blueprint(costs, [1, 0, 0, 0], [0, 0, 0, 0])
            quality_level = part1.bfs(blueprint, max_time=32)
            multiplied *= quality_level

        return multiplied


def test_th_ch():
    """
    Run `python -m pytest ./day-19/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
""".strip()
        )
        == 56 * 62
    )
