from heapq import heappop, heappush

DIRS = [1, 1j, -1, -1j]


def parse(input):
    data = input.strip().splitlines()
    maze = {}
    for y, row in enumerate(data):
        for x, cell in enumerate(row):
            if cell != '#':
                maze[x+y*1j] = [1e9 for _ in range(4)]
            if cell == 'S':
                start = x+y*1j
            elif cell == 'E':
                end = x+y*1j

    return maze, start, end


def to_coord(x):
    return x.real, x.imag


def dijkstra(maze, start, end, part2):
    out = 1e9
    h = []

    heappush(h, (0, *to_coord(start), 0))

    while h:
        s, re, im, dir = heappop(h)
        x = re+1j*im
        if x == end and s < out:
            out = s
            continue
        if s < maze[x][dir] or (part2 and s == maze[x][dir]):
            maze[x][dir] = s
            for new_dir, v in [(dir, 1), ((dir+1) % 4, 1001), ((dir-1) % 4, 1001)]:
                x_new = x+DIRS[new_dir]
                if x_new in maze:
                    heappush(h, (s+v, *to_coord(x_new), new_dir))

    return out


class Day16:
    def part1(input):
        maze, start, end = parse(input)
        return dijkstra(maze, start, end, False)

    def part2(input):
        maze, start, end = parse(input)
        min = dijkstra(maze, start, end, True)

        q = [(end, min, d) for d in range(4) if maze[end][d] == min]
        tiles = set()
        
        while q:
            x, s, dir = q.pop()
            tiles.add(x)
            if x == start:
                continue
            x_new = x-DIRS[dir]
            for new_dir, v in [(dir, 1), ((dir+1) % 4, 1001), ((dir-1) % 4, 1001)]:
                if maze[x_new][new_dir] == s-v:
                    q.append((x_new, s-v, new_dir))
        return len(tiles)
