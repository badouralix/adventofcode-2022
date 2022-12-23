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


class Intervals:
    def __init__(self, *intervals):
        self.intervals = [*intervals]

    def __repr__(self):
        return str(self.intervals)

    def __add__(self, other: Interval):
        if not isinstance(other, Interval):
            raise ValueError("Can only add interval")
        if other.start not in self.intervals and other.end not in self.intervals:
            return Intervals(*self.intervals, other)
        else:
            intervals = list(sorted([other, *self.intervals], key=lambda x: x.start))
            new_intervals = []
            start_interval = intervals[0]
            for i, I in enumerate(intervals[1:]):
                if start_interval.end + 1 >= I.start >= start_interval.start:
                    start_interval = start_interval + I
                    if i < len(intervals) - 2:
                        continue
                    else:
                        new_intervals.append(start_interval)
                else:
                    new_intervals.append(start_interval)
                    if i < len(intervals) - 2:
                        start_interval = I
                    else:
                        new_intervals.append(I)
            return Intervals(*new_intervals)


class YouyounSubmission(SubmissionPy):
    def run(self, s, y_interest=2000000):
        """
        :param s: input in string format
        :return: solution flag
        """
        interval = None
        for line in s.replace("Sensor at x=", "").replace(": closest beacon is at x=", " ").replace(", y=", " ").split(
                "\n"):
            sensor_x, sensor_y, beacon_x, beacon_y = list(map(int, line.split(" ")))
            dist = abs(sensor_x - beacon_x) + abs(sensor_y - beacon_y)
            if not sensor_y - dist < y_interest < sensor_y + dist:
                continue
            pos_from_y = abs(sensor_y - y_interest)
            if interval is None:
                interval = Intervals(Interval(sensor_x - dist + pos_from_y, sensor_x + dist - pos_from_y))
            else:
                interval = interval + Interval(sensor_x - dist + pos_from_y, sensor_x + dist - pos_from_y)
        score = 0
        for I in interval.intervals:
            score += I.end - I.start
        return score


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
""".strip(), 10
            )
            == 26
    )


if __name__ == "__main__":
    test_youyoun()
