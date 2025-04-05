import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict
import math


def part1(data):
    """ 2023 Day 8 Part 1

    >>> part1(['RL', '', 'AAA = (BBB, CCC)', 'BBB = (DDD, EEE)', 'CCC = (ZZZ, GGG)', 'DDD = (DDD, DDD)', 'EEE = (EEE, EEE)', 'GGG = (GGG, GGG)', 'ZZZ = (ZZZ, ZZZ)'])
    2
    >>> part1(['LLR', '', 'AAA = (BBB, BBB)', 'BBB = (AAA, ZZZ)', 'ZZZ = (ZZZ, ZZZ)'])
    6
    """

    instructions = [0 if c == 'L' else 1 for c in data[0]]

    nodes = {}
    for line in data[2:]:
        lineData = re.findall(r'\w+', line)
        nodes[lineData[0]] = lineData[1:]

    s = 0
    node, endNode = 'AAA', 'ZZZ'

    while node != endNode:
        node = nodes[node][instructions[s % len(instructions)]]
        s += 1

    return s


def part2(data):
    """ 2023 Day 8 Part 2

    >>> part2(['LR', '', '11A = (11B, XXX)', '11B = (XXX, 11Z)', '11Z = (11B, XXX)', '22A = (22B, XXX)', '22B = (22C, 22C)', '22C = (22Z, 22Z)', '22Z = (22B, 22B)', 'XXX = (XXX, XXX)'])
    6
    """

    instructions = [0 if c == 'L' else 1 for c in data[0]]

    nodes = {}
    for line in data[2:]:
        lineData = re.findall(r'\w+', line)
        nodes[lineData[0]] = lineData[1:]

    startNodes = {n for n in nodes.keys() if n[-1] == 'A'}
    cycles = {n: defaultdict(list) for n in startNodes}
    finishes = {n: defaultdict(list) for n in startNodes}

    for n in startNodes:
        node = n
        steps = 0
        found = False

        while not found:
            if node[-1] == 'Z':
                finishes[n][node].append(steps)
                
            cycles[n][node].append(steps)
            node = nodes[node][instructions[steps % len(instructions)]]
            steps += 1

            if node in cycles[n]:
                for prev in cycles[n][node]:
                    if (steps - prev) % len(instructions) == 0:
                        cycles[n] = steps - prev
                        found = True
                        break
    
    return math.lcm(*cycles.values())


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
        print(f"\nPart 1:\nSteps to ZZZ: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSteps to all nodes ending in Z: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)