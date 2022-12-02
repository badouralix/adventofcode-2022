from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    r = 0
    for l in s.splitlines():
      if l == "A X":
        r += 3
      elif l == "A Y":
        r += 4
      elif l == "A Z":
        r += 8
      elif l == "B X":
        r += 1
      elif l == "B Y":
        r += 5
      elif l == "B Z":
        r += 9
      elif l == "C X":
        r += 2
      elif l == "C Y":
        r += 6
      else:
        r += 7
    return r