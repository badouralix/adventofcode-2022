from tool.runners.python import SubmissionPy
from collections import defaultdict

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    r = 0
    v = s.splitlines()
    i = 0
    while i < len(v):
      for l in v[i]:
        if l in v[i+1] and l in v[i+2]:
          if l.islower():
            r += ord(l) - ord("a") + 1
          else:
            r += ord(l) - ord("A") + 27
          break
      i += 3
    return r