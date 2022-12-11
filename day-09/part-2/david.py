from tool.runners.python import SubmissionPy

from typing import List, Tuple, Dict

Direction = Tuple[int, int]
Directions = List[Direction]

class DavidSubmission(SubmissionPy):
    NUM_ROPES = 10

    DIRECTIONS: Dict[str, Direction] = {
        "L": (0,-1),
        "R": (0,1),
        "U": (-1,0),
        "D": (1,0),
    }

    @staticmethod
    def compare(i: int, j: int) -> int:
        """Compares 2 integers and returns a value that can be 0,1 or -1"""
        if i == j:
            return 0
        if i > j:
            return 1
        return -1

    def run_once(self, directions: Directions) -> Directions:
        hi, hj = 0, 0
        ti, tj = 0, 0
        tail_directions: Directions = []
        for di, dj in directions:
            hi, hj = hi+di, hj+dj
            if abs(ti-hi) <= 1 and abs(tj-hj) <= 1:
                # don't do anything if the tail touches the head
                continue

            # if head and tails are in the same row/column, simply move the tail to the direction of the head
            dti, dtj = (self.compare(hi, ti), self.compare(hj, tj))
            tail_directions.append((dti, dtj))
            ti, tj = ti+dti, tj+dtj
        
        return tail_directions


    def run(self, s:str):
        lines = s.split("\n")
        directions = []
        for line in lines:
            direction, steps = line.split(" ")
            for _ in range(int(steps)):
                directions.append(self.DIRECTIONS[direction])

        for rope_num in range(1, self.NUM_ROPES):
            directions = self.run_once(directions)

        i, j = (0,0)
        positions = {(i,j)}
        for di, dj in directions:
            i, j = i+di, j+dj
            positions.add((i,j))
        return len(positions)