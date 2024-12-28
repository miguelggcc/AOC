from collections import deque


def parse(input):
    grid = {}
    data = input.strip().splitlines()
    for y, row in enumerate(data):
        for x, cell in enumerate(row):
            if cell == '@':
                x0 = x+y*1j
            elif cell != '.':
                grid[x+y*1j] = cell
    return grid, x0


dirs = {'^': -1j, '>': 1, '<': -1, 'v': 1j}


class Day15:
    def part1(input):
        g, m = input.split('\n\n')
        grid = {}
        data = g.strip().splitlines()
        for yy, row in enumerate(data):
            for xx, cell in enumerate(row):
                if cell == '@':
                    x = xx+yy*1j
                elif cell != '.':
                    grid[xx+yy*1j] = cell

        for d in m.replace('\n', ''):
            dir = dirs[d]
            p = x+dir
            while grid.get(p, None) == 'O':
                p += dir
            if p in grid:  # p is a '#' wall
                continue
            x += dir
            if x != p:
                grid[p] = 'O'
                del grid[x]

        return sum(int(b.real+100*b.imag) for b, cell in grid.items() if cell == 'O')

    def part2(input):
        g, m = input.split('\n\n')
        grid = {}
        data = g.strip().splitlines()
        for yy, row in enumerate(data):
            for xx, cell in enumerate(row):
                if cell == '@':
                    x0 = 2*xx+yy*1j
                elif cell == 'O':
                    grid[2*xx+yy*1j] = 1 #left part of box, points to right
                    grid[2*xx+yy*1j+1] = -1 #right part of box, points to left
                elif cell == '#':
                    grid[2*xx+yy*1j] = 0
                    grid[2*xx+yy*1j+1] = 0

        q = deque()
        branch = {}

        for d in m.replace('\n', ''):
            dir = dirs[d]
            if x0+dir not in grid:
                x0 += dir
                continue

            q.append(x0+dir)

            while q:
                x = q.popleft()
                if x in grid:
                    if grid[x] == 0:
                        q.clear()
                        branch.clear()
                        break
                    branch[x] = grid[x]
                    for b in [x+grid[x], x+dir]:
                        if b not in branch:
                            q.append(b)

            if branch:
                x0 += dir
                for b, v in branch.items():
                    grid.pop(b)
                for b, v in branch.items():
                    grid[b+dir] = v
            branch.clear()

        return sum(int(b.real+100*b.imag) for b, cell in grid.items() if cell == 1)
