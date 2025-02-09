import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2023 Day 1 Part 1

    >>> part1(['1abc2', 'pqr3stu8vwx', 'a1b2c3d4e5f', 'treb7uchet'])
    142
    """

    return sum(int(''.join(d)) for d in [[re.findall('\d', line)[0], re.findall('\d', line)[-1]] for line in data])


def part2(data):
    """ 2023 Day 1 Part 2
    """

    nums = [[re.findall('\d', line)[0], re.findall('\d', line)[-1]] for line in data]

    for n, line in enumerate(data):
        d = nums[n]
        i = line.index(d[0])
        ix = [line.index(p) if p in line else float('inf') for p in PATT]

        if min(ix) < i:
            nums[n][0] = f"{ix.index(min(ix)) + 1}"

        i = line[::-1].index(d[1])
        ix = [line[::-1].index(p[::-1]) if p in line else float('inf') for p in PATT]

        if min(ix) < i:
            nums[n][1] = f"{ix.index(min(ix)) + 1}"

    return sum(int(''.join(n)) for n in nums)


PATT = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]


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
        print(f"\nPart 1:\nCalibration Value: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCalibration Value: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)