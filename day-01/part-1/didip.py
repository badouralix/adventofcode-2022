from tool.runners.python import SubmissionPy


class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        return max(map(lambda elf: sum(map(int, elf.splitlines())), s.split('\n\n')))
