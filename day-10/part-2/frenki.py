from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    c = 0
    v = 1
    r = ""
    for l in s.splitlines():
      if c > 240:
        break
      if l[0] == "n":
        if abs(c - v) <= 1:
          r += "#"
        else:
          r += "."
        if c % 40 == 39:
          r += "\n"
          c = -1
        c += 1
      else:
        for _ in range(2):
          if abs(c - v) <= 1:
            r += "#"
          else:
            r += "."
          if c % 40 == 39:
            r += "\n"
            c = -1
          c += 1
        v += int(l.split(" ")[1])
    print(r)
    return r