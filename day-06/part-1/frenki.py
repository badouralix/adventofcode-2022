from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    for i in range(4, len(s)):
      if len(set(s[i-4:i])) == 4:
        return i