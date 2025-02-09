import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2023 Day 14 Part 1

    >>> part1(['O....#....', 'O.OO#....#', '.....##...', 'OO.#O....O', '.O.....O#.', 'O.#..O.#.#', '..O..#O..O', '.......O..', '#....###..', '#OO..#....'])
    136
    """

    cubes = set()
    rocks = set()

    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                cubes.add((x, y))
            elif l == 'O':
                rocks.add((x, y))

    rocks = tuple(rocks)

    return sum(len(data) - p[1] for p in roll(rocks, cubes, (0, -1), len(data)))


def part2(data, goalIterations = 1000000000):
    """ 2023 Day 14 Part 2

    >>> part2(['O....#....', 'O.OO#....#', '.....##...', 'OO.#O....O', '.O.....O#.', 'O.#..O.#.#', '..O..#O..O', '.......O..', '#....###..', '#OO..#....'])
    64
    """

    cubes = set()
    rocks = set()

    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                cubes.add((x, y))
            elif l == 'O':
                rocks.add((x, y))

    rocks = tuple(rocks)

    states = defaultdict(lambda: [])
    cycleFound = False
    iteration = 0

    while iteration < goalIterations:
        states[rocks].append(iteration)

        for dir in [(0, -1), (-1, 0), (0, 1), (1, 0)]:
            rocks = roll(rocks, cubes, dir, len(data))

        iteration += 1

        if rocks in states and not cycleFound:
            cycleLen = iteration - states[rocks][-1]
            iteration += ((goalIterations - iteration) // cycleLen) * cycleLen
            cycleFound = True

    return sum(len(data) - p[1] for p in rocks)


def roll(rocks, cubes, dir, maxVal):
    rocks = set(rocks)
    newRocks = set()

    positions = range(maxVal - 1, -1, -1) if sum(dir) > 0 else range(maxVal)

    for p1 in positions:
        nextPos = maxVal - 1 if sum(dir) > 0 else 0
        for p2 in positions:
            pos = (p1, p2) if dir[0] == 0 else (p2, p1)

            if pos in rocks:
                newRock = (p1, nextPos) if dir[0] == 0 else (nextPos, p1)
                if newRock in newRocks or newRock in cubes:
                    print("Error!")

                newRocks.add((p1, nextPos) if dir[0] == 0 else (nextPos, p1))
                nextPos -= sum(dir)

            if pos in cubes:
                nextPos = p2 - sum(dir)

    return tuple(newRocks)


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
        print(f"\nPart 1:\nLoad on pillars: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLoad on pillars after 1,000,000,000 cycles: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)