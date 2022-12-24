from tool.runners.python import SubmissionPy

from collections import Counter


def pretty_print_elves(elves):
    w = max(x for x, _ in elves) + 1
    h = max(y for _, y in elves) + 1
    print(
        "\n".join(
            "".join(("#" if (x, y) in elves else ".") for x in range(w))
            for y in range(h)
        )
    )
    print("----------")


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        north = lambda x, y: (x, y - 1)
        south = lambda x, y: (x, y + 1)
        east = lambda x, y: (x + 1, y)
        west = lambda x, y: (x - 1, y)
        ne = lambda x, y: north(*east(x, y))
        nw = lambda x, y: north(*west(x, y))
        se = lambda x, y: south(*east(x, y))
        sw = lambda x, y: south(*west(x, y))

        elves = set()

        for y, line in enumerate(s.splitlines()):
            for x, char in enumerate(line):
                if char == "#":
                    elves.add((x, y))

        directions = [
            ([north, ne, nw], north),
            ([south, se, sw], south),
            ([west, nw, sw], west),
            ([east, ne, se], east),
        ]
        for _ in range(10):
            proposals = {}
            nb_proposals = Counter()

            for x, y in elves:
                if all(
                    move(x, y) not in elves
                    for move in [north, south, west, east, ne, nw, se, sw]
                ):
                    continue

                proposal = None
                for moves, direction in directions:
                    if all(move(x, y) not in elves for move in moves):
                        proposal = direction
                        break

                if proposal is not None:
                    new_elf = proposal(x, y)
                    proposals[(x, y)] = new_elf
                    nb_proposals[new_elf] += 1

            for elf, proposal in proposals.items():
                if nb_proposals[proposal] == 1:
                    elves.remove(elf)
                    elves.add(proposal)

            directions = directions[1:] + [directions[0]]

        x_min = min(x for x, _ in elves)
        x_max = max(x for x, _ in elves)
        y_min = min(y for _, y in elves)
        y_max = max(y for _, y in elves)
        return sum(
            (x, y) not in elves
            for x in range(x_min, x_max + 1)
            for y in range(y_min, y_max + 1)
        )


def test_th_ch():
    """
    Run `python -m pytest ./day-23/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
""".strip()
        )
        == 110
    )
