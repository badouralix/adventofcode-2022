from dataclasses import dataclass, field
from typing import Dict, Optional
from tool.runners.python import SubmissionPy

FS_TOTAL_SIZE = 70000000
UPDATE_SIZE = 30000000


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        root_dir = parse_filesystem(s)
        available = FS_TOTAL_SIZE - root_dir.get_size()
        min_deleted_size = UPDATE_SIZE - available
        return find_best_dir_to_delete(root_dir, min_deleted_size)


@dataclass
class File:
    name: str
    size: int


@dataclass
class Directory:
    name: str
    parent: Optional["Directory"] = None
    files: Dict[str, File] = field(default_factory=dict)
    subdirectories: Dict[str, "Directory"] = field(default_factory=dict)
    size: Optional[int] = None

    def get_size(self) -> int:
        if self.size is not None:
            return self.size
        return sum(f.size for f in self.files.values()) + sum(
            d.get_size() for d in self.subdirectories.values()
        )


def parse_filesystem(s: str) -> Directory:
    lines = s.splitlines()
    i = 1
    root_dir = Directory("/")
    working_dir = root_dir
    while i < len(lines):
        if lines[i].startswith("$ ls"):
            i += 1
            while i < len(lines) and not lines[i].startswith("$ "):
                if lines[i].startswith("dir"):
                    _, dirname = lines[i].split()
                    working_dir.subdirectories[dirname] = Directory(
                        dirname, working_dir
                    )
                else:
                    size, filename = lines[i].split()
                    working_dir.files[filename] = File(filename, int(size))
                i += 1
        elif lines[i].startswith("$ cd"):
            _, dirname = lines[i].split(" cd ")
            if dirname == "..":
                if working_dir.parent is None:
                    raise ValueError(f"no parent found for dir {working_dir.name}")
                working_dir = working_dir.parent
            else:
                working_dir = working_dir.subdirectories.setdefault(
                    dirname, Directory(dirname, working_dir)
                )
            i += 1
        else:
            raise ValueError(f"Unknown command: {lines[i]}")

    return root_dir


def find_best_dir_to_delete(directory: Directory, min_dir_size: int) -> float:
    if directory.get_size() < min_dir_size:
        return float("inf")

    return min(
        directory.get_size(),
        min(
            (
                find_best_dir_to_delete(sd, min_dir_size)
                for sd in directory.subdirectories.values()
            ),
            default=float("inf"),
        ),
    )


def test_thomren():
    """
    Run `python -m pytest ./day-07/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
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
7214296 k""".strip()
        )
        == 24933642
    )
