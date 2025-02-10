from collections import deque


DIRS = [-1, -1j, 1, 1j]


def parse(input):
    return [int(x) + int(y) * 1j for x, y in (c.split(',') for c in input.splitlines())]


def bfs(x0, end, grid, n):
    q = deque([(x0, 0)])
    while q:
        x, steps = q.popleft()
        if x == end:
            return steps
        if x in grid:
            continue
        grid.add(x)
        for dx in DIRS:
            new_x = x+dx
            if 0 <= new_x.real <= n and 0 <= new_x.imag <= n and new_x not in grid:
                q.append((new_x, steps+1))


class Day18:
    def part1(input):
        n = 70
        total_bytes = 1024
        grid = set(parse(input)[:total_bytes+1])
        return bfs(0, n+n*1j, grid, n)

    def part2(input):
        n = 70
        bytes = parse(input)
        low = 1024
        high = len(bytes)
        mid = 0

        while low < high-1:
            mid = (high + low) // 2
            if bfs(0, n+n*1j, set(bytes[:mid]), n):
                low = mid
            else:
                high = mid

        return str(int(bytes[mid].real))+','+str(int(bytes[mid].imag))
