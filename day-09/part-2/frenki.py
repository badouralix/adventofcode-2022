from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy): 
  positions = [[0,0] for _ in range(10)]
  v = dict()

  def run(self,s):
    self.v = dict()
    self.positions = [[0,0] for _ in range(10)]
    self.v[(0,0)] = True
    for l in s.splitlines():
      [d, i] = l.split(" ")
      for _ in range(int(i)):
        if d == "U":
          self.positions[0][0] += 1
        elif d == "D":
          self.positions[0][0] -= 1
        elif d == "R":
          self.positions[0][1] += 1
        else:
          self.positions[0][1] -= 1
        self.move(0)
    return len(self.v.keys())

  def move(self, l):
    moved = False
    if self.positions[l][0]-self.positions[l+1][0] == 2:
      self.positions[l+1][0] += 1
      moved = True
      if self.positions[l][1]-self.positions[l+1][1] >= 1:
        self.positions[l+1][1] += 1
      elif self.positions[l+1][1]-self.positions[l][1] >= 1:
        self.positions[l+1][1] -= 1
    elif self.positions[l+1][0]-self.positions[l][0] == 2:
      moved = True
      self.positions[l+1][0] -= 1
      if self.positions[l][1]-self.positions[l+1][1] >= 1:
        self.positions[l+1][1] += 1
      elif self.positions[l+1][1]-self.positions[l][1] >= 1:
        self.positions[l+1][1] -= 1
    elif self.positions[l][1]-self.positions[l+1][1] == 2:
      moved = True
      self.positions[l+1][1] += 1
      if self.positions[l][0]-self.positions[l+1][0] >= 1:
        self.positions[l+1][0] += 1
      elif self.positions[l+1][0]-self.positions[l][0] >= 1:
        self.positions[l+1][0] -= 1
    elif self.positions[l+1][1]-self.positions[l][1] == 2:
      moved = True
      self.positions[l+1][1] -= 1
      if self.positions[l][0]-self.positions[l+1][0] >= 1:
        self.positions[l+1][0] += 1
      elif self.positions[l+1][0]-self.positions[l][0] >= 1:
        self.positions[l+1][0] -= 1
    if moved:
      if l < 8:
        self.move(l+1)
      else:
        self.v[(self.positions[9][0], self.positions[9][1])] = True