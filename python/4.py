import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2023 Day 4 Part 1

    >>> part1(['Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53', 'Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19', 'Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1', 'Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83', 'Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36', 'Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11'])
    13
    """

    s = 0
    for line in data:
        line = line.split(': ')[1]
        winning, have = line.split(' | ')

        winning = set(int(n) for n in re.findall('\d+', winning))
        have = set(int(n) for n in re.findall('\d+', have))

        match = winning.intersection(have)
        
        if len(match) > 0:
            s += 2 ** (len(match) - 1)

    return s


def part2(data):
    """ 2023 Day 4 Part 2

    >>> part2(['Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53', 'Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19', 'Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1', 'Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83', 'Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36', 'Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11'])
    30
    """

    cards = {}
    for line in data:
        card, info = line.split(': ')
        winning, have = info.split(' | ')

        card = int(re.findall('\d+', card)[0])
        winning = set(int(n) for n in re.findall('\d+', winning))
        have = set(int(n) for n in re.findall('\d+', have))

        match = winning.intersection(have)
        cards[card] = list(range(card + 1, card + len(match) + 1))

    s = 0
    cardsGiven = defaultdict(lambda: 1)
    for c in cards.keys():
        s += getCards(c, cards, cardsGiven)

    return s


def getCards(c, cards, cardsGiven):
    if c in cardsGiven:
        return cardsGiven[c]
    
    cardsGiven[c] += sum(getCards(n, cards, cardsGiven) for n in cards[c])
    
    return cardsGiven[c]


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
        print(f"\nPart 1:\nScore of cards: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCards recieved: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)