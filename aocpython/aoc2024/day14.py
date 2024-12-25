from functools import reduce
from operator import mul
import numpy as np


def parse(input):
    return [
        [int(c) for c in r.translate(str.maketrans(
            {'=': ' ', ',': ' '})).split() if c.lstrip('-').isdigit()]
        for r in input.splitlines()
    ]


def draw(robots,nx,ny):
    board = [[False]*nx]*ny
    print(len(board))
    for r in robots:
        board[r[1]][r[0]] = True
    for y in range(ny):
        for x in range(nx):
            if board[y][x]:
                print('#', end='')
            else:
                print(',', end='')
        print('\n', end='')
 

class Day14:
    def part1(input):
        nx, ny = 11, 7
        t = 100
        quad = [0, 0, 0, 0]
        robots = parse(input)
        for r in robots:
            x = (r[0]+r[2]*t) % nx
            y = (r[1]+r[3]*t) % ny
            if x < nx//2 and y < ny//2:
                quad[0] += 1
            elif x > nx//2 and y < ny//2:
                quad[1] += 1
            elif x < nx//2 and y > ny//2:
                quad[2] += 1
            elif x > nx//2 and y > ny//2:
                quad[3] += 1
        print(quad)
        return reduce(mul, quad)

    def part2(input):
        nx, ny = 101, 103
        robots = parse(input)
        d = {}
        maxes = []
        for _ in range(801):
            for r in robots:
                r[0] = (r[0]+r[2]) % nx
                r[1] = (r[1]+r[3]) % ny
                d[r[0]] = d.setdefault(r[0],0)+1
            maxes.append(max(d.values())   )
            d.clear()
        draw(robots,nx,ny)
        maxes.index(max(maxes))+1
        return maxes.index(max(maxes))+1
