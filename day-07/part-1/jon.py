from tool.runners.python import SubmissionPy


class JonSubmission(SubmissionPy):
    def run(self, s):
        lines = s.split("\n")

        root = {}
        cwd = None

        i = 0
        while i < len(lines):
            line = lines[i]

            if line[:5] == "$ cd ":
                name = line[5:]
                cwd = root if name == "/" else cwd[name]
                i += 1
                continue

            assert line == "$ ls"
            i += 1
            while i < len(lines):
                l = lines[i]
                if l[0] == "$":
                    break
                size, name = l.split(" ")
                if size == "dir":
                    if name not in cwd:
                        cwd[name] = {"..": cwd}
                else:
                    cwd[name] = int(size)
                i += 1

        result = [0]

        def compute_size(dir):
            size = 0
            for k, v in dir.items():
                if k == "..":
                    continue
                if isinstance(v, int):
                    size += v
                else:
                    size += compute_size(v)
            if size <= 100000:
                result[0] += size
            return size

        compute_size(root)

        return result[0]


def test_jon():
    """
    Run `python -m pytest ./day-07/part-1/jon.py` to test the submission.
    """
    assert (
        JonSubmission().run(
            """
$ cd /
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
