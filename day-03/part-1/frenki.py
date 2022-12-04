from tool.runners.python import SubmissionPy
from collections import defaultdict

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    r = 0
    for l in s.splitlines():
      found = defaultdict(bool)
      for i in l[:len(l) // 2]:
        found[i] = True
      for i in l[len(l)//2:]:
        if found[i]:
          if i.islower():
            r += ord(i) - ord("a") + 1
          else:
            r += ord(i) - ord("A") + 27
          break
    return r