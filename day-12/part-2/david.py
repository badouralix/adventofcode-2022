from tool.runners.python import SubmissionPy

import heapq

Position = tuple[int, int]

class DavidSubmission(SubmissionPy):
    MOVES: list[Position] = [(1,0), (-1,0), (0,1), (0,-1)]

    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        start, end = None, None
        grid = []
        for i, line in enumerate(s.splitlines()):
            if "S" in line:
                start = (i, line.index("S"))
            if "E" in line:
                end = (i, line.index("E"))
            grid.append([ord(x)-ord("a") for x in line])

        n, m = len(grid), len(grid[0])

        grid[start[0]][start[1]] = 0
        grid[end[0]][end[1]] = 25

        visited: set[Position] = set()
        q = []
        heapq.heappush(q, (0, end))
        visited.add(end)
        while q:
            length, position = heapq.heappop(q)
            i, j = position
            if grid[i][j] == 0:
                return length
            for di, dj in self.MOVES:
                ni, nj = (i+di, j+dj)
                if (
                    0 <= ni < n and 0 <= nj < m and
                    grid[i][j] <= grid[ni][nj] + 1 and
                    (ni, nj) not in visited
                ):
                    heapq.heappush(q, (length+1, (ni,nj)))
                    visited.add((ni,nj))
            
             
