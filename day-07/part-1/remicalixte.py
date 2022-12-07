from tool.runners.python import SubmissionPy


class Node:
    def __init__(self, name: str, size: int, parent) -> None:
        self.name = name
        self.size = size
        self.parent = parent
        self.children = {}

    def __str__(self) -> str:
        return self.str(0)
    def str(self, indent: int) -> str:
        s = '  ' * indent + f'- {self.name} {self.size}\n'
        for child in self.children.values():
            s += '  ' * indent + f'  {child.str(indent+1)}'

        return s


    def get_size_sum(self) -> int:
        true_size = 0
        size_sum = 0
        if self.size > 0:
            true_size = self.size
        else:
            for child in self.children.values():
                child_true_size, child_size_sum = child.get_size_sum()
                true_size += child_true_size
                size_sum += child_size_sum
            if true_size <= 100000:
                size_sum += true_size

        return true_size, size_sum


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):

        root = Node('', 0, None)
        current_node = root
        ls = []
        for line in s.split('\n'):
            part = line.split()
            if part[0] == '$':
                process_ls(ls, current_node)
                ls = []
                cmd = part[1]
                if cmd == 'cd':
                    folder = part[2]
                    if folder == '/':
                        current_node = root
                    elif folder == '..':
                        current_node = current_node.parent
                    else:
                        child = current_node.children.get(folder, None)
                        if child is None:
                            child = Node(folder, 0, current_node)
                            current_node.children[folder] = child
                        current_node = child
            else:
                ls.append(line)
        process_ls(ls, current_node)

        return root.get_size_sum()[1]



def process_ls(ls, current_node):
    if len(ls) > 0:
        for line in ls:
            size, name = line.split()
            if size == 'dir':
                current_node.children.setdefault(name, Node(name, 0, current_node))
            else:
                current_node.children[name] = Node(name, int(size), current_node)



def test_remicalixte():
    """
    Run `python -m pytest ./day-07/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
