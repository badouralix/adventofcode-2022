import errno
import os
import stat
import tempfile
from subprocess import PIPE, Popen, check_output

from tool.runners.exceptions import CompilationError, RuntimeError
from tool.runners.wrapper import SubmissionWrapper


class SubmissionGo(SubmissionWrapper):
    def __init__(self, file):
        SubmissionWrapper.__init__(self)
        tmp = tempfile.NamedTemporaryFile(prefix="aoc")
        tmp.close()
        compile_output = check_output(["go", "build", "-o", tmp.name, file]).decode()
        if compile_output:
            raise CompilationError(compile_output)
        os.chmod(tmp.name, os.stat(tmp.name).st_mode | stat.S_IEXEC)
        self.executable = tmp.name

    def exec(self, input):
        try:
            p = Popen([self.executable], stdin=PIPE, stdout=PIPE)
            stdout, _ = p.communicate(input.encode())
            return stdout.decode()
        except OSError as e:
            if e.errno == errno.ENOENT:
                # executable not found
                raise CompilationError(e)
            else:
                # subprocess exited with another error
                raise RuntimeError(e)
