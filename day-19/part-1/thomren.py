from dataclasses import astuple, dataclass
from functools import cached_property
from math import ceil
import re
from typing import Iterable, List, Tuple
from tool.runners.python import SubmissionPy

N_MINUTES = 24


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        return sum(
            Blueprint(*parse_ints(line)).quality(N_MINUTES) for line in s.splitlines()
        )


def parse_ints(s: str) -> Iterable[int]:
    return map(int, re.findall(r"\d+", s))


@dataclass(frozen=True)
class Blueprint:
    bp_id: int
    ore_ore_cost: int
    clay_ore_cost: int
    obsidian_ore_cost: int
    obsidian_clay_cost: int
    geode_ore_cost: int
    geode_obsidian_cost: int

    @cached_property
    def max_ore_cost(self):
        return max(
            self.ore_ore_cost,
            self.clay_ore_cost,
            self.obsidian_ore_cost,
            self.geode_ore_cost,
        )

    def max_geodes(self, minutes: int) -> int:
        stack: List[Tuple[int, int, int, int, int, int, int, int, int]] = [
            (minutes, 0, 0, 0, 0, 1, 0, 0, 0)
        ]
        best = 0
        while len(stack) > 0:
            (
                minutes,
                ore,
                clay,
                obsidian,
                geodes,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots,
            ) = stack.pop()

            best = max(best, geodes)

            if minutes == 0:
                continue

            # no need to build more robots that what can be consumed in one minute
            if ore_robots < self.max_ore_cost:
                missing_ore = max(0, self.ore_ore_cost - ore)
                missing_minutes = ceil(missing_ore / ore_robots)
                s = (
                    minutes - (1 + missing_minutes),
                    ore + ore_robots * (1 + missing_minutes) - self.ore_ore_cost,
                    clay + clay_robots * (1 + missing_minutes),
                    obsidian + obsidian_robots * (1 + missing_minutes),
                    geodes + geode_robots * (1 + missing_minutes),
                    ore_robots + 1,
                    clay_robots,
                    obsidian_robots,
                    geode_robots,
                )
                if s[0] >= 0 and upper_bound(*s) > best:
                    stack.append(s)
            if clay_robots < self.obsidian_clay_cost:
                missing_ore = max(0, self.clay_ore_cost - ore)
                missing_minutes = ceil(missing_ore / ore_robots)
                s = (
                    minutes - (1 + missing_minutes),
                    ore + ore_robots * (1 + missing_minutes) - self.clay_ore_cost,
                    clay + clay_robots * (1 + missing_minutes),
                    obsidian + obsidian_robots * (1 + missing_minutes),
                    geodes + geode_robots * (1 + missing_minutes),
                    ore_robots,
                    clay_robots + 1,
                    obsidian_robots,
                    geode_robots,
                )
                if s[0] >= 0 and upper_bound(*s) > best:
                    stack.append(s)
            if clay_robots > 0 and obsidian_robots < self.geode_obsidian_cost:
                missing_ore = max(0, self.obsidian_ore_cost - ore)
                missing_clay = max(0, self.obsidian_clay_cost - clay)
                missing_minutes = max(
                    ceil(missing_ore / ore_robots),
                    ceil(missing_clay / clay_robots),
                )
                s = (
                    minutes - (1 + missing_minutes),
                    ore + ore_robots * (1 + missing_minutes) - self.obsidian_ore_cost,
                    clay
                    + clay_robots * (1 + missing_minutes)
                    - self.obsidian_clay_cost,
                    obsidian + obsidian_robots * (1 + missing_minutes),
                    geodes + geode_robots * (1 + missing_minutes),
                    ore_robots,
                    clay_robots,
                    obsidian_robots + 1,
                    geode_robots,
                )
                if s[0] >= 0 and upper_bound(*s) > best:
                    stack.append(s)
            if obsidian_robots > 0:
                missing_ore = max(0, self.geode_ore_cost - ore)
                missing_obsidian = max(0, self.geode_obsidian_cost - obsidian)
                missing_minutes = max(
                    ceil(missing_ore / ore_robots),
                    ceil(missing_obsidian / obsidian_robots),
                )
                s = (
                    minutes - (1 + missing_minutes),
                    ore + ore_robots * (1 + missing_minutes) - self.geode_ore_cost,
                    clay + clay_robots * (1 + missing_minutes),
                    obsidian
                    + obsidian_robots * (1 + missing_minutes)
                    - self.geode_obsidian_cost,
                    geodes + geode_robots * (1 + missing_minutes),
                    ore_robots,
                    clay_robots,
                    obsidian_robots,
                    geode_robots + 1,
                )
                if s[0] >= 0 and upper_bound(*s) > best:
                    stack.append(s)

        return best

    def quality(self, minutes: int) -> int:
        return self.max_geodes(minutes) * self.bp_id


def upper_bound(
    minutes,
    _ore,
    _clay,
    _obsidian,
    geodes,
    _ore_robots,
    _clay_robots,
    _obsidian_robots,
    geode_robots,
):
    # assume we can build one geode robot per minute until the end
    return geodes + geode_robots * minutes + minutes * (minutes - 1) // 2


def test_thomren():
    """
    Run `python -m pytest ./day-19/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
""".strip()
        )
        == 33
    )
