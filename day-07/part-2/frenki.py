from tool.runners.python import SubmissionPy
from collections import defaultdict

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    curr = []
    sizes = defaultdict(int)
    for l in s.splitlines():
      v = l.split(" ")
      if len(v) == 3:
        if v[2] == "/":
          curr = ["/"]
        elif v[2] == "..":
          curr = curr[:-1]
        else:
          curr.append(curr[-1] +  "/" + v[2])
      else:
        e = -1
        try:
          e = int(v[0])
        except ValueError:
          pass
        if e != -1:
          for i in curr:
            sizes[i] += e

    p = sizes.get("/") - 40000000
    r = 70000000
    for k in sizes.values():
      if k > p and k < r:
        r = k
    return r