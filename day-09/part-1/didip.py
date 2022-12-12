from tool.runners.python import SubmissionPy

class DidipSubmission(SubmissionPy):
    def distance(self, head, tail):
        return max(abs(head[0] - tail[0]), abs(head[1] - tail[1]))

    def update_tail(self, new_head, old_head, tail):
        distance = self.distance(new_head, tail)
        if distance == 2:  # Oh no we not touching anymore
            new_tail = old_head

            self.places_seen.add(new_tail)
            return new_tail

        return tail


    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        self.places_seen = set({(0, 0)})
        head = (0, 0)
        tail = (0, 0)

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
                new_head = (head[0] + update[0], head[1] + update[1])

                if self.distance(new_head, tail) == 2:
                    tail = head
                    self.places_seen.add(tail)

                head = new_head

        return len(self.places_seen)




def test_didip():
    """
    Run `python -m pytest ./day-09/part-1/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
""".strip()
        )
        == 13
    )
