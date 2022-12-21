from tool.runners.python import SubmissionPy
from math import inf
from collections import defaultdict

class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        grid = set()
        sand_source = (500, 0)

        for line in s.splitlines():
            directions = line.split('->')

            i = 0
            x, y = map(int, directions[i].split(','))
            grid.add((x,y))

            for i in range(1, len(directions)):
                new_x, new_y = map(int, directions[i].split(','))
                if new_x == x:
                    # Moving on the y 
                    grid |= set([(x, y_moving) for y_moving in range(min(y, new_y), max(y, new_y) + 1)])

                else:
                    # print('moving on x')
                    grid |= set([(x_moving, y) for x_moving in range(min(x, new_x), max(x, new_x) + 1)])

                x, y = new_x, new_y

        min_x, min_y = inf, inf
        max_x, max_y = -inf, -inf
        for (x, y) in grid:
            min_x = min(x, min_x)
            min_y = min(y, min_y)
            max_x = max(x, max_x)
            max_y = max(y, max_y)

        # Adding the floor
        max_y = max_y + 2
        min_x = min(min_x, sand_source[0] - max_y)
        max_x = max(max_x, sand_source[0] + max_y)
        grid |= set([(x, max_y) for x in range(min_x, max_x + 1)])


        number_grains = 0

        while True:
            current_sand = sand_source
            moving = True
            # print('grain ', number_grains)
            while moving:
                if (current_sand[0], current_sand[1] + 1) not in grid:
                    current_sand = (current_sand[0], current_sand[1] + 1)
                elif (current_sand[0] - 1, current_sand[1] + 1) not in grid:
                    current_sand = (current_sand[0] - 1, current_sand[1] + 1)
                elif (current_sand[0] + 1, current_sand[1] + 1) not in grid:
                    current_sand = (current_sand[0] + 1, current_sand[1] + 1)
                else:
                    number_grains += 1
                    grid.add(current_sand)
                    moving = False
                    if current_sand == sand_source:
                        return number_grains

def test_didip():
    """
    Run `python -m pytest ./day-14/part-2/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
""".strip()
        )
        == 93
    )
