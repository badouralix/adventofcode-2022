from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    for i in range(14, len(s)):
      if len(set(s[i-14:i])) == 14:
        return i