import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from functools import cache


def part1(data):
    """ 2023 Day 12 Part 1

    >>> part1(['???.### 1,1,3', '.??..??...?##. 1,1,3', '?#?#?#?#?#?#?#? 1,3,1,6', '????.#...#... 4,1,1', '????.######..#####. 1,6,5', '?###???????? 3,2,1'])
    21
    """

    s = 0
    for line in data:
        groups, numbers = line.split(' ')
        groups = tuple(re.findall('[#?]+', groups))
        numbers = tuple(int(n) for n in re.findall(r'\d+', numbers))
        s += calcCombs(groups, numbers)

    return s


def part2(data):
    """ 2023 Day 12 Part 2

    >>> part2(['???.### 1,1,3', '.??..??...?##. 1,1,3', '?#?#?#?#?#?#?#? 1,3,1,6', '????.#...#... 4,1,1', '????.######..#####. 1,6,5', '?###???????? 3,2,1'])
    525152
    """

    s = 0
    for line in data:
        groups, numbers = line.split(' ')
        groups = '?'.join([groups] * 5)
        numbers = (numbers + ',') * 5
        groups = tuple(re.findall('[#?]+', groups))
        numbers = tuple(int(n) for n in re.findall(r'\d+', numbers))
        
        s += calcCombs(groups, numbers)

    return s


@cache
def calcCombs(groups, numbers):
    combs = 0

    groupIx = 0
    while groupIx < len(groups) and len(groups[groupIx]) == 0:
        groupIx += 1

    if groupIx == len(groups):
        return len(numbers) == 0
    elif len(numbers) == 0:
        return all(set(g) == {'?'} for g in groups[groupIx:])

    # If first group is only #'s, check to make sure it matches the first number. If not, return 0
    # If first group does match first number, check if it is only group. If yes, return 1
    # Otherwise, remove fir
    if set(groups[groupIx]) == {'#'}:
        if len(numbers) == 0 or len(groups[groupIx]) != numbers[0]:
            return 0
        
        if len(groups) == 1 + groupIx:
            return len(numbers) == 1
        
        return calcCombs(groups[groupIx + 1:], numbers[1:])

    amt = 0
    for i, c in enumerate(groups[groupIx]):
        if c == '#':        
            # If character in first group is #, increase the amount found
            amt += 1
        else:
            # Check if amt found is consistent with first group. Otherwise, try both . and #
            if amt == 0 or amt == numbers[0]:
                # Conditions to check a .
                newGroups = [groups[groupIx][:i], groups[groupIx][i + 1:]] + list(groups[groupIx + 1:])
                combs += calcCombs(tuple(newGroups), numbers)

            if amt != numbers[0]:
                # Conditions to check a #
                newGroups = [groups[groupIx][:i] + '#' + groups[groupIx][i + 1:]] + list(groups[groupIx + 1:])
                combs += calcCombs(tuple(newGroups), numbers)

            break

    return combs


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
        print(f"\nPart 1:\nSum of possible arrangements: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of possible arrangements: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)