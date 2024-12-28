from collections import deque

DIRS = {'^': -1j, '>': 1, '<': -1, 'v': 1j}


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
            dir = DIRS[d]
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
                    x = 2*xx+yy*1j
                elif cell == 'O':
                    grid[2*xx+yy*1j] = 1  # left part of box, points to right
                    # right part of box, points to left
                    grid[2*xx+yy*1j+1] = -1
                elif cell == '#':
                    grid[2*xx+yy*1j] = 0
                    grid[2*xx+yy*1j+1] = 0

        q = deque()
        sub_boxes = {}

        for d in m.replace('\n', ''):
            dir = DIRS[d]
            if x+dir not in grid:
                x += dir
                continue

            q.append(x+dir)

            while q:
                p = q.popleft()
                if p in grid:
                    if grid[p] == 0:
                        q.clear()
                        sub_boxes.clear()
                        break
                    sub_boxes[p] = grid[p]
                    for b in [p+grid[p], p+dir]:
                        if b not in sub_boxes:
                            q.append(b)

            if sub_boxes:
                x += dir
                for b in sub_boxes.keys():
                    grid.pop(b)
                for b, v in sub_boxes.items():
                    grid[b+dir] = v
                sub_boxes.clear()

        return sum(int(b.real+100*b.imag) for b, cell in grid.items() if cell == 1)
