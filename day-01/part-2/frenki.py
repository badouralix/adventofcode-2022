from tool.runners.python import SubmissionPy


class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    r1 = 0
    r2 = 0
    r3 = 0
    c = 0
    for line in s.splitlines():
      if line == "":
        if c > r1:
          r3 = r2
          r2 = r1
          r1 = c
        elif c > r2:
          r3 = r2
          r2 = c
        elif c > r3:
          r3 = c
        c = 0
      else:
        c += int(line)
    if c > r1:
      r3 = r2
      r2 = r1
      r1 = c
    elif c > r2:
      r3 = r2
      r2 = c
    elif c > r3:
      r3 = c
    return r1 + r2 + r3