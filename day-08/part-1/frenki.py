from tool.runners.python import SubmissionPy

class FrenkiSubmission(SubmissionPy):
  def run(self,s):
    l = s.splitlines()
    m = len(l)
    n = len(l[0])
    r = 0
    viewed = [[False for _ in range(n)] for _ in range(m)]
    for i in range(m):
      s = -1
      k = 0
      for j in range(n):
        v = int(l[i][j])
        if v > s:
          viewed[i][j] = True
          s = v
          r += 1
          k = j
      e = -1
      for j in range(n - 1, -1, -1):
        if j == k:
          break
        v = int(l[i][j])
        if v > e:
          viewed[i][j] = True
          e = v
          r += 1

    for j in range(n):
      s = -1
      k = 0
      for i in range(m):
        v = int(l[i][j])
        if v > s:
          if not viewed[i][j]:
            r += 1
            viewed[i][j] = True
          s = v
          k = i
      e = -1
      for i in range(m - 1, -1, -1):
        if i == k:
          break
        v = int(l[i][j])
        if v > e:
          if not viewed[i][j]:
            r += 1
            viewed[i][j] = True
          e = v

    return r
      











    # lines = s.splitlines()
    # start = lines[0]
    # end = lines[-1]
    # res = len(start) * 2
    # all_viewed = []
    # for k in range(1, len(lines) - 1):
    #   line = lines[k]
    #   s = line[0]
    #   viewed = []
    #   i = 0
    #   res += 1
    #   for p in range(1, len(line) - 1):
    #       if line[p] > start[p]:
    #         start[p] = line[p]
    #         res += 1
    #         viewed.append(p)
    #         continue
    #       if line[p] > s:
    #         s = line[p]
    #         res += 1
    #         i = p
    #   e = line[-1]
    #   for p in range(len(line) - 2, 0, -1):
    #     if p == i:
    #       break
    #     if p in viewed:
    #       continue
    #     if line[p] > e:
    #       e = line[p]
    #       res += 1
      
    # for k in range(len(line) - 2, 0, -1):
    #   line = lines[k]
    #   for p in range(1, len(line) - 1):
    #     if line[p] > end[p]:
    #       end[p] = line[p]
    #       res += 1
