import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import heapq


def part1(data):
    """ 2023 Day 17 Part 1

    >>> part1(['2413432311323', '3215453535623', '3255245654254', '3446585845452', '4546657867536', '1438598798454', '4457876987766', '3637877979653', '4654967986887', '4564679986453', '1224686865563', '2546548887735', '4322674655533'])
    102
    """

    costs = {}
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            costs[(x, y)] = int(l)

    start = (0, 0)
    end = (len(data[0]) - 1, len(data) - 1)

    openList = []
    openDict = {}
    closedDict = {}

    for d in [(0, 1), (1, 0)]:
        heapq.heappush(openList, (0, (start), d, 0))
        openDict[(start, d, 0)] = 0

    while len(openList) != 0:
        pathLen, pos, d, d_moved = heapq.heappop(openList)
        del(openDict[(pos, d, d_moved)])

        if pos == end:
            return pathLen
        
        for offset in [d, (d[1], -d[0]), (-d[1], d[0])]:
            newPos = tuple(p + o for p, o in zip(pos, offset))
            if newPos not in costs:
                continue

            new_d_moved = d_moved + 1 if offset == d else 0

            if new_d_moved == 3:
                continue

            newPathLen = pathLen + costs[newPos]

            if (newPos, offset, new_d_moved) in openDict and openDict[(newPos, offset, new_d_moved)] <= newPathLen:
                continue

            if (newPos, offset, new_d_moved) in closedDict and closedDict[(newPos, offset, new_d_moved)] <= newPathLen:
                continue

            heapq.heappush(openList, (newPathLen, newPos, offset, new_d_moved))
            openDict[(newPos, offset, new_d_moved)] = newPathLen

        closedDict[(pos, d, d_moved)] = pathLen

    return -1


def part2(data):
    """ 2023 Day 17 Part 2

    >>> part2(['2413432311323', '3215453535623', '3255245654254', '3446585845452', '4546657867536', '1438598798454', '4457876987766', '3637877979653', '4654967986887', '4564679986453', '1224686865563', '2546548887735', '4322674655533'])
    94
    >>> part2(['111111111111', '999999999991', '999999999991', '999999999991', '999999999991'])
    71
    """

    costs = {}
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            costs[(x, y)] = int(l)

    start = (0, 0)
    end = (len(data[0]) - 1, len(data) - 1)

    openList = []
    openDict = {}
    closedDict = {}

    for d in [(0, 1), (1, 0)]:
        heapq.heappush(openList, (0, start, d, 0))
        openDict[(start, d, 0)] = 0

    while len(openList) != 0:
        pathLen, pos, d, d_moved = heapq.heappop(openList)
        del(openDict[(pos, d, d_moved)])

        if pos == end and d_moved >= 3:
            return pathLen
        
        if d_moved < 3:
            new_dirs = [d]
        else:
            new_dirs = [d, (d[1], -d[0]), (-d[1], d[0])]
        
        for offset in new_dirs: 
            newPos = tuple(p + o for p, o in zip(pos, offset))
            if newPos not in costs:
                continue

            new_d_moved = d_moved + 1 if offset == d else 0

            if new_d_moved == 10:
                continue

            newPathLen = pathLen + costs[newPos]

            if (newPos, offset, new_d_moved) in openDict and openDict[(newPos, offset, new_d_moved)] <= newPathLen:
                continue

            if (newPos, offset, new_d_moved) in closedDict and closedDict[(newPos, offset, new_d_moved)] <= newPathLen:
                continue

            heapq.heappush(openList, (newPathLen, newPos, offset, new_d_moved))
            openDict[(newPos, offset, new_d_moved)] = newPathLen

        closedDict[(pos, d, d_moved)] = pathLen

    return -1


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
        print(f"\nPart 1:\nMinimum heat loss: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMinimum heat loss: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)