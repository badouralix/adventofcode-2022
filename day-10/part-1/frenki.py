from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    c = 0
    v = 1
    r = 0
    for l in s.splitlines():
      if c > 220:
        break
      if l[0] == "n":
        if c % 40 == 19:
          r += v * (c + 1)
        c += 1
      else:
        if c % 40  == 18:
          r += v * (c + 2)
        elif c % 40 == 19:
          r += v * (c + 1)
        c += 2
        v += int(l.split(" ")[1])
    return r