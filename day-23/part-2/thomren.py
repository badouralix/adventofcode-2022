from collections import defaultdict
import itertools
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        positions = {
            (x, y)
            for x, row in enumerate(s.splitlines())
            for y, c in enumerate(row)
            if c == "#"
        }
        directions = ["N", "S", "W", "E"]

        r = 0
        while r < 100_000:
            r += 1
            candidates = defaultdict(list)
            for (x, y) in positions:
                if not any(
                    (x + dx, y + dy) in positions
                    for dx, dy in itertools.product((-1, 0, 1), (-1, 0, 1))
                    if dx != 0 or dy != 0
                ):
                    # no neighbor
                    continue

                for direction in directions:
                    candidate = self.get_candidate(positions, x, y, direction)
                    if candidate is not None:
                        candidates[candidate].append((x, y))
                        break

            has_move = False
            for p, p_from in candidates.items():
                if len(p_from) == 1:
                    # 1 elve want to move to p, so he moves
                    positions.remove(p_from[0])
                    positions.add(p)
                    has_move = True
            if not has_move:
                return r

            directions = [d for d in directions[1:]] + [directions[0]]

        return -1

    @staticmethod
    def get_candidate(positions, x, y, direction):
        if direction == "N" and not any(
            p in positions for p in [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]
        ):
            return (x - 1, y)
        elif direction == "S" and not any(
            p in positions for p in [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
        ):
            return (x + 1, y)
        elif direction == "E" and not any(
            p in positions for p in [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
        ):
            return (x, y + 1)
        elif direction == "W" and not any(
            p in positions for p in [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]
        ):
            return (x, y - 1)
        else:
            return None

    @staticmethod
    def pretty_format(positions):
        xmin = min(x for x, _ in positions)
        xmax = max(x for x, _ in positions)
        ymin = min(y for _, y in positions)
        ymax = max(y for _, y in positions)
        return "\n".join(
            "".join("#" if (x, y) in positions else "." for y in range(ymin, ymax + 1))
            for x in range(xmin, xmax + 1)
        )


def test_thomren():
    """
    Run `python -m pytest ./day-23/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..

""".strip()
        )
        == 20
    )
