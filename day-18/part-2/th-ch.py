from tool.runners.python import SubmissionPy
from importlib import import_module

part1 = import_module("day-18.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        cubes = set()
        for cube in s.splitlines():
            cubes.add(tuple(int(x) for x in cube.split(",")))

        nb_intersecting_cubes = 0
        for x, y, z in cubes:
            for neighbor in part1.get_neighbors(x, y, z):
                if neighbor in cubes:
                    nb_intersecting_cubes += 1

        x_min = min(x for x, y, z in cubes)
        x_max = max(x for x, y, z in cubes)
        y_min = min(y for x, y, z in cubes)
        y_max = max(y for x, y, z in cubes)
        z_min = min(z for x, y, z in cubes)
        z_max = max(z for x, y, z in cubes)

        cuboid = [
            (x, y, z)
            for x in range(x_min, x_max + 1)
            for y in range(y_min, y_max + 1)
            for z in range(z_min, z_max + 1)
            if (x, y, z) not in cubes
        ]
        not_trapped_neighbors = set()
        to_process = cuboid
        has_found_not_trapped = True
        while has_found_not_trapped:
            has_found_not_trapped = False
            to_process_updated = []
            for x, y, z in to_process:
                if (
                    x <= x_min
                    or x >= x_max
                    or y <= y_min
                    or y >= y_max
                    or z <= z_min
                    or z >= z_max
                    or any(
                        neighbor in not_trapped_neighbors
                        for neighbor in part1.get_neighbors(x, y, z)
                    )
                ):
                    not_trapped_neighbors.add((x, y, z))
                    has_found_not_trapped = True
                else:
                    to_process_updated.append((x, y, z))
            to_process = to_process_updated

        trapped = set(cuboid) - not_trapped_neighbors

        nb_intersecting_trapped = 0
        for x, y, z in trapped:
            for neighbor in part1.get_neighbors(x, y, z):
                if neighbor in trapped:
                    nb_intersecting_trapped += 1

        return (
            len(cubes) * 6
            - nb_intersecting_cubes
            - len(trapped) * 6
            + nb_intersecting_trapped
        )


def test_th_ch():
    """
    Run `python -m pytest ./day-18/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
""".strip()
        )
        == 58
    )
