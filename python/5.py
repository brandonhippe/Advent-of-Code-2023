import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2023 Day 5 Part 1

    >>> part1(['seeds: 79 14 55 13', '', 'seed-to-soil map:', '50 98 2', '52 50 48', '', 'soil-to-fertilizer map:', '0 15 37', '37 52 2', '39 0 15', '', 'fertilizer-to-water map:', '49 53 8', '0 11 42', '42 0 7', '57 7 4', '', 'water-to-light map:', '88 18 7', '18 25 70', '', 'light-to-temperature map:', '45 77 23', '81 45 19', '68 64 13', '', 'temperature-to-humidity map:', '0 69 1', '1 0 69', '', 'humidity-to-location map:', '60 56 37', '56 93 4'])
    35
    """

    maps = defaultdict(list)
    conversions = {}
    for line in data[2:]:
        if len(line) == 0:
            continue

        numStrings = re.findall(r'\d+', line)
        if len(numStrings) == 0:
            currConv = line.split(' ')[0]
            currConv = tuple(currConv.split('-to-'))
            conversions[currConv[0]] = currConv[1]
        else:
            maps[currConv].append([int(n) for n in numStrings])

    seeds = [int(n) for n in re.findall(r'\d+', data[0])]
    nums = seeds[:]
    currType = 'seed'

    while currType in conversions:
        currConv = (currType, conversions[currType])

        newData = []
        for d in nums:
            appended = False
            for destS, sourceS, r in maps[currConv]:
                if sourceS <= d < sourceS + r:
                    newData.append(destS + d - sourceS)
                    appended = True
                    break

            if not appended:
                newData.append(d)

        nums = newData
        currType = currConv[1]

    return min(nums)


def part2(data):
    """ 2023 Day 5 Part 2

    >>> part2(['seeds: 79 14 55 13', '', 'seed-to-soil map:', '50 98 2', '52 50 48', '', 'soil-to-fertilizer map:', '0 15 37', '37 52 2', '39 0 15', '', 'fertilizer-to-water map:', '49 53 8', '0 11 42', '42 0 7', '57 7 4', '', 'water-to-light map:', '88 18 7', '18 25 70', '', 'light-to-temperature map:', '45 77 23', '81 45 19', '68 64 13', '', 'temperature-to-humidity map:', '0 69 1', '1 0 69', '', 'humidity-to-location map:', '60 56 37', '56 93 4'])
    46
    """

    maps = defaultdict(list)
    conversions = {}
    for line in data[2:]:
        if len(line) == 0:
            continue

        numStrings = re.findall(r'\d+', line)
        if len(numStrings) == 0:
            currConv = line.split(' ')[0]
            currConv = tuple(currConv.split('-to-'))
            conversions[currConv[0]] = currConv[1]
        else:
            maps[currConv].append([int(n) for n in numStrings])

    seeds = [int(n) for n in re.findall(r'\d+', data[0])]

    s = float('inf')

    for i in range(0, len(seeds), 2):
        rangeS, r = seeds[i:i + 2]
        
        ranges = [[rangeS, r]]
        currType = 'seed'
        while currType in conversions:
            currConv = (currType, conversions[currType])

            newRanges = []
            while len(ranges) != 0:
                rangeS, r = ranges.pop()
                split = False
                for destS, sourceS, searchR in maps[currConv]:
                    if sourceS <= rangeS and rangeS + r <= sourceS + searchR:
                        split = True
                        newRanges.append([destS + rangeS - sourceS, r])
                        break

                    if sourceS <= rangeS < sourceS + searchR:
                        split = True
                        ranges.append([rangeS, searchR - (rangeS - sourceS)])
                        ranges.append([sourceS + searchR, r - (searchR - (rangeS - sourceS))])
                        break

                    if sourceS < rangeS + r <= sourceS + searchR:
                        split = True
                        ranges.append([rangeS, sourceS - rangeS])
                        ranges.append([sourceS, r - (sourceS - rangeS)])
                        break

                if not split:
                    newRanges.append([rangeS, r])

            currType = currConv[1]
            ranges = newRanges

        s = min(s, min([r[0] for r in ranges]))

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
        print(f"\nPart 1:\nMinimum Location Number: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMinimum Location Number: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)