from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        instructions = s.split("\n")

        cycle = 1
        register = 1
        buffer = [None, None]
        result = 0
        def next_cycle():
            nonlocal cycle, register, result, buffer
            if cycle in {20, 60, 100, 140, 180, 220}:
                result += cycle*register
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

        return result