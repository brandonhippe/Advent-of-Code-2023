import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from z3 import *


def part1(data, minBound = 200000000000000, maxBound = 400000000000000):
    """ 2023 Day 24 Part 1

    >>> part1(['19, 13, 30 @ -2,  1, -2', '18, 19, 22 @ -1, -1, -2', '20, 25, 34 @ -2, -2, -4', '12, 31, 28 @ -1, -2, -1', '20, 19, 15 @  1, -5, -3'], 7, 27)
    2
    """

    ps = []
    vs = []
    for line in data:
        p, v = line.split(' @ ')
        ps.append(tuple(int(n) for n in re.findall('-?\d+', p)))
        vs.append(tuple(int(n) for n in re.findall('-?\d+', v)))

    return intersections([tuple(p[:-1]) for p in ps], [tuple(v[:-1]) for v in vs], minBound, maxBound)


def part2(data):
    """ 2023 Day 24 Part 2

    >>> part2(['19, 13, 30 @ -2,  1, -2', '18, 19, 22 @ -1, -1, -2', '20, 25, 34 @ -2, -2, -4', '12, 31, 28 @ -1, -2, -1', '20, 19, 15 @  1, -5, -3'])
    47
    """

    ps = []
    vs = []
    for line in data:
        p, v = line.split(' @ ')
        ps.append(tuple(int(n) for n in re.findall('-?\d+', p)))
        vs.append(tuple(int(n) for n in re.findall('-?\d+', v)))

    posVels = {axis: set(range(-1000, 1001)) for axis in [0, 1, 2]}
    for i in range(len(vs) - 1):
        for j in range(i + 1, len(vs)):
            for ax in range(2):
                if vs[i][ax] != vs[j][ax]:
                    continue
                
                distDiff = abs(ps[i][ax] - ps[j][ax])
                for v in list(posVels[ax]):
                    if v == vs[i][ax] or distDiff % (v - vs[i][ax]) != 0:
                        posVels[ax].remove(v)

    throwPos = findCoordinates(ps, vs, *[list(posVels[i])[0] if len(posVels[i]) == 1 else None for i in sorted(posVels.keys())])

    if throwPos is not None:
        return sum(throwPos)

    return -1


def gaussJordan(augMatrix):
    try:
        for i in range(len(augMatrix[0]) - 1):
            multRow = 1 / augMatrix[i][i]
            for j in range(len(augMatrix[i])):
                augMatrix[i][j] *= multRow

            for j in range(len(augMatrix)):
                if j == i:
                    continue

                subAmt = -augMatrix[j][i]
                augMatrix[j] = list(addVectors(augMatrix[j], (c * subAmt for c in augMatrix[i])))
    except ZeroDivisionError:
        return None
    
    return augMatrix


def isintersect(a1x, a1y, b1x, b1y, a2x, a2y, b2x, b2y, d1x, d1y, d2x, d2y):    
    augMatrix = [[-d1y / d1x, 1, a1y - (d1y * a1x / d1x)], [-d2y / d2x, 1, a2y - (d2y * a2x / d2x)]]
    augMatrix = gaussJordan(augMatrix)
    if augMatrix is None:
        return False

    x = augMatrix[0][-1]
    y = augMatrix[1][-1]

    return min(a1x, b1x) <= x <= max(a1x, b1x) and min(a1y, b1y) <= y <= max(a1y, b1y) and min(a2x, b2x) <= x <= max(a2x, b2x) and min(a2y, b2y) <= y <= max(a2y, b2y)


def intersections(ps, vs, boundMin, boundMax):
    segments = []

    for p, v in zip(ps, vs):
        pChanged = True
        valid = True
        while pChanged and valid:
            pChanged = False
            for i in range(2):
                if p[i] < boundMin:
                    mult = (boundMin - p[i]) / v[i]
                    if mult < 0:
                        valid = False
                        break

                    p = tuple(c + (mult * o) for c, o in zip(p, v))
                    pChanged = True

                if p[i] > boundMax:
                    mult = (boundMax - p[i]) / v[i]
                    if mult < 0:
                        valid = False
                        break

                    p = tuple(c + (mult * o) for c, o in zip(p, v))
                    pChanged = True

        if valid:
            segStart = p
            mult = float('inf')
            for i in range(2):
                if v[i] < 0:
                    mult = min(mult, (boundMin - p[i]) / v[i])
                elif v[i] > 0:
                    mult = min(mult, (boundMax - p[i]) / v[i])

            segEnd = tuple(c + (mult * o) for c, o in zip(p, v))
            segments.append((segStart, segEnd, v))

    intersects = 0
    for i, seg1 in enumerate(segments[:-1]):
        (a1x, a1y), (b1x, b1y), (d1x, d1y) = seg1
        for seg2 in segments[i + 1:]:
            (a2x, a2y), (b2x, b2y), (d2x, d2y) = seg2
            intersects += isintersect(a1x, a1y, b1x, b1y, a2x, a2y, b2x, b2y, d1x, d1y, d2x, d2y)

    return intersects


def addVectors(v1, v2):
    return tuple(c1 + c2 for c1, c2 in zip(v1, v2))


def findCoordinates(positions, velocities, rockVX = None, rockVY = None, rockVZ = None):
    s = Solver()
    pX, pY, pZ = [], [], []
    t = []
    velX = Int('velX')
    velY = Int('velY')
    velZ = Int('velZ')
    posX = Int('posX')
    posY = Int('posY')
    posZ = Int('posZ')

    for i, ((px, py, pz), (vx, vy, vz)) in enumerate(zip(positions, velocities)):
        pX.append(Int(f'pX{i}'))
        pY.append(Int(f'pY{i}'))
        pZ.append(Int(f'pZ{i}'))
        t.append(Int(f't{i}'))
        s.add(pX[i] == px + (t[i] * vx), pY[i] == py + (t[i] * vy), pZ[i] == pz + (t[i] * vz), t[i] != 0)
        s.add(posX + t[i] * velX == pX[i], posY + t[i] * velY == pY[i], posZ + t[i] * velZ == pZ[i])

    if rockVX is not None:
        s.add(velX == rockVX)

    if rockVY is not None:
        s.add(velY == rockVY)

    if rockVZ is not None:
        s.add(velZ == rockVZ)

    # print(s.check())
    if s.check():
        m = s.model()

        return [int(str(m.evaluate(coord))) for coord in [posX, posY, posZ]]
    else:
        return None


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
        print(f"\nPart 1:\nNumber of 2D Intersections within Area: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of Coordinates of Rock Throw: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)