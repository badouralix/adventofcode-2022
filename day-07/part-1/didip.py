from tool.runners.python import SubmissionPy

class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        tree_weights = {}
        current_dir = []

        lines = iter(s.splitlines())
        line = next(lines)
        current_sum = 0

        try:
            while line:
                if line[:4] == '$ cd':
                    if line[5:7] == '..':
                        tree_weights['/'.join(current_dir)] = current_sum
                        current_dir.pop()
                        tree_weights['/'.join(current_dir)] += current_sum
                        current_sum = tree_weights['/'.join(current_dir)]
                    else:
                        tree_weights['/'.join(current_dir)] = current_sum
                        current_dir.append(line[5:])

                    line = next(lines)

                elif line[:4] == '$ ls':
                    line = next(lines)
                    current_sum = 0
                    while line[:4] != '$ cd':
                        if line[:3] != 'dir':
                            current_sum += int(line.split()[0])

                        line = next(lines)

        except StopIteration:
            while len(current_dir) > 0:
                # doing continuous cd .. operations until reaching root
                tree_weights['/'.join(current_dir)] = current_sum
                current_dir.pop()
                tree_weights['/'.join(current_dir)] += current_sum
                current_sum = tree_weights['/'.join(current_dir)]

        return sum(filter(lambda x: x < 100000, tree_weights.values()))


def test_didip():
    """
    Run `python -m pytest ./day-07/part-1/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
""".strip()
        )
        == 95437
    )
