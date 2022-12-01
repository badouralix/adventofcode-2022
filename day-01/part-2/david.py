from tool.runners.python import SubmissionPy
import heapq

class DavidSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        return sum(heapq.nlargest(3, self.iterate(s)))

    def iterate(self, s):
        for group in s.split("\n\n"):
            yield sum(int(x) for x in group.split("\n"))
