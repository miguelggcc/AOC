from collections import deque
from functools import cache
from itertools import accumulate


@cache
def triangular(n):
    return n*(n-1)//2


def checksum(id, i, n):
    return id*(i*n + triangular(n))


class Day9:
    def part1(input):
        files = deque([(i, int(n))for i, n in enumerate(input[::2])])
        free = deque([*map(int, input[1::2])])
        out = 0
        i = 0
        reserve = files.pop()

        while files:
            id, n = files.popleft()
            out += checksum(id, i, n)
            i += n
            f = free.popleft()

            while f > 0 and files:
                if f >= reserve[1]:
                    out += checksum(reserve[0], i, reserve[1])
                    f -= reserve[1]
                    i += reserve[1]
                    reserve = files.pop()
                else:
                    out += checksum(reserve[0], i, f)
                    i += f
                    reserve = (reserve[0], reserve[1]-f)
                    f = 0

        if reserve:
            out += checksum(reserve[0], i, reserve[1])
        return out

    def part2(input):
        files = [(i, int(n))for i, n in enumerate(input[::2])]
        free = [*map(int, input[1::2])]
        i_s = [0]+list(accumulate(map(int, input)))
        out = 0

        for id, n in reversed(files):
            fit = next((i for i, x in enumerate(free[:id]) if x >= n), None)
            if fit is not None:
                out += checksum(id, i_s[fit*2+1], n)
                free[fit] -= n
                i_s[fit*2+1] += n
            else:
                out += checksum(id, i_s[id*2], n)

        return out
