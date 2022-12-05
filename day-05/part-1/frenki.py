from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self,s):
    d = ["" for i in range(9)]
    i = False
    for l in s.splitlines():
      if not i:
        if l[1] == "1":
          i = True
          continue
        for k in range(9):
          if len(l) > k*4+2 and l[k*4+1] != " ":
            d[k] += l[k*4+1]
        continue
      if l == "":
        continue
      t = l.split(" ")
      a = int(t[1])
      b = int(t[3]) - 1
      c = int(t[5]) - 1
      d[c] = d[b][:a][::-1] + d[c]
      d[b] = d[b][a:]
    return d[0][0] + d[1][0] + d[2][0] + d[3][0] + d[4][0] + d[5][0] + d[6][0] + d[7][0] + d[8][0]
