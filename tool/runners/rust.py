import errno
import os
import subprocess
import tempfile

from tool.runners.exceptions import CompilationError, RuntimeError
from tool.runners.wrapper import SubmissionWrapper


class SubmissionRs(SubmissionWrapper):
    def __init__(self, file):
        DEVNULL = open(os.devnull, "wb")
        SubmissionWrapper.__init__(self)
        tmpdir = tempfile.TemporaryDirectory(prefix="aoc")
        tmpdir.cleanup()
        # try:
        #     subprocess.check_output(
        #         ["cargo", "test", "--bin", file.replace("/", "-")[:-3]], stderr=DEVNULL
        #     ).decode()
        # except subprocess.CalledProcessError as e:
        #     raise CompilationError(e.output)

        p = subprocess.Popen(
            ["cargo", "build", "--release", "--bin", file.replace("/", "-")[:-3]],
            env={
                **os.environ,
                "CARGO_TARGET_DIR": tmpdir.name,
                "RUSTFLAGS": "-C target-cpu=native",
            },
            stdout=DEVNULL,
            stderr=DEVNULL,
        ).wait()
        if p > 0:
            raise CompilationError("Could not compile " + file)
        self.executable = tmpdir.name + "/release/" + file.replace("/", "-")[:-3]

    def exec(self, input):
        try:
            return subprocess.check_output([self.executable, input]).decode()
        except OSError as e:
            if e.errno == errno.ENOENT:
                # executable not found
                raise CompilationError(e)
            else:
                # subprocess exited with another error
                raise RuntimeError(e)
