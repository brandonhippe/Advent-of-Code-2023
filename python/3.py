import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2023 Day 3 Part 1

    >>> part1(['467..114..', '...*......', '..35..633.', '......#...', '617*......', '.....+.58.', '..592.....', '......755.', '...$.*....', '.664.598..'])
    4361
    """

    symbols = {}
    for y, line in enumerate(data):
        for s in re.finditer('[^.]', line):
            symbols[(s.span()[0], y)] = s.group()

    partNumbers = set()
    for pos, symbol in symbols.items():
        if symbol.isdigit():
            continue

        for yoff in range(-1, 2):
            for xoff in range(-1, 2):
                nPos = tuple(p + o for p, o in zip(pos, (xoff, yoff)))

                if nPos in symbols and symbols[nPos].isdigit():
                    partNumbers.add(nPos)

    partNumberLocs = {}
    while len(partNumbers) != 0:
        x, y = list(partNumbers)[0]
        while (x - 1, y) in symbols and symbols[(x - 1, y)].isdigit():
            x -= 1

        xStart = x

        toRemove = {(x, y)}
        num = symbols[(x, y)]
        while (x + 1, y) in symbols and symbols[(x + 1, y)].isdigit():
            num += symbols[(x + 1, y)]
            toRemove.add((x + 1, y))
            x += 1

        xEnd = x

        partNumberLocs[(xStart, xEnd, y)] = int(num)
        partNumbers.difference_update(toRemove)

    return sum(partNumberLocs.values())


def part2(data):
    """ 2023 Day 3 Part 2

    >>> part2(['467..114..', '...*......', '..35..633.', '......#...', '617*......', '.....+.58.', '..592.....', '......755.', '...$.*....', '.664.598..'])
    467835
    """

    symbols = {}
    for y, line in enumerate(data):
        for s in re.finditer('[^.]', line):
            symbols[(s.span()[0], y)] = s.group()

    partNumbers = set()
    for pos, symbol in symbols.items():
        if symbol.isdigit():
            continue

        for yoff in range(-1, 2):
            for xoff in range(-1, 2):
                nPos = tuple(p + o for p, o in zip(pos, (xoff, yoff)))

                if nPos in symbols and symbols[nPos].isdigit():
                    partNumbers.add(nPos)

    partNumberLocs = {}
    while len(partNumbers) != 0:
        x, y = list(partNumbers)[0]
        while (x - 1, y) in symbols and symbols[(x - 1, y)].isdigit():
            x -= 1

        xStart = x

        toRemove = {(x, y)}
        num = symbols[(x, y)]
        while (x + 1, y) in symbols and symbols[(x + 1, y)].isdigit():
            num += symbols[(x + 1, y)]
            toRemove.add((x + 1, y))
            x += 1

        xEnd = x

        partNumberLocs[(xStart, xEnd, y)] = int(num)
        partNumbers.difference_update(toRemove)

    s = 0
    for pos, symbol in symbols.items():
        if symbol != '*':
            continue

        adj = set()
        for yoff in range(-1, 2):
            for xoff in range(-1, 2):
                nPos = tuple(p + o for p, o in zip(pos, (xoff, yoff)))

                for xStart, xEnd, y in partNumberLocs.keys():
                    if y == nPos[1] and xStart <= nPos[0] <= xEnd:
                        adj.add((xStart, xEnd, y))

        if len(adj) == 2:
            s += partNumberLocs[list(adj)[0]] * partNumberLocs[list(adj)[1]]

    return s


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
        print(f"\nPart 1:\nSum of Part Numbers: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of Gear Ratios: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)