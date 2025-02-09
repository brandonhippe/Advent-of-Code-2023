import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from itertools import product


def part1(data):
    """ 2023 Day 7 Part 1

    >>> part1(['32T3K 765', 'T55J5 684', 'KK677 28', 'KTJJT 220', 'QQQJA 483'])
    6440
    """

    hands = []
    bets = {}

    for line in data:
        hand, bet = line.split(' ')
        hand = tuple(cardVals_P1[c] for c in hand)

        hands.append(hand)
        bets[hand] = int(bet)

    hands.sort(key=handsort_P1)

    return sum((i + 1) * bets[h] for i, h in enumerate(hands))


def part2(data):
    """ 2023 Day 7 Part 2

    >>> part2(['32T3K 765', 'T55J5 684', 'KK677 28', 'KTJJT 220', 'QQQJA 483'])
    5905
    """

    hands = []
    bets = {}

    for line in data:
        hand, bet = line.split(' ')
        hand = tuple(cardVals_P2[c] for c in hand)

        hands.append(hand)
        bets[hand] = int(bet)

    hands.sort(key=handsort_P2)

    return sum((i + 1) * bets[h] for i, h in enumerate(hands))


cardVals_P1 = {'A': 14, 'K': 13, 'Q': 12, 'J': 11, 'T': 10, '9': 9, '8': 8, '7': 7, '6': 6, '5': 5, '4': 4, '3': 3, '2': 2}
cardVals_P2 = {'A': 14, 'K': 13, 'Q': 12, 'J': 1, 'T': 10, '9': 9, '8': 8, '7': 7, '6': 6, '5': 5, '4': 4, '3': 3, '2': 2}
handScores = [(1, 1, 1, 1, 1), (1, 1, 1, 2), (1, 2, 2), (1, 1, 3), (2, 3), (1, 4), (5,)]


def handsort_P1(hand):
    handScore = 0
    cardCounts = tuple(sorted([len([n for n in hand if n == c]) for c in set(hand)]))

    handScore += handScores.index(cardCounts) + 1

    for c in hand:
        handScore *= 100
        handScore += c

    return handScore


def handsort_P2(hand):
    handScore = 0
    cardCounts = [len([n for n in hand if n == c]) for c in set(hand) if c != 1]

    for p in product(range(len(cardCounts)), repeat=5-sum(cardCounts)):
        potHand = cardCounts[:]
        for ix in p:
            potHand[ix] += 1

        handScore = max(handScore, handScores.index(tuple(sorted(potHand))) + 1)

    if len(cardCounts) == 0:
        handScore = 7

    for c in hand:
        handScore *= 100
        handScore += c

    return handScore


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
        print(f"\nPart 1:\nTotal Winnings: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal Winnings: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)