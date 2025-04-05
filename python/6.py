import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
import numpy as np
import math


def part1(data):
    """ 2023 Day 6 Part 1

    >>> part1(['Time:      7  15   30', 'Distance:  9  40  200'])
    288
    """

    times = [int(n) for n in re.findall(r'\d+', data[0])]
    distances = [int(n) for n in re.findall(r'\d+', data[1])]

    product = 1
    for t, d in zip(times, distances):
        winCounts = 0

        for s in range(t):
            winCounts += (t - s) * s > d

        product *= winCounts

    return product


def part2(data):
    """ 2023 Day 6 Part 2

    >>> part2(['Time:      7  15   30', 'Distance:  9  40  200'])
    71503
    """

    t = int(data[0].replace(' ', '').split(':')[1])
    d = int(data[1].replace(' ', '').split(':')[1])

    roots = np.roots([1, -t, d])
    return math.floor(max(roots)) - math.ceil(min(roots)) + 1


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
        print(f"\nPart 1:\nProduct of ways to beat the record: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nWays to beat the record: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)