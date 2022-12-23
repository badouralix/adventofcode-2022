from tool.runners.python import SubmissionPy


class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
        self.previous = None

    def __repr__(self):
        return str(self.data)


class LinkedList:
    def __init__(self):
        self.head = None
        self.current_node = None

    def __iter__(self):
        node = self.head
        while node.next != self.head:
            yield node
            node = node.next
        yield node

    def __repr__(self):
        return " -> ".join(str(node) for node in self)

    def mix(self, nodes):
        for node in nodes:
            offset = node.data % (len(nodes) - 1)
            prev_node = node.previous
            next_node = node.next
            # Remove the node
            prev_node.next = next_node
            next_node.previous = prev_node
            # Append it after the offset
            move = lambda node: node.previous if offset < 0 else node.next
            for _ in range(abs(offset)):
                prev_node = move(prev_node)
            node.next = prev_node.next
            node.previous = prev_node
            node.previous.next = node
            node.next.previous = node


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        llist = LinkedList()
        node = None
        nbs = map(int, s.splitlines())
        for nb in nbs:
            next_node = Node(nb)
            if node is not None:
                node.next = next_node
                next_node.previous = node
            else:
                llist.head = next_node
            node = next_node
        node.next = llist.head
        llist.head.previous = node

        nodes = list(llist)
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
    Run `python -m pytest ./day-20/part-1/th-ch.py` to test the submission.
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
        == 3
    )
