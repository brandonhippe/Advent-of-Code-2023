import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict, deque


def part1(data):
    """ 2023 Day 22 Part 1

    >>> part1(['1,0,1~1,2,1', '0,0,2~2,0,2', '0,2,3~2,2,3', '0,0,4~0,2,4', '2,0,5~2,2,5', '0,1,6~2,1,6', '1,1,8~1,1,9'])
    5
    """

    bricks = sorted([[[int(n) for n in re.findall(r'\d+', l)] for l in line.split('~')] for line in data], key=lambda b: b[0][-1])
    bricks = fall(bricks)

    support = defaultdict(lambda: [])
    supportedBy = defaultdict(lambda: [])

    for i, (c1, c2) in enumerate(bricks):
        for c3, c4 in bricks[i + 1:]:
            if c3[-1] > c2[-1] + 1:
                break

            if c3[-1] == c2[-1] + 1 and intersect(*c1[:-1], *c2[:-1], *c3[:-1], *c4[:-1]):
                support[(c1, c2)].append((c3, c4))
                supportedBy[(c3, c4)].append((c1, c2))

    disintegrate = set(bricks)
    for b in supportedBy.values():
        if len(b) != 1:
            continue

        b = b[0]
        if b in disintegrate:
            disintegrate.remove(b)

    return len(disintegrate)


def part2(data):
    """ 2023 Day 22 Part 2

    >>> part2(['1,0,1~1,2,1', '0,0,2~2,0,2', '0,2,3~2,2,3', '0,0,4~0,2,4', '2,0,5~2,2,5', '0,1,6~2,1,6', '1,1,8~1,1,9'])
    7
    """

    bricks = sorted([[[int(n) for n in re.findall(r'\d+', l)] for l in line.split('~')] for line in data], key=lambda b: b[0][-1])
    bricks = fall(bricks)

    support = defaultdict(lambda: [])
    supportedBy = defaultdict(lambda: [])

    for i, (c1, c2) in enumerate(bricks):
        for c3, c4 in bricks[i + 1:]:
            if c3[-1] > c2[-1] + 1:
                break

            if c3[-1] == c2[-1] + 1 and intersect(*c1[:-1], *c2[:-1], *c3[:-1], *c4[:-1]):
                support[(c1, c2)].append((c3, c4))
                supportedBy[(c3, c4)].append((c1, c2))

    chain = set()
    disintegrate = set(bricks)
    for b in supportedBy.values():
        if len(b) != 1:
            continue

        b = b[0]
        if b in disintegrate:
            chain.add(b)
            disintegrate.remove(b)

    total = 0
    for b in chain:
        fallen = set()
        toFall = deque()
        for b1 in support[b]:
            if len(supportedBy[b1]) == 1:
                toFall.append(b1)

        while len(toFall) != 0:
            b1 = toFall.popleft()
            fallen.add(b1)

            for b2 in support[b1]:
                if all(s in fallen or s == b for s in supportedBy[b2]):
                    toFall.append(b2)

        total += len(fallen)

    return total


def intersect(x1, y1, x2, y2, x3, y3, x4, y4):
    if x2 < x3:
        return False
    
    if x4 < x1:
        return False
    
    if y2 < y3:
        return False
    
    if y4 < y1:
        return False
    
    return True


def fall(bricks):
    newBricks = []
    for c1, c2 in bricks:
        zHeight = c2[-1] - c1[-1]

        newZ = 1
        for c3, c4 in newBricks:
            if not intersect(*c1[:-1], *c2[:-1], *c3[:-1], *c4[:-1]):
                continue

            newZ = max(newZ, c4[-1] + 1)

        newBricks.append((tuple(list(c1[:-1]) + [newZ]), tuple(list(c2[:-1]) + [newZ + zHeight])))

    return sorted(newBricks, key=lambda b: b[0][-1])


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
        print(f"\nPart 1:\nNumber of safe bricks: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal number of falling bricks: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)