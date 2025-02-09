import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from math import ceil


def part1(data):
    """ 2023 Day 10 Part 1

    >>> part1(['..F7.', '.FJ|.', 'SJ.L7', '|F--J', 'LJ...'])
    8
    """

    pipes = {}
    tiles = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '|':
                pipes[(x, y)] = [(x, y - 1), (x, y + 1)]
            elif l == '-':
                pipes[(x, y)] = [(x - 1, y), (x + 1, y)]
            elif l == 'L':
                pipes[(x, y)] = [(x, y - 1), (x + 1, y)]
            elif l == 'J':
                pipes[(x, y)] = [(x, y - 1), (x - 1, y)]
            elif l == '7':
                pipes[(x, y)] = [(x, y + 1), (x - 1, y)]
            elif l == 'F':
                pipes[(x, y)] = [(x, y + 1), (x + 1, y)]
            elif l == 'S':
                animal = (x, y)
            else:
                tiles.add((x, y))

    pipes[animal] = []
    for pos, connects in pipes.items():
        if animal in connects and pos not in pipes[animal]:
            pipes[animal].append(pos)

    pos = animal
    loop = set()

    while pos not in loop:
        loop.add(pos)
        pos = pipes[pos][0] if pipes[pos][0] not in loop else pipes[pos][1]

    return ceil(len(loop) / 2)


def part2(data):
    """ 2023 Day 10 Part 2

    >>> part2(['FF7FSF7F7F7F7F7F---7', 'L|LJ||||||||||||F--J', 'FL-7LJLJ||||||LJL-77', 'F--JF--7||LJLJ7F7FJ-', 'L---JF-JLJ.||-FJLJJ7', '|F|F-JF---7F7-L7L|7|', '|FFJF7L7F-JF7|JL---7', '7-L-JL7||F7|L7F-7F7|', 'L.L7LFJ|||||FJL7||LJ', 'L7JLJL-JLJLJL--JLJ.L'])
    10
    """

    pipes = {}
    tiles = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '|':
                pipes[(x, y)] = [(x, y - 1), (x, y + 1)]
            elif l == '-':
                pipes[(x, y)] = [(x - 1, y), (x + 1, y)]
            elif l == 'L':
                pipes[(x, y)] = [(x, y - 1), (x + 1, y)]
            elif l == 'J':
                pipes[(x, y)] = [(x, y - 1), (x - 1, y)]
            elif l == '7':
                pipes[(x, y)] = [(x, y + 1), (x - 1, y)]
            elif l == 'F':
                pipes[(x, y)] = [(x, y + 1), (x + 1, y)]
            elif l == 'S':
                animal = (x, y)
            else:
                tiles.add((x, y))

    pipes[animal] = []
    for pos, connects in pipes.items():
        if animal in connects and pos not in pipes[animal]:
            pipes[animal].append(pos)

    pos = animal
    loop = set()

    while pos not in loop:
        loop.add(pos)
        pos = pipes[pos][0] if pipes[pos][0] not in loop else pipes[pos][1]

    s = 0
    for y, line in enumerate(data):
        inside = 0
        sides = {-1: False, 1: False}
        for x, l in enumerate(line):
            if (x, y) in loop:
                for k in sides.keys():
                    if (x, y + k) in pipes[(x, y)]:
                        sides[k] = ~sides[k]
            else:
                s += inside

            if all(sides.values()):
                if inside == 0:
                    inside = 1
                else:
                    inside = 0

                sides = {-1: False, 1: False}

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
        print(f"\nPart 1:\nDistance to furthest point on loop: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTiles enclosed by loop: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)