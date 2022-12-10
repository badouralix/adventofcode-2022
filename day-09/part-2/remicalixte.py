from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        rope_len = 10
        visited = set()
        visited.add((0,0))
        rope = [(0,0) for _ in range(rope_len)]
        for line in s.split('\n'):
            d, cnt = line.split()
            cnt = int(cnt)
            increment = (0,0)
            if d == 'U':
                increment = (0, 1)
            elif d == 'D':
                increment = (0, -1)
            elif d == 'R':
                increment = (1, 0)
            elif d == 'L':
                increment = (-1, 0)
            while cnt > 0:
                head = rope[0]
                rope[0] = (head[0]+increment[0], head[1] + increment[1])
                for i in range(1, rope_len):
                    rope[i] = move_tail(rope[i-1],rope[i])
                cnt -= 1
                visited.add(rope[-1])

        return len(visited)


def move_tail(head, tail):
    head_x, head_y = head
    tail_x, tail_y = tail

    step_x = head_x - tail_x
    step_y = head_y - tail_y

    move_x, move_y = 0, 0

    if abs(step_x) > 1 or abs(step_y) > 1:
        if step_x > 0:
            move_x = 1
        elif step_x < 0:
            move_x = -1

        if step_y > 0:
            move_y = 1
        elif step_y < 0:
            move_y = -1

    return (tail_x+move_x, tail_y+move_y)



def test_remicalixte():
    """
    Run `python -m pytest ./day-09/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
