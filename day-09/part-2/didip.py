from tool.runners.python import SubmissionPy
from math import copysign

class DidipSubmission(SubmissionPy):
    @staticmethod
    def distance(head, tail):
        return max(abs(head[0] - tail[0]), abs(head[1] - tail[1]))

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        self.places_seen = set({(0, 0)})
        rope = [(0, 0) for _ in range(10)]

        directions = {
            'U': (0, 1),
            'D': (0, -1),
            'L': (-1, 0),
            'R': (1, 0)
        }

        for instruction in s.splitlines():
            direction, value = instruction.split()

            for i in range(int(value)):
                update = directions[direction]
                old_head = rope[0]
                rope[0] = (old_head[0] + update[0], old_head[1] + update[1])

                for j in range(1, len(rope)):
                    rope_tail = rope[j]
                    rope_head = rope[j - 1]
                    # print(rope_head, rope_tail, self.distance(rope_head, rope_tail))
                    if self.distance(rope_head, rope_tail) == 2:
                        if rope_head[0] == rope_tail[0]:
                            rope[j] = (rope_tail[0], rope_tail[1] + copysign(1, rope_head[1] - rope_tail[1]))
                        elif rope_head[1] == rope_tail[1]:
                            rope[j] = (rope_tail[0] + copysign(1, rope_head[0] - rope_tail[0])), rope_tail[1]
                        else:
                            rope[j] = (rope_tail[0] + copysign(1, rope_head[0] - rope_tail[0]), rope_tail[1] + copysign(1, rope_head[1] - rope_tail[1]))

                    old_head = rope_tail

                self.places_seen.add(rope[-1])
                # print(rope)
                # print(i, self.places_seen)

        return len(self.places_seen)



def test_didip():
    """
    Run `python -m pytest ./day-09/part-2/didip.py` to test the submission.
    """
#     assert (
#         DidipSubmission().run(
#             """R 4
# U 4
# L 3
# D 1
# R 4
# D 1
# L 5
# R 2
# """.strip()
#         )
#         == 1
#     )
    assert(
        DidipSubmission().run(
            """R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20""".strip())
        ) == 36
