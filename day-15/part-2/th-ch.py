from tool.runners.python import SubmissionPy

from collections import defaultdict
from importlib import import_module
import re

part1 = import_module("day-15.part-1.th-ch")


def find_point_not_in_intervals(intervals, max_beacon_coord):
    sorted_intervals = sorted(intervals)
    candidate = 0
    for x, y in sorted_intervals:
        if candidate < x:
            return candidate
        candidate = max(candidate, y + 1)
    if candidate <= max_beacon_coord:
        return candidate
    return None


class ThChSubmission(SubmissionPy):
    def run(self, s, max_beacon_coord=4000000):
        """
        :param s: input in string format
        :return: solution flag
        """
        sensors_with_scope = {}
        intervals_by_line = defaultdict(list)
        for line in s.splitlines():
            matches = re.search(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
                line,
            )
            sensor_x = int(matches.group(1))
            sensor_y = int(matches.group(2))
            beacon_x = int(matches.group(3))
            beacon_y = int(matches.group(4))

            dist = part1.manhattan(sensor_x, sensor_y, beacon_x, beacon_y)
            for y in range(
                max(0, sensor_y - dist), min(sensor_y + dist, max_beacon_coord) + 1
            ):
                dx = dist - abs(y - sensor_y)
                x_min = max(0, sensor_x - dx)
                x_max = min(sensor_x + dx, max_beacon_coord)
                intervals_by_line[y].append((x_min, x_max))

        for y, intervals in intervals_by_line.items():
            x = find_point_not_in_intervals(intervals, max_beacon_coord)
            if x is not None:
                return x * 4000000 + y


def test_th_ch():
    """
    Run `python -m pytest ./day-15/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
""".strip(),
            max_beacon_coord=20,
        )
        == 56000011
    )
