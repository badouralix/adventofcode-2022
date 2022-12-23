from tool.runners.python import SubmissionPy

from importlib import import_module

part1 = import_module("day-20.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        llist = part1.LinkedList()
        node = None
        nbs = map(lambda nb: int(nb) * 811589153, s.splitlines())
        for nb in nbs:
            next_node = part1.Node(nb)
            if node is not None:
                node.next = next_node
                next_node.previous = node
            else:
                llist.head = next_node
            node = next_node
        node.next = llist.head
        llist.head.previous = node

        nodes = list(llist)
        for _ in range(10):
            llist.mix(nodes)

        result = 0
        node_0 = next(node for node in llist if node.data == 0)
        for val in [1000, 2000, 3000]:
            val = val % len(nodes)
            node = node_0
            for _ in range(val):
                node = node.next
            result += node.data
        return result


def test_th_ch():
    """
    Run `python -m pytest ./day-20/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
1
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
