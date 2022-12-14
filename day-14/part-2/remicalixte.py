from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s: str):
        grid = []
        max_x = 0
        for line in s.split('\n'):
            last_x, last_y = -1, -1
            for i, coor in enumerate(line.split('->')):
                x, y = (int(c) for c in coor.strip().split(','))
                max_x = max(max_x, x)
                if i != 0:
                    draw_line(grid, max_x, x, y, last_x, last_y)

                last_x = x
                last_y = y

        bottom = len(grid) + 1
        source_x, source_y = 500, 0
        falling_x, falling_y = 0, 0
        falling = False
        settled = 0
        while True:
            # print(falling_x, falling_y)
            if not falling:
                falling_x, falling_y = source_x, source_y
                falling = True

            if available(grid, max_x, bottom, falling_x, falling_y+1):
                falling_y += 1
            elif available(grid, max_x, bottom, falling_x-1, falling_y+1):
                falling_x -= 1
                falling_y += 1
            elif available(grid, max_x, bottom, falling_x+1, falling_y+1):
                falling_x += 1
                falling_y += 1
            else:
                set_sand(grid, max_x, falling_x, falling_y)
                settled += 1
                falling = False

                if (falling_x, falling_y) == (source_x, source_y):
                    return settled
                # for line in grid:
                #     print(line[494:])



def available(grid, max_x, bottom, x, y):
    if y >= bottom:
        return False
    ensure_space(grid, max_x, x, y)
    return grid[y][x] == '.'


def set_sand(grid, max_x, x, y):
    ensure_space(grid, max_x, x, y)
    grid[y][x] = 'o'


def draw_line(grid: list[list[str]], max_x: int, x: int, y: int, last_x: int, last_y: int):
    if abs(x-last_x) != 0:
        for j in range(min(x,last_x), max(x,last_x)+1):
            ensure_space(grid, max_x, j, y)
            grid[y][j] = '#'

    if abs(y-last_y) != 0:
        for i in range(min(y,last_y), max(y,last_y)+1):
            ensure_space(grid, max_x, x, i)
            grid[i][x] = '#'


def ensure_space(grid, max_x, x, y):
    while len(grid) <= y:
        grid.append(['.' for _ in range(max_x+1)])
    while len(grid[y]) <= x:
        grid[y].append('.')


def test_remicalixte():
    """
    Run `python -m pytest ./day-14/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
