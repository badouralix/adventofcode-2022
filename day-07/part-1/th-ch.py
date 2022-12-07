from tool.runners.python import SubmissionPy


class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size

    def __str__(self):
        return self.name

    def __iter__(self):
        yield self


class Filesystem:
    def __init__(self, name, parent_folder=None):
        self.name = name
        self.parent_folder = parent_folder
        self.children = []

    def __str__(self):
        return self.name

    def __iter__(self):
        if isinstance(self, File):
            yield self
        else:
            yield self
            for child in self.children:
                yield from child

    def add_subfolder(self, name):
        existing = next(
            (sub_fs for sub_fs in self.children if sub_fs.name == name), None
        )
        if existing:
            return existing
        subfolder = Filesystem(name, self)
        self.children.append(subfolder)
        return subfolder

    def add_file(self, name, size):
        self.children.append(File(name, size))

    @property
    def size(self):
        if isinstance(self, File):
            return self.size

        return sum(child.size for child in self.children)


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        fs = Filesystem("/")
        navigation = fs
        for output in s.splitlines():
            if output.startswith("$ "):
                # command
                if output.startswith("$ cd "):
                    folder = output.replace("$ cd ", "")
                    if folder == "/":
                        navigation = fs
                    elif folder == "..":
                        navigation = navigation.parent_folder
                    else:
                        navigation = navigation.add_subfolder(folder)
            else:
                type_of_fs, name = output.split()
                if type_of_fs == "dir":
                    navigation.add_subfolder(name)
                else:
                    size = int(type_of_fs)
                    navigation.add_file(name, size)

        sum_of_small_folders = 0
        for child in fs:
            if not isinstance(child, File):
                size = child.size
                if size <= 100000:
                    sum_of_small_folders += size
        return sum_of_small_folders


def test_th_ch():
    """
    Run `python -m pytest ./day-07/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
