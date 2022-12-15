from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s: str):
        sensors = []
        beacons = []
        for line in s.split('\n'):
            sensor, beacon = line.split(':')
            sensor = sensor.removeprefix('Sensor at ')
            beacon = beacon.removeprefix(' closest beacon is at ')
            sensors.append(parse_coor_str(sensor))
            beacons.append(parse_coor_str(beacon))

        y = 2000000


        intervals = []
        for i in range(len(sensors)):
            d_beacon = manhattan_distance(sensors[i], beacons[i])
            sx, sy = sensors[i]
            dy = abs(sy-y)
            delta = d_beacon-dy
            if delta >= 0:
                intervals.append((sx-delta,sx+delta+1))


        intervals.sort(key=lambda interval: interval[0])
        merged_intervals = [intervals[0]]
        for interval in intervals[1:]:
            s,e = interval
            ms, me = merged_intervals[-1]
            if ms <= s and s <= me:
                merged_intervals[-1] = (ms, max(e, me))
            else:
                merged_intervals.append(interval)

        return sum(e-s for (s,e) in merged_intervals) - len(set(bx for (bx,by) in beacons if by == y))







def manhattan_distance(p1, p2):
    x1, y1 = p1
    x2, y2 = p2
    return abs(x1-x2) + abs(y1-y2)



def parse_coor_str(s: str):
    sx, sy = s.split(', ')
    x = int(sx.removeprefix('x='))
    y = int(sy.removeprefix('y='))

    return (x,y)


def test_remicalixte():
    """
    Run `python -m pytest ./day-15/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
