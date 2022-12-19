from tool.runners.python import SubmissionPy

from typing import Union

Packet = Union[int, list[int]]

class DavidSubmission(SubmissionPy):
    def parse_packet(self, raw: str) -> Packet:
        return eval(raw)

    def analyze_raw_pair(self, pair_block: str) -> bool:
        left, right = [self.parse_packet(raw) for raw in pair_block.split("\n")]
        return self.analyze_pair(left, right)

    def analyze_pair(self, left: Packet, right: Packet) -> bool:
        if isinstance(left, int) and isinstance(right, int):
            if left < right:
                return True
            if left > right:
                return False
            return None
        
        if isinstance(left, list) and isinstance(right, list):
            idx = 0
            for idx in range(len(left)):
                if idx >= len(right):
                    return False
                result = self.analyze_pair(left[idx], right[idx])
                if result is True or result is False:
                    return result
            if len(right) > len(left):
                return True
            return None

        # only one of them should be an int
        assert (isinstance(left, int) != isinstance(right, int))

        if isinstance(left, int):
            return self.analyze_pair([left], right)
        else:
            return self.analyze_pair(left, [right])

    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        for idx, block in enumerate(s.split("\n\n"), 1):
            if self.analyze_raw_pair(block):
                result += idx

        return result
