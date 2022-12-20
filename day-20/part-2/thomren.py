from dataclasses import dataclass
from typing import Optional
from tool.runners.python import SubmissionPy

DECRYPTION_KEY = 811589153
N_MIXES = 10


from dataclasses import dataclass
from typing import Optional
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        it = map(int, s.splitlines())
        nodes = [Node(next(it) * DECRYPTION_KEY, None, None)]
        for n in it:
            node = Node(n * DECRYPTION_KEY, nodes[-1], None)
            nodes[-1].after = node
            nodes.append(node)
        nodes[-1].after = nodes[0]
        nodes[0].prev = nodes[-1]

        for _ in range(N_MIXES):
            for node in nodes:
                node.prev.after = node.after
                node.after.prev = node.prev
                x, y = node.prev, node.after
                for _ in range(node.value % (len(nodes) - 1)):
                    x, y = x.after, y.after
                x.after = node
                node.prev = x
                node.after = y
                y.prev = node

        node = nodes[0]
        while node.value != 0:
            node = node.after

        res = 0
        for _ in range(3):
            for _ in range(1000):
                node = node.after
            res += node.value

        return res


@dataclass
class Node:
    value: int
    prev: Optional["Node"]
    after: Optional["Node"]


def test_thomren():
    """
    Run `python -m pytest ./day-20/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """1
2
-3
3
-2
0
4
""".strip()
        )
        == 1623178306
    )
