from tool.runners.python import SubmissionPy


class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    r = 0
    c = 0
    for line in s.splitlines():
      if line == "":
        if c > r:
          r = c
        c = 0
      else:
        c += int(line)
    if c > r:
      r = c
    return r