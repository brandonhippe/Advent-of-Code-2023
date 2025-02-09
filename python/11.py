import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from itertools import combinations


def part1(data):
    """ 2023 Day 11 Part 1

    >>> part1(['...#......', '.......#..', '#.........', '..........', '......#...', '.#........', '.........#', '..........', '.......#..', '#...#.....'])
    374
    """

    xVals = set(range(len(data[0])))
    yVals = set(range(len(data)))
    galaxies = []
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                if x in xVals:
                    xVals.remove(x)

                if y in yVals:
                    yVals.remove(y)

                galaxies.append((x, y))

    emptyX = sorted(xVals)
    emptyY = sorted(yVals)

    return sum(galaxyDist(g1, g2, 2, emptyX, emptyY) for g1, g2 in combinations(galaxies, 2))


def part2(data, expandAmt = 1_000_000):
    """ 2023 Day 11 Part 2

    >>> part2(['...#......', '.......#..', '#.........', '..........', '......#...', '.#........', '.........#', '..........', '.......#..', '#...#.....'], 10)
    1030
    >>> part2(['...#......', '.......#..', '#.........', '..........', '......#...', '.#........', '.........#', '..........', '.......#..', '#...#.....'], 100)
    8410
    """

    xVals = set(range(len(data[0])))
    yVals = set(range(len(data)))
    galaxies = []
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                if x in xVals:
                    xVals.remove(x)

                if y in yVals:
                    yVals.remove(y)

                galaxies.append((x, y))

    emptyX = sorted(xVals)
    emptyY = sorted(yVals)

    return sum(galaxyDist(g1, g2, expandAmt, emptyX, emptyY) for g1, g2 in combinations(galaxies, 2))


def galaxyDist(p1, p2, expandMult, emptyX, emptyY):
    betweenX = []
    betweenY = []

    for x in emptyX:
        if max(p1[0], p2[0]) < x:
            break

        if min(p1[0], p2[0]) < x:
            betweenX.append(x)

    for y in emptyY:
        if max(p1[1], p2[1]) < y:
            break

        if min(p1[1], p2[1]) < y:
            betweenY.append(y)

    return sum(abs(c1 - c2) for c1, c2 in zip(p1, p2)) + (expandMult - 1) * (len(betweenX) + len(betweenY))


def manhatDist(p1, p2):
    return sum(abs(c1 - c2) for c1, c2 in zip(p1, p2))


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
        print(f"\nPart 1:\nSum of distances between galaxies: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of distances between galaxies: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)