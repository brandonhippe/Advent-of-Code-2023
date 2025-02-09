import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2023 Day 2 Part 1

    >>> part1(['Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green', 'Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue', 'Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red', 'Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red', 'Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green'])
    8
    """

    s = 0
    gameData = []
    for line in data:
        gameData.append(defaultdict(lambda: 0))
        id = int(re.findall('\d+', line)[0])

        data = line.split(": ")[1]
        validId = True
        for draw in data.split("; "):
            for color_data in draw.split(', '):
                n, c = color_data.split(' ')

                if int(n) > COLORS[c]:
                    validId = False

                gameData[-1][c] = max(gameData[-1][c], int(n))

        if validId:
            s += id

    return s


def part2(data):
    """ 2023 Day 2 Part 2

    >>> part2(['Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green', 'Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue', 'Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red', 'Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red', 'Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green'])
    2286
    """

    gameData = []
    for line in data:
        gameData.append(defaultdict(lambda: 0))

        data = line.split(": ")[1]
        for draw in data.split("; "):
            for color_data in draw.split(', '):
                n, c = color_data.split(' ')

                gameData[-1][c] = max(gameData[-1][c], int(n))

    s = 0
    for game in gameData:
        p = 1
        for n in game.values():
            p *= n

        s += p

    return s


COLORS = {'blue': 14, 'green': 13, 'red': 12}


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
        print(f"\nPart 1:\nSum of IDs of possible games: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of power sets of games: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)