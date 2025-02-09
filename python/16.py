import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2023 Day 16 Part 1

    >>> part1(['.|...\....', '|.-.\.....', '.....|-...', '........|.', '..........', '.........\\\\', r'..../.\\\..', '.-.-/..|..', '.|....-|.\\\\', '..//.|....'])
    46
    """

    neighbors = {}
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '.':
                neighbors[(x, y)] = {(0, 1) : [(0, 1)], (0, -1) : [(0, -1)], (1, 0) : [(1, 0)], (-1, 0) : [(-1, 0)]}
            elif l == '|':
                neighbors[(x, y)] = {(0, 1) : [(0, 1)], (0, -1) : [(0, -1)], (1, 0) : [(0, 1), (0, -1)], (-1, 0) : [(0, 1), (0, -1)]}
            elif l == '-':
                neighbors[(x, y)] = {(0, 1) : [(1, 0), (-1, 0)], (0, -1) : [(1, 0), (-1, 0)], (1, 0) : [(1, 0)], (-1, 0) : [(-1, 0)]}
            elif l == '/':
                neighbors[(x, y)] = {(0, 1) : [(-1, 0)], (0, -1) : [(1, 0)], (1, 0) : [(0, -1)], (-1, 0) : [(0, 1)]}
            elif l == '\\':
                neighbors[(x, y)] = {(0, 1) : [(1, 0)], (0, -1) : [(-1, 0)], (1, 0) : [(0, 1)], (-1, 0) : [(0, -1)]}

    return len(energize((-1, 0), (1, 0), len(data[0]), len(data), neighbors)) - 1


def part2(data):
    """ 2023 Day 16 Part 2

    >>> part2(['.|...\....', '|.-.\.....', '.....|-...', '........|.', '..........', '.........\\\\', r'..../.\\\..', '.-.-/..|..', '.|....-|.\\\\', '..//.|....'])
    51
    """

    neighbors = {}
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '.':
                neighbors[(x, y)] = {(0, 1) : [(0, 1)], (0, -1) : [(0, -1)], (1, 0) : [(1, 0)], (-1, 0) : [(-1, 0)]}
            elif l == '|':
                neighbors[(x, y)] = {(0, 1) : [(0, 1)], (0, -1) : [(0, -1)], (1, 0) : [(0, 1), (0, -1)], (-1, 0) : [(0, 1), (0, -1)]}
            elif l == '-':
                neighbors[(x, y)] = {(0, 1) : [(1, 0), (-1, 0)], (0, -1) : [(1, 0), (-1, 0)], (1, 0) : [(1, 0)], (-1, 0) : [(-1, 0)]}
            elif l == '/':
                neighbors[(x, y)] = {(0, 1) : [(-1, 0)], (0, -1) : [(1, 0)], (1, 0) : [(0, -1)], (-1, 0) : [(0, 1)]}
            elif l == '\\':
                neighbors[(x, y)] = {(0, 1) : [(1, 0)], (0, -1) : [(-1, 0)], (1, 0) : [(0, 1)], (-1, 0) : [(0, -1)]}

    s = 0
    for x in range(len(data[0])):
        topSide = len(energize((x, -1), (0, 1), len(data[0]), len(data), neighbors)) - 1
        bottomSide = len(energize((x, len(data)), (0, -1), len(data[0]), len(data), neighbors)) - 1
        s = max(s, topSide, bottomSide)

    for y in range(len(data)):
        leftSide = len(energize((-1, y), (1, 0), len(data[0]), len(data), neighbors)) - 1
        rightSide = len(energize((len(data[0]), y), (-1, 0), len(data[0]), len(data), neighbors)) - 1
        s = max(s, leftSide, rightSide)

    return s


def energize(start, startD, maxX, maxY, neighbors):
    toEval = [(start, startD)]
    visited = set()

    while len(toEval) != 0:
        pos, d = toEval.pop(0)
        if (pos, d) in visited:
            continue

        visited.add((pos, d))
        
        newPos = tuple(p + o for p, o in zip(pos, d))
        if min(newPos) < 0 or newPos[0] >= maxX or newPos[1] >= maxY:
            continue

        for newD in neighbors[newPos][d]:
            toEval.append([newPos, newD])

    return set(v[0] for v in visited)


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
        print(f"\nPart 1:\nTiles energized: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMaximum tiles energized: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)