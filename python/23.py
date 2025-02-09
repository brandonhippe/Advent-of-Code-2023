import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import deque


def part1(data):
    """ 2023 Day 23 Part 1

    >>> part1(['#.#####################', '#.......#########...###', '#######.#########.#.###', '###.....#.>.>.###.#.###', '###v#####.#v#.###.#.###', '###.>...#.#.#.....#...#', '###v###.#.#.#########.#', '###...#.#.#.......#...#', '#####.#.#.#######.#.###', '#.....#.#.#.......#...#', '#.#####.#.#.#########v#', '#.#...#...#...###...>.#', '#.#.#v#######v###.###v#', '#...#.>.#...>.>.#.###.#', '#####v#.#.###v#.#.###.#', '#.....#...#...#.#.#...#', '#.#########.###.#.#.###', '#...###...#...#...#.###', '###.###.#.###v#####v###', '#...#...#.#.>.>.#.>.###', '#.###.###.#.###.#.#v###', '#.....###...###...#...#', '#####################.#'])
    94
    """

    slopes = {}
    paths = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                continue

            if l == '.':
                paths.add((x, y))
                if y == 0:
                    start = (x, y)
                if y == len(data) - 1:
                    end = (x, y)
            else:
                slopes[(x, y)] = l

    nodes = {start: {}, end: {}}

    for pos in list(slopes.keys()) + list(paths):
        neighbors = []
        for offset in DIRS.values():
            nPos = tuple(p + o for p, o in zip(pos, offset))

            if nPos in slopes or nPos in paths:
                neighbors.append(nPos)

        if len(neighbors) > 2:
            if pos in paths:
                nodes[pos] = {}

    for pos in nodes.keys():
        visited = set()
        openList = deque()
        openList.append((0, pos))

        while len(openList) != 0:
            pathLen, currPos = openList.popleft()

            if currPos in visited:
                continue

            if currPos != pos and currPos in nodes:
                nodes[pos][currPos] = pathLen
                continue

            visited.add(currPos)

            if currPos in slopes:
                neighbors = [DIRS[slopes[currPos]]]
            else:
                neighbors = DIRS.values()

            for offset in neighbors:
                newPos = tuple(p + o for p, o in zip(currPos, offset))
                if newPos not in visited and (newPos in slopes or newPos in paths):
                    openList.append((pathLen + 1, newPos))

    return bfs(start, end, nodes)


def part2(data, testing = False):
    """ 2023 Day 23 Part 2

    >>> part2(['#.#####################', '#.......#########...###', '#######.#########.#.###', '###.....#.>.>.###.#.###', '###v#####.#v#.###.#.###', '###.>...#.#.#.....#...#', '###v###.#.#.#########.#', '###...#.#.#.......#...#', '#####.#.#.#######.#.###', '#.....#.#.#.......#...#', '#.#####.#.#.#########v#', '#.#...#...#...###...>.#', '#.#.#v#######v###.###v#', '#...#.>.#...>.>.#.###.#', '#####v#.#.###v#.#.###.#', '#.....#...#...#.#.#...#', '#.#########.###.#.#.###', '#...###...#...#...#.###', '###.###.#.###v#####v###', '#...#...#.#.>.>.#.>.###', '#.###.###.#.###.#.#v###', '#.....###...###...#...#', '#####################.#'], True)
    154
    """

    paths = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l != '#':
                paths.add((x, y))
                if y == 0:
                    start = (x, y)
                if y == len(data) - 1:
                    end = (x, y)

    nodes = {p: {} for p in [start, end] + [pos for pos in paths if sum(tuple(p + o for p, o in zip(pos, offset)) in paths for offset in DIRS.values()) >= 3]}

    for node in nodes.keys():
        openList = deque()
        openList.append((0, node, {node}))

        while len(openList) != 0:
            q, pos, currPath = openList.popleft()
            if pos != node and pos in nodes:
                nodes[node][pos] = q
                continue

            for nPos in [tuple(p + o for p, o in zip(pos, offset)) for offset in DIRS.values()]:
                if nPos not in currPath and nPos in paths:
                    openList.append((q + 1, nPos, currPath.union({nPos})))

    for node, connections in nodes.items():
        if len(connections) > 3:
            continue

        minDist = float('inf')
        newNode = {}
        minPerimeter = None
        for n, q in connections.items():
            if len(nodes[n]) > 3:
                newNode[n] = q
                continue

            dist = manhatDist(n, end)
            if dist < minDist:
                if minPerimeter:
                    del(newNode[minPerimeter])

                minPerimeter = n
                minDist = dist
                newNode[n] = q

        nodes[node] = newNode

    return bfs(start, end, nodes)


DIRS = {'<': (-1, 0), '>': (1, 0), 'v': (0, 1), '^': (0, -1)}


def bfs(start, end, nodes):
    openList = deque()
    openList.append((0, start, {start}))

    longest = -1

    while len(openList) != 0:
        q, pos, currPath = openList.popleft()

        if pos == end:
            longest = max(longest, q)
            continue

        for nPos, pLen in nodes[pos].items():
            if nPos not in currPath:
                openList.append((q + pLen, nPos, currPath.union({nPos})))

    return longest


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
        print(f"\nPart 1:\nLongest Path: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLongest Path: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)