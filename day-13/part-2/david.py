from tool.runners.python import SubmissionPy

from typing import Union
from functools import cmp_to_key

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
        self.comparisons = 0
        packets: list[Packet] = []
        for raw_packet in s.splitlines():
            if raw_packet:
                packets.append(self.parse_packet(raw_packet))
        
        # we don't need to sort the packets entirely to get the answer...
        # just need to find the indexes of the two special packets if the list was sorted
        p2 = [[2]]
        idx2 = sum(1 for packet in packets if not self.analyze_pair(p2, packet)) + 1
        p6 = [[6]]
        idx6 = sum(1 for packet in packets if not self.analyze_pair(p6, packet)) + 1
        min_idx, max_idx = sorted([idx2, idx6])
        return min_idx * (max_idx + 1)
