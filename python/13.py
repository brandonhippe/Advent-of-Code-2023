import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2023 Day 13 Part 1

    >>> part1(['#.##..##.', '..#.##.#.', '##......#', '##......#', '..#.##.#.', '..##..##.', '#.#.##.#.', '', '#...##..#', '#....#..#', '..##..###', '#####.##.', '#####.##.', '..##..###', '#....#..#'])
    405
    """

    lines = data[:] + ['']

    s = 0
    pattern = []
    for line in lines:
        if len(line) == 0:
            vert, horiz = findRefs(pattern)

            for k, v in vert.items():
                if v == 0:
                    s += k
            for k, v in horiz.items():
                if v == 0:
                    s += 100 * k

            pattern = []
        else:
            pattern.append(line)

    return s


def part2(data):
    """ 2023 Day 13 Part 2

    >>> part2(['#.##..##.', '..#.##.#.', '##......#', '##......#', '..#.##.#.', '..##..##.', '#.#.##.#.', '', '#...##..#', '#....#..#', '..##..###', '#####.##.', '#####.##.', '..##..###', '#....#..#'])
    400
    """

    lines = data[:] + ['']

    s = 0
    pattern = []
    for line in lines:
        if len(line) == 0:
            vert, horiz = findRefs(pattern)

            for k, v in vert.items():
                if v == 1:
                    s += k
            for k, v in horiz.items():
                if v == 1:
                    s += 100 * k

            pattern = []
        else:
            pattern.append(line)

    return s


def findRefs(pattern):
    ## Find vertical reflections
    vertical = {}
    for i in range(1, len(pattern[0])):
        vertical[i] = 0
        leftCol, rightCol = i - 1, i

        while leftCol >= 0 and rightCol < len(pattern[0]):
            for row in range(len(pattern)):
                vertical[i] += pattern[row][leftCol] != pattern[row][rightCol]

            leftCol -= 1
            rightCol += 1

    ## Find horizontal reflections
    horizontal = {}
    for i in range(1, len(pattern)):
        horizontal[i] = 0
        aboveRow, belowRow = i - 1, i

        while aboveRow >= 0 and belowRow < len(pattern):
            for col in range(len(pattern[0])):
                horizontal[i] += pattern[aboveRow][col] != pattern[belowRow][col]

            aboveRow -= 1
            belowRow += 1

    return vertical, horizontal


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nSummary of Notes: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSummary after desmudging: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)