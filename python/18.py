import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2023 Day 18 Part 1

    >>> part1(['R 6 (#70c710)', 'D 5 (#0dc571)', 'L 2 (#5713f0)', 'D 2 (#d2c081)', 'R 2 (#59c680)', 'D 2 (#411b91)', 'L 5 (#8ceee2)', 'U 2 (#caa173)', 'L 1 (#1b58a2)', 'U 2 (#caa171)', 'R 2 (#7807d2)', 'U 3 (#a77fa3)', 'L 2 (#015232)', 'U 2 (#7a21e3)'])
    62
    """

    pos = (0, 0)
    adjacencies = defaultdict(lambda: set())
    xs = set()
    ys = set()

    for line in data:
        prev = pos
        direction, dist, color = line.split(' ')
        pos = tuple(p + (o * int(dist)) for p, o in zip(pos, OFFSETS[direction]))
        xs.add(pos[0])
        ys.add(pos[1])
        adjacencies[pos].add(prev)
        adjacencies[prev].add(pos)

    return enclosedArea(adjacencies, xs, ys)


def part2(data):
    """ 2023 Day 18 Part 2

    >>> part2(['R 6 (#70c710)', 'D 5 (#0dc571)', 'L 2 (#5713f0)', 'D 2 (#d2c081)', 'R 2 (#59c680)', 'D 2 (#411b91)', 'L 5 (#8ceee2)', 'U 2 (#caa173)', 'L 1 (#1b58a2)', 'U 2 (#caa171)', 'R 2 (#7807d2)', 'U 3 (#a77fa3)', 'L 2 (#015232)', 'U 2 (#7a21e3)'])
    952408144115
    """

    pos = (0, 0)
    adjacencies = defaultdict(lambda: set())
    xs = set()
    ys = set()

    for line in data:
        direction, dist, color = line.split(' ')

        prev = pos
        dist = re.findall('\w+', color)[0]
        direction = list(OFFSETS.keys())[int(dist[-1])]
        dist = int(dist[:-1], 16)
        pos = tuple(p + (o * dist) for p, o in zip(pos, OFFSETS[direction]))
        xs.add(pos[0])
        ys.add(pos[1])
        adjacencies[pos].add(prev)
        adjacencies[prev].add(pos)

    return enclosedArea(adjacencies, xs, ys)


OFFSETS = {'R': (1, 0), 'D': (0, 1), 'L': (-1, 0), 'U': (0, -1)}


def enclosedArea(adjacencies, xs, ys):
    total = 0
    xs = sorted(xs)
    ys = sorted(ys)

    for i, y in enumerate(ys):
        inside_line = False
        first_line = True
        collected_line = None

        inside_area = False

        for x in xs:
            ### Handle areas above this line, up to but not including previous line
            if (x, y) in adjacencies:
                for n in adjacencies[(x, y)]:
                    if n[1] != y and n[1] < y:
                        if inside_area:
                            inside_area = False
                            total += (y - ys[i - 1] - 1) * (x - px_area + 1)
                        else:
                            inside_area = True
                            px_area = x

                        break
            else:
                toggled = False
                for y1 in ys[:i]:
                    if (x, y1) not in adjacencies:
                        continue

                    for n in adjacencies[(x, y1)]:
                        if y < n[1]:
                            if inside_area:
                                inside_area = False
                                total += (y - ys[i - 1] - 1) * (x - px_area + 1)
                            else:
                                inside_area = True
                                px_area = x

                            toggled = True
                            break

                    if toggled:
                        break
                        
            ### Handle areas on this line
            if (x, y) in adjacencies:
                if collected_line is None:
                    if not inside_line:
                        inside_line = True
                        px_line = x

                    for n in adjacencies[(x, y)]:
                        if n[1] != y:
                            collected_line = abs(n[1] - y) // (n[1] - y)
                            break
                else:
                    for n in adjacencies[(x, y)]:
                        if n[1] != y:
                            if ((abs(n[1] - y) // (n[1] - y)) != collected_line) ^ first_line:
                                inside_line = False
                                first_line = True
                                total += x - px_line + 1
                            else:       
                                first_line = False

                            collected_line = None
                            break
            else:
                toggled = False
                for y1 in ys[:i]:
                    if (x, y1) not in adjacencies:
                        continue

                    for n in adjacencies[(x, y1)]:
                        if y < n[1]:
                            if inside_line:
                                inside_line = False
                                first_line = True
                                total += x - px_line + 1
                            else:
                                inside_line = True
                                first_line = False
                                px_line = x

                            toggled = True
                            break

                    if toggled:
                        break

    return total


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
        print(f"\nPart 1:\nEnclosed Area: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nEnclosed Area: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)