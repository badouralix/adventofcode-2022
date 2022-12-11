from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    DIRECTIONS = {
        "L": (0,-1),
        "R": (0,1),
        "U": (-1,0),
        "D": (1,0),
    }

    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        hi, hj = (0,0)
        ti, tj = (0,0)
        visited = {(ti,tj)}
        for line in s.split("\n"):
            direction, steps = line.split(" ")
            steps = int(steps)
            di, dj = self.DIRECTIONS[direction]
            for _ in range(steps):
                hi, hj = hi+di, hj+dj
                if abs(hi-ti) <= 1 and abs(hj-tj) <= 1:
                    continue

                ti, tj = hi-di,hj-dj
                visited.add((ti,tj))

        return len(visited)