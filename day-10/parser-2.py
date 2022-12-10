from tool.parser import Parser


letters = {
    "#..#.#.#..##...#.#..#.#..#..#.": "K",
    "#....#....#....#....#....####.": "L",
    ".##..#..#.#..#.####.#..#.#..#.": "A",
    ".##..#..#.#....#.##.#..#..###.": "G",
    "###..#..#.#..#.###..#....#....": "P",
    "#..#.#..#.####.#..#.#..#.#..#.": "H",
    "####....#...#...#...#....####.": "Z",
    "..##....#....#....#.#..#..##..": "J",
    "###..#..#.#..#.###..#.#..#..#.": "R",
    "###..#..#.###..#..#.#..#.###..": "B",
    "####.#....###..#....#....#....": "F",
    ".##..#..#.#....#....#..#..##..": "C",
    "####.#....###..#....#....####.": "E",
    "#..#.#..#.#..#.#..#.#..#..##..": "U",
}

LETTER_WIDTH = 5
N_LETTERS = 8
HEIGHT = 6
WIDTH = N_LETTERS * LETTER_WIDTH


class D10P2Parser(Parser):
    def __init__(self):
        self.unknown = set()

    def parse(self, s):
        ss = s.replace("\n", "").strip()
        if len(ss) != WIDTH * HEIGHT:
            return s
        if not all(c in ("#", ".") for c in ss):
            return s

        lines = [ss[WIDTH * i : WIDTH * (i + 1)] for i in range(HEIGHT)]

        def letter(pos):
            return "".join(
                l[LETTER_WIDTH * pos : LETTER_WIDTH * (pos + 1)] for l in lines
            )

        res = "".join(self.parse_letter(letter(pos)) for pos in range(N_LETTERS))
        return res

    def parse_letter(self, s):
        if s not in letters:
            self.unknown.add(s)
            return "?"
        return letters[s]

    def cleanup(self):
        if len(self.unknown) == 0:
            return
        print("Some letters are unknown to the parser, please add them.")
        print("Copy this code and replace the question marks with the letters below,")
        print("then add it to " + __file__)
        print()
        unknown = list(self.unknown)
        for s in unknown:
            print('"{}": "?",'.format(s))
        print()
        for s in unknown:
            pretty_print_letter(s)
            print()


def pretty_print_letter(s):
    ss = s.replace("0", " ").replace("1", u"\u2588")
    for l in range(HEIGHT):
        print(ss[LETTER_WIDTH * l : LETTER_WIDTH * (l + 1)])
