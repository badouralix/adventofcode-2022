from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self, s):
    l = s.splitlines()
    n = len(l)
    m = len(l[0])
    r = 0
    for i in range(1,n-1):
      for j in range(1,m-1):
        w,x,y,z = 1,1,1,1
        v = int(l[i][j])
        while not(i - w == 0 or int(l[i-w][j]) >= v):
          w += 1
        while not(j - x == 0 or int(l[i][j-x]) >= v):
          x += 1
        while not(i + y == n - 1 or int(l[i+y][j]) >= v):
          y += 1
        while not(j + z == m - 1 or int(l[i][j+z]) >= v):
          z += 1
        s = w * x * y * z
        if s > r:
          r = s
    return r
