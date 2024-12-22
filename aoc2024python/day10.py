from collections import deque


def parse(input):
    data = input.splitlines()
    nx, ny = len(data), len(data[0])
    return {x+y*1j: int(data[y][x]) for y in range(nx) for x in range(ny)}


def dfs(x0, part2, grid):
    count = 0
    q = deque([(x0, 0)])
    visited = set()
    while q:
        x, v = q.popleft()

        if part2 or x not in visited:
            visited.add(x)

            if v == 9:
                count += 1
                continue

            for dx in [1, 1j, -1, -1j]:
                if grid.get(x+dx, 0) == v+1:
                    q.append((x+dx, v+1))
    return count


class Day10:
    def part1(input):
        grid = parse(input)
        return sum(dfs(x0, False, grid) for x0, v0 in grid.items() if v0 == 0)

    def part2(input):
        grid = parse(input)
        return sum(dfs(x0, True, grid) for x0, v0 in grid.items() if v0 == 0)
