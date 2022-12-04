from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    r = 0
    for l in s.splitlines():
      [a,b] = l.split(",")
      [x,y] = a.split("-")
      [v,w] = b.split("-")
      if int(x) <=  int(w) and int(v) <= int(y):
        r += 1
    return r