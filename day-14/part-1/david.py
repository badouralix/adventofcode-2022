from tool.runners.python import SubmissionPy

from typing import Iterable
Position = tuple[int, int]

class FallsIntoVoid(Exception):
    pass

class DavidSubmission(SubmissionPy):
    SAND_ORIGIN = (500, 0)

    def find_resting_position(self, position: Position, blocks: set[Position], ground: int) -> Position:
        x, y = position
        while True:
            if y > ground:
                raise FallsIntoVoid()
            if (x,y+1) not in blocks:
                y += 1
            elif (x-1,y+1) not in blocks:
                x -= 1
                y += 1
            elif (x+1,y+1) not in blocks:
                x += 1
                y += 1
            else:
                return (x,y)

    def parse_position(self, raw_position: str) -> Position:
        raw_x, raw_y = raw_position.split(",")
        return (int(raw_x), int(raw_y))
    
    def parse_line(self, line: str) -> set[Position]:
        positions: set[Position] = set()
        initial_positions = [self.parse_position(raw) for raw in line.split(" -> ")]
        positions.add(initial_positions[0])
        for p1, p2 in zip(initial_positions, initial_positions[1:]):
            x1, y1 = p1
            x2, y2 = p2
            if x1 == x2:
                for dy in range(min(y1, y2), max(y1,y2)+1):
                    positions.add((x1,dy))
            elif y1 == y2:
                for dx in range(min(x1, x2), max(x1,x2)+1):
                    positions.add((dx,y1))
        return positions

    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        blocks: set[Position] = set()
        for line in s.splitlines():
            blocks |= self.parse_line(line)

        ground = max(y for _,y in blocks)

        result = 0
        while True:
            try:
                resting_position = self.find_resting_position(self.SAND_ORIGIN, blocks, ground)
                blocks.add(resting_position)
                result += 1
            except FallsIntoVoid:
                return result
