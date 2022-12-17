from tool.runners.python import SubmissionPy

import re


def manhattan(sensor_x, sensor_y, beacon_x, beacon_y):
    return abs(sensor_x - beacon_x) + abs(sensor_y - beacon_y)


class ThChSubmission(SubmissionPy):
    def run(self, s, beacon_line=2000000):
        """
        :param s: input in string format
        :return: solution flag
        """
        x_without_beacon = set()
        beacons_on_line = set()
        for line in s.splitlines():
            matches = re.search(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
                line,
            )
            sensor_x = int(matches.group(1))
            sensor_y = int(matches.group(2))
            beacon_x = int(matches.group(3))
            beacon_y = int(matches.group(4))

            if beacon_y == beacon_line:
                beacons_on_line.add(beacon_x)

            distance = manhattan(sensor_x, sensor_y, beacon_x, beacon_y)
            if sensor_y - distance <= beacon_line <= sensor_y + distance:
                dx = distance - abs(beacon_line - sensor_y)
                x_min = sensor_x - dx
                x_max = sensor_x + dx
                x_without_beacon.update(range(x_min, x_max + 1))

        return len(x_without_beacon - beacons_on_line)


def test_th_ch():
    """
    Run `python -m pytest ./day-15/part-1/th-ch.py` to test the submission.
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
            beacon_line=10,
        )
        == 26
    )
