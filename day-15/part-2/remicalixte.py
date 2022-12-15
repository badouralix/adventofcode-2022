from tool.runners.python import SubmissionPy


max_coor = 4000000


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

        intervalss = [[] for i in range(max_coor+1)]
        for i in range(len(sensors)):
            d_beacon = manhattan_distance(sensors[i], beacons[i])
            populate_intervalss(intervalss, sensors[i], d_beacon)

        (x,y) = find_spot(intervalss)

        return x * max_coor + y


def populate_intervalss(intervalss: list[list], sensor: tuple[int], d_beacon: int):
    sx, sy = sensor
    min_y = sy - d_beacon
    max_y = sy + d_beacon
    for y in range(max(0,min_y), sy):
        delta = y - min_y
        intervalss[y].append((sx-delta, sx+delta+1))
    for y in range(sy, min(max_y,max_coor)+1):
        delta = max_y - y
        intervalss[y].append((sx-delta, sx+delta+1))


def find_spot(intervalss: list[list]):
        for y, intervals in enumerate(intervalss):
            intervals.sort(key=lambda interval: interval[0])
            x = 0
            for interval in intervals:
                start, end = interval
                if x < start:
                    break
                if end > x:
                    x = end
            if x <= max_coor:
                break
        return (x,y)



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
