from tool.runners.python import SubmissionPy


class Interval:
    def __init__(self, start, end):
        self.start = start
        self.end = end

    def __repr__(self):
        return f"[{self.start} - {self.end}]"

    def __add__(self, other):
        if other.start > self.end + 1 or self.start > other.end + 1:
            raise ValueError("Intervals do not meet")
        return Interval(min(self.start, other.start), max(self.end, other.end))

    def __iter__(self):
        yield self

    def __eq__(self, other):
        return self.start <= other <= self.end


class YouyounSubmission(SubmissionPy):
    def run(self, s, max_coord=4000000):
        """
        :param s: input in string format
        :return: solution flag
        """
        y_interval_map = {}
        for line in s.replace("Sensor at x=", "").replace(": closest beacon is at x=", " ").replace(", y=",
                                                                                                    " ").split(
            "\n"):
            sensor_x, sensor_y, beacon_x, beacon_y = list(map(int, line.split(" ")))
            dist = abs(sensor_x - beacon_x) + abs(sensor_y - beacon_y)
            for y_interest in range(max(0, sensor_y - dist), min(sensor_y + dist, max_coord)):
                pos_from_y = abs(sensor_y - y_interest)
                current_interval = Interval(max(0, sensor_x - dist + pos_from_y),
                                            min(max_coord, sensor_x + dist - pos_from_y))
                if y_interest not in y_interval_map:
                    y_interval_map[y_interest] = [current_interval]
                else:
                    y_interval_map[y_interest].append(current_interval)
        for y in y_interval_map:
            intervals = list(sorted(y_interval_map[y], key=lambda x: x.start))
            I = intervals[0]
            for I2 in intervals[1:]:
                try:
                    I = I + I2
                except ValueError:
                    return (I2.start - 1) * 4000000 + y


def test_youyoun():
    """
    Run `python -m pytest ./day-15/part-1/youyoun.py` to test the submission.
    """
    assert (
            YouyounSubmission().run(
                """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
""".strip(), 20
            )
            == 56000011
    )


if __name__ == "__main__":
    test_youyoun()
