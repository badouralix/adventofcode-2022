from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    r = 0
    for l in s.splitlines():
      [a,b] = l.split(",")
      [x,y] = a.split("-")
      [v,w] = b.split("-")
      c = int(x)
      d = int(y)
      e = int(v)
      f = int(w)
      if c <= e and d >= f:
        r += 1
      elif c >= e and d <= f:
        r += 1

    return r