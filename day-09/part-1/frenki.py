from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self,s):
    h = [0,0]
    t = [0,0]
    v = dict()
    v[(0,0)] = True
    for l in s.splitlines():
      [d, i] = l.split(" ")
      for _ in range(int(i)):
        if d == "U":
          h[0] += 1
          if h[0]-t[0] == 2:
            t[0] += 1
            if h[1]-t[1] == 1:
              t[1] += 1
            elif t[1]-h[1] == 1:
              t[1] -= 1
        elif d == "D":
          h[0] -= 1
          if t[0]-h[0] == 2:
            t[0] -= 1
            if h[1]-t[1] == 1:
              t[1] += 1
            elif t[1]-h[1] == 1:
              t[1] -= 1
        elif d == "R":
          h[1] += 1
          if h[1]-t[1] == 2:
            t[1] += 1
            if h[0]-t[0] == 1:
              t[0] += 1
            elif t[0]-h[0] == 1:
              t[0] -= 1
        else:
          h[1] -= 1
          if t[1]-h[1] == 2:
            t[1] -= 1
            if h[0]-t[0] == 1:
              t[0] += 1
            elif t[0]-h[0] == 1:
              t[0] -= 1
        
        v[(t[0],t[1])] = True
    return len(v.keys())