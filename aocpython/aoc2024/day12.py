from collections import deque

DIRS = [-1, -1j, 1, 1j]


def parse(input):
    data = input.splitlines()
    nx, ny = len(data), len(data[0])
    return {x+y*1j: data[y][x] for y in range(nx) for x in range(ny)}


def get_corners(x, region):
    return sum(int(x+dx1 not in region and x+dx2 not in region)
               + int(x+dx1+dx2 not in region and x+dx1 in region and x +
                     dx2 in region)
               for dx1, dx2 in zip(DIRS, DIRS[1:]+DIRS[:1]))


def floodfill(grid, x0, id, part1):
    q = [x0]
    region = {x0}
    per = 0

    while q:
        x = q.pop()
        for dx in DIRS:
            new_x = x+dx
            if grid.get(new_x) == id:
                del grid[new_x]
                region.add(new_x)
                q.append(new_x)
            elif part1:
                per += new_x not in region

    if part1:
        return len(region)*per
    corners = sum(get_corners(x, region)
                  for x in region)  # n corners = n edges
    return len(region)*corners


class Day12:
    def part1(input):
        grid = parse(input)
        ans = 0
        while grid:
            x0, id = grid.popitem()
            ans += floodfill(grid, x0, id, True)
        return ans

    def part2(input):
        grid = parse(input)
        ans = 0
        while grid:
            x0, id = grid.popitem()
            ans += floodfill(grid, x0, id, False)
        return ans
