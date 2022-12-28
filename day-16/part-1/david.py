from tool.runners.python import SubmissionPy
from collections import defaultdict

class DavidSubmission(SubmissionPy):
    MINUTES_LIMIT = 31

    def parse_input(self, s: str):
        lines = s.splitlines()
        flows: dict[str, int] = dict()
        tunnels: dict[str, set[str]] = dict()
        for line in lines:
            line = line.replace("rate=", "").replace(";", "").replace(", ", ",")
            tokens = line.split(" ")
            valve = tokens[1]
            flow = int(tokens[4])
            children = tokens[9].split(",")
            flows[valve] = flow
            tunnels[valve] = set(children)
        return tunnels, flows

    def compute_distances(self, tunnels: dict[str, set[str]]) -> dict[str, dict[str, int]]:
        distances = defaultdict(lambda: defaultdict(lambda: float('inf')))
        nodes = list(tunnels.keys())
        n = len(nodes)
        for node in nodes:
            distances[node][node] = 0
            for neighbor in tunnels[node]:
                distances[node][neighbor] = 1
        
        for k in range(n):
            for i in range(n):
                for j in range(n):
                    d = distances[nodes[i]][nodes[k]] + distances[nodes[k]][nodes[j]]
                    if d <= distances[nodes[i]][nodes[j]]:
                        distances[nodes[i]][nodes[j]] = d

        return distances

    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        tunnels, flows = self.parse_input(s)
        distances = self.compute_distances(tunnels)

        start = "AA"
        q = []
        q.append((0, 0, start, set()))
        nodes = set(n for n in flows if flows[n] > 0)
        max_score = 0
        while q:
            score, minutes, position, activated = q.pop()
            if score > max_score:
                max_score = score

            for next_node in nodes:
                if next_node in activated:
                    continue
                if distances[position][next_node] + minutes >= self.MINUTES_LIMIT:
                    continue

                next_score = score + (self.MINUTES_LIMIT-minutes-distances[position][next_node]-2)*flows[next_node]
                q.append((next_score, minutes+distances[position][next_node]+1, next_node, activated | {next_node}))

        return max_score
