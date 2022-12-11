from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    ROWS = 6
    COLUMNS = 40

    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        instructions = s.split("\n")

        cycle = 1
        register = 1
        buffer = [None, None]
        grid = [[False for _ in range(self.COLUMNS)] for _ in range(self.ROWS)]
        def next_cycle():
            nonlocal cycle, register            
            i, j = int((cycle-1)/self.COLUMNS), (cycle-1)%self.COLUMNS
            grid[i][j] = register in {j-1, j, j+1}
            # print(f"cycle={cycle} i={i} j={j} x={register} grid={grid[i][j]}")
            # if cycle in {40, 80, 120, 160, 200, 240}:
            #    print("".join(["#" if grid[i][k] else "." for k in range(self.COLUMNS)]))
            register += buffer[cycle % 2] or 0
            cycle += 1
        
        for instruction in instructions:
            if instruction == "noop":
                buffer[(cycle+1)%2] = 0
                next_cycle()
            else:
                value = int(instruction[5:])
                buffer[(cycle+1)%2] = value
                next_cycle()
                next_cycle()

        result = []
        for line in grid:
            result.append(''.join(["#" if x else "." for x in line]))
        return '\n'.join(result)