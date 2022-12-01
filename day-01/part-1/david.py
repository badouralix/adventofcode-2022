from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        return max(self.iterate(s))

    def iterate(self, s):
        for group in s.split("\n\n"):
            yield sum(int(x) for x in group.split("\n"))
